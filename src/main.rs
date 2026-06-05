#![windows_subsystem = "console"]

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
use winit::raw_window_handle::HasWindowHandle;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

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
}

impl App {
    fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            window: None,
            gl: None,
        }
    }

    fn setup(&mut self) {
        println!("Setup\n");
    }

    fn shutdown(&mut self) {
        println!("Shutdown\n");
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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

        self.setup();
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
                self.update();
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
    let mut app = App::new("Game", 1280, 720);
    event_loop.run_app(&mut app).unwrap();
}
