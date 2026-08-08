use sdl3::event::Event;
use sdl3::keyboard::Keycode;
pub struct Input{
        teclado: Vec<Keycode>
        
}

impl Input{
        pub fn new() -> Self{
               Self { teclado: Vec::new() }
        }

        pub fn input(&mut self, event: &Event){

                


                match event {

                        Event::KeyDown { keycode, .. } =>
                        {       
                                //key a
                                if *keycode == Some(Keycode::A){
                                        if!self.teclado.contains(&Keycode::A){
                                                self.teclado.push(Keycode::A);}
                                }

                                //key w
                                if let Some(Keycode::W) = keycode{       
                                        if !self.teclado.contains(&Keycode::W){
                                                self.teclado.push(Keycode::W);}
                                }
                                //key d
                                if *keycode ==Some(Keycode::D){
                                        if !self.teclado.contains(&Keycode::D){
                                                self.teclado.push(Keycode::D);}
                                }
                                //key s
                                if *keycode ==Some(Keycode::S){
                                        if !self.teclado.contains(&Keycode::S){
                                                self.teclado.push(Keycode::S);}
                                }
                        }
                        //
                        Event::KeyUp { keycode,..}=>
                        {
                                if let Some(tecla) = keycode{
                                        if let Some(index) = self.teclado.iter().position(|&x| x == *tecla ){
                                                self.teclado.remove(index);
                                        }
                                }
                        }
                        _ => {}
                }


                  
        }
        
       
       
}