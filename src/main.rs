// IF YOU ARE READING THIS RUST MAKES ME GENUINELY WANT TO THROW MY COMPUTER OFF A CLIFF

use gl::types::*;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use std::{num::NonZeroU32};
use winit::raw_window_handle::HasWindowHandle;
use nalgebra_glm as glm;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use std::path::{Path, PathBuf};

mod opengl;
mod scene;
use opengl::{ComputeShader, GeometryShader, Image2D, StorageBuffer, UniformBuffer};
use opengl::shaders::{BLIT_FRAG_SRC, BLIT_VERT_SRC, RT_COMPUTE_SRC};
use scene::{Camera, CameraData, Sphere, Triangle};

const RESOURCE_ROOT: &str = "res";

fn resource_path(name: &str) -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join(RESOURCE_ROOT).join(name);
            if p.exists() { return p; }
        }
    }
    let dev = Path::new(env!("CARGO_MANIFEST_DIR")).join(RESOURCE_ROOT).join(name);
    if dev.exists() { return dev; }
    Path::new(RESOURCE_ROOT).join(name)
}

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
    frame: u32,
    
    camera: Option<Camera>,
    camera_dirty: bool,
    sample_count: u32,
    mouse_down: bool,
    shift_down: bool,
    last_cursor: Option<(f32, f32)>,
    
    rt_compute: Option<ComputeShader>,
    blit_shader: Option<GeometryShader>,
    
    output_tex: Option<Image2D>,
    env_tex: Option<Image2D>,
    
    vao: GLuint,
    camera_ubo: Option<UniformBuffer>,
    sphere_ssbo: Option<StorageBuffer>,
    triangle_ssbo: Option<StorageBuffer>,
}

impl App {
    fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            window: None,
            gl: None,
            frame: 0,
            
            camera: None,
            camera_dirty: false,
            sample_count: 0,
            mouse_down: false,
            shift_down: false,
            last_cursor: None,

            rt_compute: None,
            blit_shader: None,

            output_tex: None,
            env_tex: None,
            
            vao: 0,
            camera_ubo: None,
            sphere_ssbo: None,
            triangle_ssbo: None,
        }
    }

    fn setup(&mut self, width: u32, height: u32) {
        unsafe {
            self.rt_compute = Some(ComputeShader::new(RT_COMPUTE_SRC));
            self.blit_shader = Some(GeometryShader::new(BLIT_VERT_SRC, BLIT_FRAG_SRC));
            
            self.output_tex = Some(Image2D::new(width, height, gl::RGBA32F));
            
            let hdr = image::open(resource_path("kloofendal_48d_partly_cloudy_puresky_2k.hdr")).expect("failed to load HDR skybox").to_rgba32f();
            let (hw, hh) = hdr.dimensions();
            self.env_tex = Some(Image2D::load_rgba_f32(hw, hh, &hdr.into_raw()));
            
            gl::GenVertexArrays(1, &mut self.vao);
            
            self.camera_ubo = Some(UniformBuffer::new(std::mem::size_of::<CameraData>(), 0));
            
            let spheres = vec![
                Sphere::glass(glm::vec3(-2.25, 0.0, 0.0), 0.5, 1.5),
                Sphere::diffuse(glm::vec3(-0.75, 0.0, 0.0), 0.5, glm::vec3(1.0, 1.0, 1.0)),
                Sphere::metal(glm::vec3(0.75, 0.0, 0.0), 0.5, glm::vec3(1.0, 1.0, 1.0), 0.0),
                Sphere::metal(glm::vec3(2.25, 0.0, 0.0), 0.5, glm::vec3(0.5, 0.5, 1.0), 0.3),
                ];
            self.sphere_ssbo = Some(StorageBuffer::from_slice(&spheres, 0));

            let y = -0.5;
            let h = 5.0;
            let c0 = glm::vec3(-h, y, -h);
            let c1 = glm::vec3( h, y, -h);
            let c2 = glm::vec3( h, y,  h);
            let c3 = glm::vec3(-h, y,  h);
            let ground = glm::vec3(0.7, 0.7, 0.7);
            let tris = vec![
                Triangle::diffuse(c0, c1, c2, ground),
                Triangle::diffuse(c0, c2, c3, ground),
            ];
            self.triangle_ssbo = Some(StorageBuffer::from_slice(&tris, 1));
                
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
        self.env_tex = None;
        self.camera_ubo = None;
        self.sphere_ssbo = None;
        self.triangle_ssbo = None;
        self.camera = None;
        println!("Shutdown\n");
    }

    fn render(&mut self) {
        if self.camera_dirty {
            self.sample_count = 0;
            self.camera_dirty = false;
        }

        unsafe {            
            if let (Some(rt), Some(img), Some(cam), Some(ubo)) = (&self.rt_compute, &self.output_tex, &self.camera, &self.camera_ubo) {
                
                let camera_data = cam.to_data(self.width, self.height, self.frame, self.sample_count);
                ubo.update(&camera_data);

                rt.bind();
                img.bind_storage(0, gl::READ_WRITE);
                if let Some(env) = &self.env_tex { env.bind_sampled(1); }
                let gx = (self.width + 7) / 8;
                let gy = (self.height + 7) / 8;
                if self.sample_count < 1000 { rt.dispatch(gx, gy, 1); }
                gl::MemoryBarrier(gl::TEXTURE_FETCH_BARRIER_BIT | gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            }

            self.frame = self.frame.wrapping_add(1);
            self.sample_count = self.sample_count.wrapping_add(1);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            if let Some(blit) = &self.blit_shader {
                blit.bind();
                if let Some(img) = &self.output_tex { img.bind_sampled(0); }
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.sample_count = 0;
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
                self.render();
                if let Some(gl) = &self.gl { gl.swap_buffers(); }
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
                            if self.shift_down { cam.zoom(dy); } else { cam.orbit(dx, dy); }
                        }
                        self.camera_dirty = true;
                    }
                }
                self.last_cursor = Some((x, y));
            }
            _ => {}
        }
    }
}

fn main() {
    println!("Hello, World!");
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new("Path Tracing Demo", 1024, 640);
    event_loop.run_app(&mut app).unwrap();
}
