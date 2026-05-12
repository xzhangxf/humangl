use std::ffi::CString;

pub fn compile_program(vertex_src: &str, fragment_src: &str) -> gl::types::GLuint {
    let vs = compile_shader(gl::VERTEX_SHADER, vertex_src);
    let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_src);

    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut ok: gl::types::GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut ok);

        if ok == 0 {
            let log = get_program_info_log(program);
            gl::DeleteProgram(program);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            panic!("Failed to link program: {}", log);
        }
        gl::DetachShader(program, vs);
        gl::DetachShader(program, fs);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        program
    }
}

fn compile_shader(kind: gl::types::GLenum, src: &str) -> gl::types::GLuint {
    let c_src = CString::new(src).expect("Shader source contains a NULL byte");

    unsafe {
        let shader = gl::CreateShader(kind);
        gl::ShaderSource(shader, 1, &c_src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check compile status
        let mut ok: gl::types::GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok);

        if ok == 0 {
            let log = get_shader_info_log(shader);
            gl::DeleteShader(shader);
            panic!("Shader compile failed:\n{}", log);
        }

        shader
    }
}

fn get_shader_info_log(shader: gl::types::GLuint) -> String {
    unsafe {
        let mut len: gl::types::GLint = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        if len <= 1 {
            return "(no shader log)".to_string();
        }

        let mut buf = vec![0u8; len as usize];
        gl::GetShaderInfoLog(
            shader,
            len,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut gl::types::GLchar,
        );

        String::from_utf8_lossy(&buf).trim().to_string()
    }
}

fn get_program_info_log(program: gl::types::GLuint) -> String {
    unsafe {
        let mut len: gl::types::GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

        if len <= 1 {
            return "(no programme log)".to_string();
        }

        let mut buf = vec![0u8; len as usize];
        gl::GetProgramInfoLog(
            program,
            len,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut gl::types::GLchar,
        );

        String::from_utf8_lossy(&buf).trim().to_string()
    }
}