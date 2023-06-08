use macroquad::{
    shapes::draw_rectangle, 
    prelude::{DARKBLUE, BLACK, ORANGE, WHITE, DARKGRAY}, 
    text::{Font, draw_text_ex, TextParams}
};

#[derive(Clone, PartialEq, Copy)]
pub enum Field {
    Land,
    Sea,
    War,
    Sand,
}

pub fn draw_info(
    font: Font, 
    score: &str, 
    lives: &str, 
    lvl_num: &str,
    ratio: &str,
    map_width: f32,
    map_height: f32,
) {
    // Draw area for game information
    draw_rectangle(
        1.0, 
        map_height,
        map_width, 
        super::INFO_BAR_HEIGHT, 
        DARKGRAY);


    let info_text = format!("SCORE: {}     LIVES: {}     LEVEL: {}     RATIO: {}%", score, lives, lvl_num, ratio).to_string();

    draw_text_ex(info_text.as_str(), 40.0, map_height + 30.0, 
        TextParams {
            font,
            font_size: 25,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_map(map: &Vec<Vec<Field>>) {
    let r = map.len();
    let c = map[0].len();

    for i in 0..r {
        for j in 0..c {
            match map[i][j] {
                Field::Land => { // Land (0)
                    draw_rectangle(
                        i as f32 * 10.0, 
                        j as f32 * 10.0,
                        10.0, 
                        10.0, 
                        BLACK)
                },
                Field::Sea => { // Sea (1)
                    draw_rectangle(
                        i as f32 * 10.0, 
                        j as f32 * 10.0,
                        10.0, 
                        10.0, 
                        DARKBLUE)
                },
                Field::Sand => { // Sand (2)
                    draw_rectangle(
                        i as f32 * 10.0, 
                        j as f32 * 10.0,
                        10.0, 
                        10.0, 
                        ORANGE)
                },
                _ => {}
            }
        }
    }
}


pub fn make_map_array(width: usize, height: usize) -> Vec<Vec<Field>> {

    let mut map: Vec<Vec<Field>> = vec![vec![Field::Sea; width]; height];
    
    for (y, row) in map.iter_mut().enumerate() {
        for (x, field) in row.iter_mut().enumerate() {
            if x < 2 || x >= width - 2 || y < 2 || y >= height - 2 {
                *field = Field::Land;
            }
        }
    }

    return map
}