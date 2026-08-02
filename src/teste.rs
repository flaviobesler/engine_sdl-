use sdl::event::Event;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let sdl = sdl3::init()?;

    let video = sdl.video()?;

    let window = video
    .window("car", 800, 600)
    .position_centered()
    .build()?;

    let mut events = sdl.event_pump()?;
    let mut last_frame = Instant::now();

       'running: loop {
        let now = Instant::now();
        let delta_time = now - last_frame;
        last_frame = now;

        let delta_seconds = delta_time.as_secs_f32();

        print!("delta: {}", delta_seconds);



        for event in events.poll_iter(){
            match event {
            Event::Quit { .. } => break 'running,
            _ => {}
            }
        }
    }

    drop(window);

    Ok(())
}