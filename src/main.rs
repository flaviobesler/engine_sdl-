
mod engine;
use engine::window::Window;
use engine::platform::Platform;
use engine::timer::Timer;
use engine::input::Input;

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
        

        let mut timer = Timer::new();

        let mut _input = Input::new();



        'running: loop{
                
                timer.update();

                for event in event_pump.poll_iter(){
                        _input.input(&event);
                        match event {
                                Event::Quit {..} => break 'running,
                                _ => {}
                        
                        }
                        
                }
        
        }

        

        Ok(())
}