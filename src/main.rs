use macroquad::prelude::*;

mod game;
use game::Game;

mod resources;
use resources::*;

mod levels;
use levels::*;

mod player;
use player::Player;

mod sea_enemy;
use sea_enemy::SeaEnemy;

mod land_enemy;
use land_enemy::LandEnemy;

const MAP_WIDTH: usize = 75;
const MAP_HEIGHT: usize = 50;
const INFO_BAR_HEIGHT: f32 = 40.0;
const WIN_RATIO: i32 = 80;
const SEA_ENEMY_SPEED: f64 = 0.04;
const LAND_ENEMY_SPEED: f64 = 0.10;

fn conf() -> Conf {
    let mut title = String::from("Xonix v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: MAP_WIDTH as i32 * 10,
        window_height: MAP_HEIGHT as i32 * 10 + INFO_BAR_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

pub enum GameState {
    Game,
    Intro,
    InitLevel,
    LevelCompleted,
    LevelFailed,
    GameOver,
}

fn fill(map: &mut Vec<Vec<Field>>, enemies: &Vec<SeaEnemy>) -> i32 {
    let mut q = std::collections::VecDeque::new();
    
    for y in 1..MAP_WIDTH - 1 {
        for x in 1..MAP_HEIGHT - 1 {
            if map[y][x] == Field::Sea {
                map[y][x] = Field::War;
            }
        }
    }

    for en in enemies {
        if map[en.pos_x][en.pos_y] == Field::War {
            q.push_back((en.pos_y, en.pos_x));
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                if map[y][x] == Field::War {
                    map[y][x] = Field::Sea;
                    q.push_back((x, y - 1));
                    q.push_back((x, y + 1));
                    q.push_back((x - 1, y));
                    q.push_back((x + 1, y));
                }
            }
        }
    }

    // Find all elements with sand  or war marker and mark them as land
    for y in 1..MAP_WIDTH - 1 {
        for x in 1..MAP_HEIGHT - 1 {
            if map[y][x] == Field::Sand || map[y][x] == Field::War {
                map[y][x] = Field::Land;
            }
        }
    }

    // Calculate fill ratio
    let mut elements: f32 = 0.0;
    for y in 1..MAP_WIDTH - 1 {
        for x in 1..MAP_HEIGHT - 1 {
            if map[y][x] == Field::Land {
                elements += 1.0;
            }
        }
    }

    (elements / (MAP_WIDTH as f32 * MAP_HEIGHT as f32) * 100.0).round() as i32
}

#[macroquad::main(conf)]
async fn main() {
    let resources = Resources::new().await;
    let mut game_state = GameState::Intro;
    let mut map: Vec<Vec<Field>> = make_map_array(MAP_HEIGHT, MAP_WIDTH);
    let mut game = Game::new().await;
    let mut player = Player::new().await;
    let mut sea_enemies: Vec<SeaEnemy> = Vec::new();
    let mut land_enemies: Vec<LandEnemy> = Vec::new();

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro=> {
                draw_texture(resources.intro_texture, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lvl_num = 1;
                    game.lives = 2;
                    game_state=GameState::InitLevel;
                }
            },
            GameState::InitLevel => {
                map.clear();
                map = make_map_array(MAP_HEIGHT, MAP_WIDTH);
                player = Player::new().await;
                sea_enemies.clear();
                game.ratio = 0;

                // Add number of enemies depends on level number
                for _ in 0..game.lvl_num {
                    sea_enemies.push(
                        SeaEnemy::new().await
                    );
                }

                for _ in 0..game.lvl_num {
                    land_enemies.push(
                        LandEnemy::new().await
                    );
                }

                game_state=GameState::Game;
            },
            GameState::Game => {
                draw_texture(background_texture(game.lvl_num, &resources).await, 20.0, 20.0, WHITE);
                draw_map(&map);

                if is_key_pressed(KeyCode::Left) {
                    player.dir = player::Dir::Left
                }
                if is_key_pressed(KeyCode::Right) {
                    player.dir = player::Dir::Right
                }
                if is_key_pressed(KeyCode::Up) {
                    player.dir = player::Dir::Up
                }
                if is_key_pressed(KeyCode::Down) {
                    player.dir = player::Dir::Down
                }
                
                if get_time() - player.last_move >= 0.04 {
                    player.previous_point = map[player.x][player.y];
                    match player.dir {
                        player::Dir::Left => {
                            if player.x > 0 {
                                player.x -= 1;
                            }
                        },
                        player::Dir::Right => {
                            if player.x < MAP_WIDTH - 1 {
                                player.x += 1;
                            }
                        },
                        player::Dir::Up => {
                            if player.y > 0 {
                                player.y -= 1;
                            }
                        },
                        player::Dir::Down => {
                            if player.y < MAP_HEIGHT - 1 {
                                player.y += 1;
                            }
                        },
                        player::Dir::None => {},
                    }
                    player.last_move = get_time();

                    if map[player.x][player.y] == Field::Sand {
                        game_state=GameState::LevelFailed;
                    } else {
                        if map[player.x][player.y] == Field::Sea {
                            map[player.x][player.y] = Field::Sand;
                        } else {
                            if player.previous_point != Field::Land {
                                // Check and fill
                                player.dir = player::Dir::None;
                                game.ratio =  fill(&mut map, &sea_enemies);

                                game.score += game.ratio;

                                if game.ratio >= WIN_RATIO {
                                    game_state=GameState::LevelCompleted
                                }
                            }
                        }
                    }
                }

                player.draw();

                for sea_enemy in &mut sea_enemies {
                    if map[sea_enemy.pos_x][sea_enemy.pos_y] == Field::Sand {
                        if game.lives >= 1 {
                            game.lives -= 1;
                            game_state=GameState::LevelFailed;
                        } else {
                            game_state=GameState::GameOver;
                        }
                    }
                    if get_time() - sea_enemy.last_move >= SEA_ENEMY_SPEED {
                        sea_enemy.update(&map);
                        sea_enemy.last_move = get_time();
                    }
                    sea_enemy.draw();
                }

                for land_enemy in &mut land_enemies {
                    if get_time() - land_enemy.last_move >= LAND_ENEMY_SPEED {
                        land_enemy.update(&map);
                        land_enemy.last_move = get_time();
                    }
                    land_enemy.draw();
                }
                
                draw_info(
                    resources.font, 
                    &game.score.to_string(), 
                    &game.lives.to_string(), 
                    &game.lvl_num.to_string(),
                    &game.ratio.to_string(),
                    ((MAP_WIDTH as i32) * 10) as f32,
                    ((MAP_HEIGHT as i32) * 10) as f32,
                );
            },
            GameState::LevelCompleted => {
                draw_texture(background_texture(game.lvl_num, &resources).await, 20.0, 20.0, WHITE);
                draw_info(
                    resources.font, 
                    &game.score.to_string(), 
                    &game.lives.to_string(), 
                    &game.lvl_num.to_string(),
                    &game.ratio.to_string(),
                    ((MAP_WIDTH as i32) * 10) as f32,
                    ((MAP_HEIGHT as i32) * 10) as f32,
                );
                draw_texture(resources.level_completed_texture, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game.lvl_num += 1;
                    game_state=GameState::InitLevel;
                }
            },
            GameState::LevelFailed=> {
                draw_map(&map);
                draw_info(
                    resources.font, 
                    &game.score.to_string(), 
                    &game.lives.to_string(), 
                    &game.lvl_num.to_string(),
                    &game.ratio.to_string(),
                    ((MAP_WIDTH as i32) * 10) as f32,
                    ((MAP_HEIGHT as i32) * 10) as f32,
                );
                draw_texture(resources.level_failed_texture, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    // Revert back sand to sea
                    for y in 0..MAP_WIDTH {
                        for x in 0..MAP_HEIGHT {
                            if map[y][x] == Field::Sand {
                                map[y][x] = Field::Sea;
                            }
                        }
                    }
                    player = Player::new().await;
                    game_state=GameState::Game;
                } 
            },
            GameState::GameOver => {
                draw_map(&map);
                draw_info(
                    resources.font, 
                    &game.score.to_string(), 
                    &game.lives.to_string(), 
                    &game.lvl_num.to_string(),
                    &game.ratio.to_string(),
                    ((MAP_WIDTH as i32) * 10) as f32,
                    ((MAP_HEIGHT as i32) * 10) as f32,
                );
                draw_texture(resources.game_over_texture, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lvl_num = 1;
                    game.lives = 2;
                    game_state=GameState::InitLevel;
                }
            }, 
        }

        next_frame().await
    }
}