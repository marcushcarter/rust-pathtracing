use std::ffi::CString;

// GEOMETRY SHADER

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
}

impl Drop for GeometryShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

// COMPUTE SHADER

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
}

impl Drop for ComputeShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}