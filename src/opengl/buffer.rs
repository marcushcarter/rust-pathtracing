use gl::types::*;

// UNIFORM BUFFER

pub struct UniformBuffer {
    id: u32,
    size: usize,
}

impl UniformBuffer
{
    pub fn new(size: usize, binding: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::UNIFORM_BUFFER, id);
            gl::BufferData(gl::UNIFORM_BUFFER, size as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, id);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
        Self { id, size }
    }

    pub fn update<T>(&self, data: &T) {
        debug_assert!(std::mem::size_of::<T>() <= self.size, "UBO upload larger than buffer");
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, std::mem::size_of::<T>() as isize, data as *const T as *const std::ffi::c_void,);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

// STORAGE BUFFER

pub struct StorageBuffer {
    id: GLuint,
}

impl StorageBuffer
{
    pub fn from_slice<T>(data: &[T], binding: u32) -> Self {
        let mut id = 0;
        let bytes = std::mem::size_of_val(data);
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, id);
            gl::BufferData(gl::SHADER_STORAGE_BUFFER, bytes as isize, data.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW,);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, id);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
        Self { id }
    }
}

impl Drop for StorageBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}