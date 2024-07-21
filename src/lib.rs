use std::error::Error;

use winit::{event, event_loop};

pub mod app;

pub fn run() -> Result<(), Box<dyn Error>>
{
    app::EngineHandler::start()?;

    return Ok(());
}