use macroquad::prelude::*;

pub struct Resources {
    //pub intro_texture: Texture2D,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            //intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
        }
    }
}