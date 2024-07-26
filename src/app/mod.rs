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

    let mut buffer = 1;
    let mut vao = 1;
    let mut ebo = 1;

    unsafe {
        gl::Viewport(0, 0, APP_WIDTH, APP_HEIGHT);

        let vertices: Vec<f32> = vec!
        [
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
            -0.5, -0.5, 0.0,
        ];
        let indices: Vec<u32> = vec!
        [
            0, 1, 2,
            1, 2, 3,
        ];

        // Generates objects
        gl::GenBuffers(1, &mut buffer);
        gl::GenBuffers(1, &mut ebo);
        gl::GenVertexArrays(1, &mut vao);

        // Binds VAO
        gl::BindVertexArray(vao);

        // Binds VBO to VAO
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

        // Loads data into current VBO
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
            vertices.as_ptr() as *const gl::types::GLvoid, 
            gl::STATIC_DRAW
        );

        // Binds and loads EBO
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            (std::mem::size_of::<u32>() * indices.len()) as gl::types::GLsizeiptr, 
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // Declares how the current VBO should be read
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT, 
            gl::FALSE, 
            (std::mem::size_of::<f32>() * 3) as i32, 
            0 as *const gl::types::GLvoid
        );
        gl::EnableVertexAttribArray(0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {},
            }
        }

        program.activate();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES, 
                6, 
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }

        window.swap_buffers();
    }
    
    return Ok(());
}