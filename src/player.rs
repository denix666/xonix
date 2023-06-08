use macroquad::prelude::*;

pub enum Dir {
    Left,
    Right,
    Up,
    Down,
    None,
}

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub last_move: f64,
    pub dir: Dir,
    pub previous_point: crate::levels::Field,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            x: super::MAP_WIDTH / 2,
            y: 0,
            last_move: get_time(),
            dir: Dir::None,
            previous_point: crate::levels::Field::Land,
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle(
            self.x as f32 * 10.0, 
            self.y as f32 * 10.0,
            10.0, 
            10.0, 
            GREEN);

        draw_rectangle(
            self.x as f32 * 10.0 + 2.5, 
            self.y as f32 * 10.0 + 2.5,
            6.0, 
            6.0, 
            BLACK);
    }
}
