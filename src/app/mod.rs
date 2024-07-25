use std::{fs::File, io::Read};

use glfw::Context;
use gl;

use crate::core::shaders::types::{FragmentShaderType, Shader, ShaderProgram, VertexShaderType};

const APP_WIDTH: i32 = 900;
const APP_HEIGHT: i32 = 700;

pub fn run() -> anyhow::Result<()>
{
    let mut glfw = glfw::init_no_callbacks()?;
    let (mut window, events) = glfw
        .create_window(
            APP_WIDTH as u32, 
            APP_HEIGHT as u32, 
            "DIY Graphics Engine", 
            glfw::WindowMode::Windowed
        ).ok_or(glfw::Error::NoWindowContext)?;

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    let mut vertex_source = String::new();
    File::open("assets/shaders/vertex/test.vert")?.read_to_string(&mut vertex_source)?;

    let mut fragment_source = String::new();
    File::open("assets/shaders/fragment/test.frag")?.read_to_string(&mut fragment_source)?;

    let vertex_shader: Shader<VertexShaderType> = Shader::load(&vertex_source)?;
    let fragment_shader: Shader<FragmentShaderType> = Shader::load(&fragment_source)?;
    let program = ShaderProgram::load(vertex_shader, fragment_shader)?;

    program.activate();

    unsafe {
        gl::Viewport(0, 0, APP_WIDTH, APP_HEIGHT);

        // let mut buffer = 1;

        // let vertices: Vec<f32> = vec!
        // [
        //     -0.5, -0.5, 0.0,
        //     0.5, -0.5, 0.0,
        //     0.0, 0.5, 0.0,
        // ];

        // gl::GenBuffers(1, &mut buffer);
        // gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        // gl::BufferData(
        //     gl::ARRAY_BUFFER, 
        //     (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
        //     vertices.as_ptr() as *const gl::types::GLvoid, 
        //     gl::STATIC_DRAW
        // );
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
    
    return Ok(());
}