use std::{error::Error};
use sdl2::{event::Event};

pub mod app;

pub fn run() -> Result<(), Box<dyn Error>>
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("DIY Graphics Engine", 800, 600)
        .position_centered()
        .build()?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop 
    {
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit {..} => break 'running,
                _ => {},
            }
        }
    }
    
    return Ok(());
}