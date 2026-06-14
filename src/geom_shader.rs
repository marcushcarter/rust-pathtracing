use std::ffi::CString;

pub struct GeometryShader {
    id: u32,
}

impl GeometryShader
{
    pub fn new(vert_src: &str, frag_src: &str) -> Self
    {
        unsafe {
            let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let src: CString = CString::new(vert_src).unwrap();
            gl::ShaderSource(vert_shader, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(vert_shader);

            let mut success = 0i32;
            gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0i32;
                gl::GetShaderiv(vert_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetShaderInfoLog(vert_shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
                panic!("Vertex shader compile error: {}", String::from_utf8_lossy(&buf));
            }
            
            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let src = CString::new(frag_src).unwrap();
            gl::ShaderSource(frag_shader, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(frag_shader);

            gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0i32;
                gl::GetShaderiv(frag_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetShaderInfoLog(frag_shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
                panic!("Fragment shader compile error: {}", String::from_utf8_lossy(&buf));
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, vert_shader);
            gl::AttachShader(program, frag_shader);
            gl::LinkProgram(program);

            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0i32;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
                panic!("Geometry program link error: {}", String::from_utf8_lossy(&buf));
            }

            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);
            Self { id: program }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    fn loc(&self, name: &str) -> i32 {
        unsafe {
            let c = CString::new(name).unwrap();
            gl::GetUniformLocation(self.id, c.as_ptr())
        }
    }

    pub fn set_int(&self, name: &str, v: i32) {
        unsafe { gl::Uniform1i(self.loc(name), v); }
    }
    pub fn set_float(&self, name: &str, v: f32) {
        unsafe { gl::Uniform1f(self.loc(name), v); }
    }
    pub fn set_vec2(&self, name: &str, x: f32, y: f32) {
        unsafe { gl::Uniform2f(self.loc(name), x, y); }
    }
    pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        unsafe { gl::Uniform3f(self.loc(name), x, y, z); }
    }
}

impl Drop for GeometryShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}