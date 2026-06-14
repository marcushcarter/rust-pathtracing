use gl::types::*;

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