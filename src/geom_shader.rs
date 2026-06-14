use std::ffi::CString;

pub struct GeometryShader {
    id: u32,
}

impl GeometryShader {

    pub fn new(vert_src: &str, frag_src: &str) -> Self {
        unsafe {
            let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let src = CString::new(vert_src).unwrap();
            gl::ShaderSource(vert_shader, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(vert_shader);

            let mut success = 0i32;
            gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0i32;
                gl::GetShaderiv(vert_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetShaderInfoLog(vert_shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
                let msg = String::from_utf8_lossy(&buf);
                panic!("Compute vert_shader compile error: {}", msg);
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
                let msg = String::from_utf8_lossy(&buf);
                panic!("Compute frag_shader compile error: {}", msg);
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
                let msg = String::from_utf8_lossy(&buf);
                panic!("Compute geom_shader link error: {}", msg);
            }

            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);

            Self { id: program }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let name = CString::new(name).unwrap();
            let loc = gl::GetUniformLocation(self.id, name.as_ptr());
            gl::Uniform1i(loc, value);
        }
    }
}

impl Drop for GeometryShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}