use std::{error::Error};
use glfw::{Action, Context, Key};
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

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {},
            }
        }
    }
    
    return Ok(());
}