
mod engine;
use engine::window::Window;
use engine::platform::Platform;
use sdl3::event::Event;



fn main() -> Result<(), Box<dyn std::error::Error>> {

        let mut platform = Platform::new()?;

        let _window = Window::new(
                String::from("teste"),
                600, 
                800,
                &platform.video
        
        )?;
        let event_pump = platform.event_pump();

        'running: loop{

                for event in event_pump.poll_iter(){
                        match event {
                                Event::Quit {..} => break 'running,
                                _ => {}
                            
                        }
                }
        
        }

        

        Ok(())
}