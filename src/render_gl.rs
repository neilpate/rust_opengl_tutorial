
use gl;
use gl::{types::*, COMPILE_STATUS, INFO_LOG_LENGTH, VERTEX_SHADER, FRAGMENT_SHADER};
use std;
use std::ffi::{CString, CStr};

pub struct Program {
    id: GLuint,
}

impl Program {

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram()        };
    
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id());  }
        }
        
        unsafe { gl::LinkProgram(program_id); }

        let mut success: GLint = 1;

        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);  }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len) }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe { gl::GetProgramInfoLog(
                program_id, 
                len, 
                std::ptr::null_mut(), 
                error.as_ptr() as *mut GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());

    }

        
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id());  }

        }


        Ok(Program { id: program_id})
    
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}


pub struct Shader {
    id: GLuint,
}

impl  Shader {
    fn from_source (source : &CStr,
                    kind : GLenum)
                    -> Result<Shader, String> {
                        let id = shader_from_source(source, kind)?;
                        Ok(Shader {id})
                    }

    pub fn from_vert_source(source : &CStr) -> Result<Shader, String> {
        Shader::from_source(source, VERTEX_SHADER)
    }

    pub fn from_frag_source(source : &CStr) -> Result<Shader, String> {
        Shader::from_source(source, FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id);  }
    }
}

fn shader_from_source(source: &CStr, kind : GLuint) -> Result<gl::types::GLuint, String>  {
    let id = unsafe { gl::CreateShader(kind)};

    unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
    }

    let mut success: GLint = 1;
    
    unsafe { gl::GetShaderiv(id, COMPILE_STATUS, &mut success);  }
    
    if success == 0 {
        let mut len: GLint = 0;
        unsafe { gl::GetShaderiv(id, INFO_LOG_LENGTH, &mut len);  }
        
        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe { gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);  }
            
        return Err(error.to_string_lossy().into_owned())
        }
        
    Ok(id)
    }
        

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    
    // convert buffer to a CString
    unsafe { CString::from_vec_unchecked(buffer)   }

}

