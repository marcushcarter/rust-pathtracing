use gl::types::*;

pub struct Image2D {
    id: u32,
    width: u32,
    height: u32,
    format: GLenum,
}

impl Image2D
{
    pub fn new(width: u32, height: u32, format: GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, format, width as i32, height as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self { id, width, height, format }
    }

    pub fn load_rgba_f32(width: u32, height: u32, data: &[f32]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA32F, width as i32, height as i32);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, width as i32, height as i32, gl::RGBA, gl::FLOAT, data.as_ptr() as *const std::ffi::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self { id, width, height, format: gl::RGBA32F }
    }

    pub fn bind_sampled(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn bind_storage(&self, unit: u32, access: GLenum) {
        unsafe {
            gl::BindImageTexture(unit, self.id, 0, gl::FALSE, 0, access, self.format);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == self.width && height == self.height {
            return;
        }
        *self = Image2D::new(width, height, self.format);
    }
}

impl Drop for Image2D {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}