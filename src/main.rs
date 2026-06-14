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
use std::{/*fs::OpenOptions,*/ num::NonZeroU32};
use std::time::Instant;
use winit::raw_window_handle::HasWindowHandle;
// use nalgebra_glm as glm;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

mod shaders;
use shaders::{RT_COMPUTE_SRC, BLIT_FRAG_SRC, BLIT_VERT_SRC};

mod geom_shader;
use geom_shader::GeometryShader;

mod comp_shader;
use comp_shader::ComputeShader;

mod image_2d;
use image_2d::Image2D;

mod camera;
use camera::Camera;

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
    rt_compute: Option<ComputeShader>,
    blit_shader: Option<GeometryShader>,
    output_tex: Option<Image2D>,

    camera: Option<Camera>,
    mouse_down: bool,
    shift_down: bool,
    last_cursor: Option<(f32, f32)>,
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

            rt_compute: None,
            blit_shader: None,
            output_tex: None,
            vao: 0,

            camera: None,
            mouse_down: false,
            shift_down: false,
            last_cursor: None,
        }
    }

    fn setup(&mut self, width: u32, height: u32) {
        unsafe {
            self.rt_compute = Some(ComputeShader::new(RT_COMPUTE_SRC));
            self.blit_shader = Some(GeometryShader::new(BLIT_VERT_SRC, BLIT_FRAG_SRC));
            self.output_tex = Some(Image2D::new(width, height, gl::RGBA32F));
            gl::GenVertexArrays(1, &mut self.vao);

            self.camera = Some(Camera::new());
        }
        println!("Setup\n");
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        self.rt_compute = None;
        self.blit_shader = None;
        self.output_tex = None;
        self.camera = None;
        println!("Shutdown\n");
    }

    fn update(&mut self, _dt: f32) {
    }

    fn render(&mut self) {
        unsafe {            
            if let (Some(rt), Some(img), Some(cam)) = (&self.rt_compute, &self.output_tex, &self.camera) {
                rt.bind();
                rt.set_vec2("uResolution", self.width as f32, self.height as f32);
                
                let pos = cam.position();
                let (fwd, right, up) = cam.basis();
                rt.set_vec3("uCamPos", pos.x, pos.y, pos.z);
                rt.set_vec3("uCamForward", fwd.x, fwd.y, fwd.z);
                rt.set_vec3("uCamRight", right.x, right.y, right.z);
                rt.set_vec3("uCamUp", up.x, up.y, up.z);
                rt.set_float("uTanHalfFov", cam.tan_half_fov());

                img.bind_storage(0, gl::WRITE_ONLY);
                let gx = (self.width + 7) / 8;
                let gy = (self.height + 7) / 8;
                rt.dispatch(gx, gy, 1);
                gl::MemoryBarrier(gl::TEXTURE_FETCH_BARRIER_BIT);
            }

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            if let Some(blit) = &self.blit_shader {
                blit.bind();
                if let Some(img) = &self.output_tex {
                    img.bind_sampled(0);
                }
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        if let Some(gl) = &self.gl {
            gl.resize(width, height);
            unsafe { gl::Viewport(0, 0, width as GLsizei, height as GLsizei); }
        }
        if let Some(img) = &mut self.output_tex {
            img.resize(width, height);
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
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    self.mouse_down = state == ElementState::Pressed;
                }
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.shift_down = mods.state().shift_key();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                if self.mouse_down {
                    if let Some((lx, ly)) = self.last_cursor {
                        let dx = x - lx;
                        let dy = y - ly;
                        if let Some(cam) = &mut self.camera {
                            if self.shift_down {
                                cam.zoom(dy);
                            } else {
                                cam.orbit(dx, dy);
                            }
                        }
                    }
                }
                self.last_cursor = Some((x, y)); // always track, so the first drag frame doesn't jump
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
