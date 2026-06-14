use std::ffi::CString;

pub struct ComputeShader {
    id: u32,
}

impl ComputeShader
{
    pub fn new(src: &str) -> Self
    {
        unsafe {
            let shader = gl::CreateShader(gl::COMPUTE_SHADER);
            let c_src = CString::new(src).unwrap();
            gl::ShaderSource(shader, 1, &c_src.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success = 0i32;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0i32;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
                panic!("Compute shader compile error: {}", String::from_utf8_lossy(&buf));
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, shader);
            gl::LinkProgram(program);

            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0i32;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
                panic!("Compute program link error: {}", String::from_utf8_lossy(&buf));
            }

            gl::DeleteShader(shader);
            Self { id: program }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn dispatch(&self, x: u32, y: u32, z: u32) {
        unsafe { gl::DispatchCompute(x, y, z); }
    }

    fn loc(&self, name: &str) -> i32 {
        unsafe {
            let c = CString::new(name).unwrap();
            gl::GetUniformLocation(self.id, c.as_ptr())
        }
    }

    // pub fn set_int(&self, name: &str, v: i32) {
    //     unsafe { gl::Uniform1i(self.loc(name), v); }
    // }
    // pub fn set_float(&self, name: &str, v: f32) {
    //     unsafe { gl::Uniform1f(self.loc(name), v); }
    // }
    pub fn set_vec2(&self, name: &str, x: f32, y: f32) {
        unsafe { gl::Uniform2f(self.loc(name), x, y); }
    }
    // pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
    //     unsafe { gl::Uniform3f(self.loc(name), x, y, z); }
    // }
}

impl Drop for ComputeShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}