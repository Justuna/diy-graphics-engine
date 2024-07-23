use std::{error::Error};
use glfw::{Action, Context, Key};
use gl;
pub mod app;

pub fn run() -> Result<(), Box<dyn Error>>
{
    let mut glfw = glfw::init_no_callbacks()?;
    let (mut window, events) = glfw
        .create_window(
            600, 
            900, 
            "DIY Graphics Engine", 
            glfw::WindowMode::Windowed
        ).ok_or(glfw::Error::NoWindowContext)?;

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
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