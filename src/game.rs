use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub lvl_num: i32,
    pub ratio: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            lvl_num: 0,
            ratio: 0,
        }
    }
}
