#![windows_subsystem = "console"]

// IF YOU ARE READING THIS RUST MAKES ME WANT TO THROW MY COMPUTER OFF A CLIFF

use gl::types::*;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use std::num::NonZeroU32;
use std::time::Instant;
use winit::raw_window_handle::HasWindowHandle;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

mod shaders;
use shaders::{RTX_VERT_SRC, RTX_FRAG_SRC};

mod geom_shader;
use geom_shader::GeometryShader;

struct GlContext {
    surface: glutin::surface::Surface<WindowSurface>,
    context: glutin::context::PossiblyCurrentContext,
}

impl GlContext {
    fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }

    fn resize(&self, width: u32, height: u32) {
        self.surface.resize(
            &self.context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
    }
}

struct App {
    title: String,
    width: u32,
    height: u32,
    window: Option<Window>,
    gl: Option<GlContext>,
    last_frame: Instant,
    
    vao: GLuint,
    vbo: GLuint,

    rtx_shader: Option<GeometryShader>,
}

impl App {
    fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            window: None,
            gl: None,
            last_frame: Instant::now(),

            vao: 0,
            vbo: 0,
            rtx_shader: None,
        }
    }

    fn setup(&mut self, width: u32, height: u32) {
 
        unsafe {
            self.rtx_shader = Some(GeometryShader::new(RTX_VERT_SRC, RTX_FRAG_SRC));
            
            let vertices: [f32; 9] = [
                -1.0,  3.0, 0.0,
                -1.0, -1.0, 0.0,
                3.0, -1.0, 0.0,
            ];

            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

            let stride = (3 * std::mem::size_of::<f32>()) as GLsizei;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as *const _);
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }
        println!("Setup\n");
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
        self.rtx_shader = None;
        println!("Shutdown\n");
    }

    fn update(&mut self, _dt: f32) {
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if let Some(shader) = &self.rtx_shader {
                shader.bind();
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        if let Some(gl) = &self.gl {
            gl.resize(width, height);
            unsafe { gl::Viewport(0, 0, width as GLsizei, height as GLsizei); }
        }
    }
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attrs));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| { 
                configs.reduce(|a, b| if a.num_samples() > b.num_samples() { a } else { b }).unwrap()
            })
            .unwrap();

        let window = window.unwrap();
        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let gl_display = gl_config.display();

        let context_attrs = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version::new(4, 6))))
            .build(Some(raw_window_handle));

        let not_current_ctx = unsafe {
            gl_display.create_context(&gl_config, &context_attrs).unwrap()
        };

        let (width, height): (u32, u32) = window.inner_size().into();
        let surface_attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            gl_display.create_window_surface(&gl_config, &surface_attrs).unwrap()
        };

        let context = not_current_ctx.make_current(&surface).unwrap();

        gl::load_with(|s| {
            let s = std::ffi::CString::new(s).unwrap();
            gl_display.get_proc_address(&s)
        });

        self.gl = Some(GlContext { surface, context });
        self.window = Some(window);
        self.last_frame = Instant::now();

        self.setup(self.width, self.height);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.shutdown();
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    self.on_resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;
                
                let fps = (1.0 / dt) as u32;
                let title = format!("{} | {:.4}ms | {}fps", self.title, dt * 1000.0, fps);
                self.window.as_ref().unwrap().set_title(&title);

                self.update(dt);
                self.render();

                if let Some(gl) = &self.gl {
                    gl.swap_buffers();
                }

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    println!("Hello, World!");
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new("Game", 800, 600);
    event_loop.run_app(&mut app).unwrap();
}
