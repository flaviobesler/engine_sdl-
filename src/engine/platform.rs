
pub struct Platform{
        pub sdl: sdl3::Sdl,
        pub video: sdl3::VideoSubsystem,
        pub event_pump: sdl3::EventPump
}
impl Platform{

        pub fn new() -> Result< Self, Box<dyn std::error::Error>>{
                let sdl = sdl3::init()?;
                let video = sdl.video()?;
                let event_pump = sdl.event_pump()?;

                Ok(Self{
                        sdl, video, event_pump
                })
        
        }
        pub fn event_pump(&mut self)-> &mut sdl3::EventPump{
                &mut self.event_pump
        
        }



}