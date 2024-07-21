use std::error::Error;

use winit::
{
    application::ApplicationHandler, 
    event, 
    event_loop::
    {
        self, ActiveEventLoop
    }, 
    window::
    {
        self, 
        Window
    },
};
pub struct EngineHandler 
{
    window: Option<Window>,
}

impl ApplicationHandler for EngineHandler 
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) 
    {
        println!("Resumed...");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: window::WindowId,
        event: event::WindowEvent,
    ) {
        match event {
            event::WindowEvent::CloseRequested => 
            {
                println!("Exiting...");
                event_loop.exit();
            }
            event::WindowEvent::RedrawRequested => 
            {
                match &self.window 
                {
                    Some(_window) if _window.id() == window_id => 
                    {
                        println!("Redraw!");
                    }
                    Some(_window) => 
                    {
                        println!("Event from different window, ignoring...");
                    }
                    None => 
                    {
                        println!("Window disappeared??? Exiting...");
                        event_loop.exit();
                    }
                }
            }
            _event => ()
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: event::StartCause) {
        match cause {
            event::StartCause::Init => 
            {
                println!("Started application!");

                let config = Window::default_attributes()
                    .with_transparent(true)
                    .with_title("DIY Graphics Engine");
                
                match event_loop.create_window(config) 
                {
                    Ok(_window) => self.window = Some(_window),
                    Err(_error) => {
                        println!("Error occured: {:#?}", _error);
                        event_loop.exit();
                    }
                }
            }
            _cause => ()
        }
    }
}

impl EngineHandler 
{
    pub fn start() -> Result<Self, Box<dyn Error>> 
    {
        let mut engine = EngineHandler{ window: None };
        
        let event_loop = event_loop::EventLoop::new()?;

        event_loop.run_app(&mut engine)?;

        return Ok(engine);
    }
}