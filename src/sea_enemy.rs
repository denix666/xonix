use macroquad::prelude::*;

pub enum HorizontalDir {
    Left,
    Right,
}

pub enum VerticalDir {
    Up,
    Down,
}

pub struct SeaEnemy {
    pub x: f32,
    pub y: f32,
    pub pos_x: usize,
    pub pos_y: usize,
    pub last_move: f64,
    pub horizontal_dir: HorizontalDir,
    pub vertical_dir: VerticalDir,
}


impl SeaEnemy {
    pub async fn new() -> Self {
        let rand_x = macroquad::rand::gen_range(20, super::MAP_WIDTH - 20);
        let rand_y = macroquad::rand::gen_range(20, super::MAP_HEIGHT - 20);

        let rand_horizontal_dir: HorizontalDir = match macroquad::rand::gen_range(0, 1) { 
            0 => HorizontalDir::Right,
            _ => HorizontalDir::Left,
        };

        let rand_vertical_dir: VerticalDir = match macroquad::rand::gen_range(0, 1) { 
            0 => VerticalDir::Up,
            _ => VerticalDir::Down,
        };
        
        Self {
            last_move: get_time(),
            horizontal_dir: rand_horizontal_dir,
            vertical_dir: rand_vertical_dir,
            x: rand_x as f32,
            y: rand_y as f32,
            pos_x: rand_x,
            pos_y: rand_y,
        }
    }

    pub fn update(&mut self, map: &Vec<Vec<crate::levels::Field>>) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        match self.horizontal_dir {
            HorizontalDir::Left => {
                if map[self.pos_x - 1][self.pos_y] == crate::levels::Field::Land {
                    dx += 1.0;
                    self.horizontal_dir = HorizontalDir::Right
                } else {
                    dx -= 1.0;
                }
            },
            HorizontalDir::Right => {
                if map[self.pos_x + 1][self.pos_y] == crate::levels::Field::Land {
                    dx -= 1.0;
                    self.horizontal_dir = HorizontalDir::Left
                } else {
                    dx += 1.0;
                }
            },
        }

        match self.vertical_dir {
            VerticalDir::Up => {
                if map[self.pos_x][self.pos_y - 1] == crate::levels::Field::Land {
                    dy += 1.0;
                    self.vertical_dir = VerticalDir::Down
                } else {
                    dy -= 1.0;
                }
            },
            VerticalDir::Down => {
                if map[self.pos_x][self.pos_y + 1] == crate::levels::Field::Land {
                    dy -= 1.0;
                    self.vertical_dir = VerticalDir::Up
                } else {
                    dy += 1.0;
                }
            },
        }

        self.x += dx;
        self.y += dy;

        self.pos_x = self.x as usize;
        self.pos_y = self.y as usize;
    }

    pub fn draw(&mut self) {
        draw_circle(
            self.x * 10.0 + 5.0, 
            self.y * 10.0 + 5.0,
            5.0,
            YELLOW)
    }
}
