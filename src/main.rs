use gl::{types::{GLuint, GLsizeiptr, GLvoid, GLint}, ARRAY_BUFFER, STATIC_DRAW, FLOAT, FALSE, TRIANGLES};

extern crate sdl2;
extern crate gl;

mod render_gl;


fn main() {
    println!("Hello, world!");

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);


    let window = video_subsystem
        .window("Hi", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();   
    
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    
    
    use std::ffi::CString;
    
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();
    
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();
    
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up vertex buffer object

    let vertices: Vec<f32> = vec![
        // positions        // colours
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,  // bottom right
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0,  // bottom left
        0.0, 0.5, 0.0,      0.0, 0.0, 1.0   // top
        ];
        
    let mut vbo: GLuint = 0;
    
    unsafe { 
        gl::GenBuffers(1, &mut vbo);  
        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr, 
            vertices.as_ptr() as *const GLvoid, 
            STATIC_DRAW
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }


    // set up vertex array object
   
    let mut vao: GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao); 
        gl::BindVertexArray(vao);
        gl::BindBuffer(ARRAY_BUFFER, vbo);
       
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
        (6 * std::mem::size_of::<f32>()) as GLint,   // now that the colour information has been added the stride is 6
        std::ptr::null()
        );


        gl::EnableVertexAttribArray(1); // index 1 is the colour information
        gl::VertexAttribPointer(
            1, 
            3, 
            FLOAT, 
            FALSE, 
        (6 * std::mem::size_of::<f32>()) as GLint, 
        (3 * std::mem::size_of::<f32>()) as *const GLvoid
    );
    
        gl::BindBuffer(ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
            }


        unsafe {
            gl::Viewport(0, 0, 900, 700); 
            gl::ClearColor(0.3, 0.3, 0.5, 1.0); 
        }
        
        
        let mut event_pump = sdl.event_pump().unwrap();
        
        'main: loop {
            for event in event_pump.poll_iter(){
                //Handle user input here
                match event {
                    sdl2::event::Event::Quit { timestamp } => {print!("Program ran for {} ms", timestamp); break 'main},
                    _ => {},
                }     
                
            }
            
            shader_program.set_used();
            unsafe{ gl::Clear(gl::COLOR_BUFFER_BIT); }
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(
                    TRIANGLES,
                    0,
                    3
                );
            }

            //Render window contents here

            window.gl_swap_window();

        }

}

