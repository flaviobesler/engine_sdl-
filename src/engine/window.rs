pub struct Window{
        nome: String,
        largura: u32,
        altura: u32,
        pub janela: sdl3::video::Window

}

impl Window{
        pub fn new(     nome: String, 
                        altura: u32, 
                        largura: u32, 
                        video: &sdl3::VideoSubsystem)-> Result< Self, Box <dyn std::error::Error>>{
                
                let janela  = video
                .window(&nome,largura, altura)
                .position_centered()
                .build()?;

                Ok(Self {       nome,
                                largura, 
                                altura, 
                                janela })
                
        
        }

}