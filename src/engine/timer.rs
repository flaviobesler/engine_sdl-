use std::time::{Duration, Instant};

pub struct Timer{   
        last_frame: Instant,
        delta_timer: Duration

}

impl Timer {
        pub fn new() ->Self{
                Self {  last_frame: Instant::now(),
                        delta_timer: Duration::from_secs(0)}
                
        }

        pub fn update(&mut self){
                let now = Instant::now();

                self.delta_timer = now - self.last_frame;
                self.last_frame = now;
                
        }

}