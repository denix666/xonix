use macroquad::prelude::*;

pub struct Resources {
    pub intro_texture: Texture2D,
    pub bg_1: Texture2D,
    pub bg_2: Texture2D,
    pub bg_3: Texture2D,
    pub bg_4: Texture2D,
    pub level_completed_texture: Texture2D,
    pub level_failed_texture: Texture2D,
    pub game_over_texture: Texture2D,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            bg_1: load_texture("assets/images/bg_1.png").await.unwrap(),
            bg_2: load_texture("assets/images/bg_2.png").await.unwrap(),
            bg_3: load_texture("assets/images/bg_3.png").await.unwrap(),
            bg_4: load_texture("assets/images/bg_4.png").await.unwrap(),
            level_completed_texture: load_texture("assets/images/level_completed.png").await.unwrap(),
            level_failed_texture: load_texture("assets/images/level_failed.png").await.unwrap(),
            game_over_texture: load_texture("assets/images/game_over.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
        }
    }
}