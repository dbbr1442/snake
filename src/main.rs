use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use macroquad::rand::srand;

use crate::block::Block;
use crate::text_helper::text_helper;
use crate::game_handler::Game;
use crate::snake::Direction;

mod game_handler;
mod block;
mod snake;
mod text_helper;

const GREY: Color = GRAY;

fn config() -> Conf {
    Conf {
        window_height: 700,
        window_width: 600,
        window_title: "Snake".to_string(),
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {

    if cfg!(target_family = "wasm") {
        srand((get_time() * 10000.0) as u64);
    } else {
        srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    }
    let mut game = Game::new();
    let mut overlap = false;

    'main: loop {
        clear_background(BLACK);

        for key in get_keys_pressed() {
            match key {
                KeyCode::A => game.snake.rotate(Direction::West),
                KeyCode::W => game.snake.rotate(Direction::North),
                KeyCode::D => game.snake.rotate(Direction::East),
                KeyCode::S => game.snake.rotate(Direction::South),
                KeyCode::Escape => break 'main,
                KeyCode::Enter => {
                    if game.lost || game.snake.blocks.len() > 395 {
                        game = Game::new();
                    }
                },
                _ => (),
            }
        }

        if game.lost {
            text_helper(None, 64, 300.0, 200.0, "YOU LOST");
            text_helper(None, 64, 300.0, 300.0, "ENTER TO PLAY AGAIN");
            text_helper(None, 64, 300.0, 400.0, "ESC TO EXIT");

        } else if game.snake.blocks.len() > 395 {
            text_helper(None, 64, 300.0, 200.0, "YOU WON");
            text_helper(None, 64, 300.0, 300.0, "ENTER TO PLAY AGAIN");
            text_helper(None, 64, 300.0, 400.0, "ESC TO EXIT");
        } else {
            if get_time()-game.timer > 0.1 {
                if overlap {
                    game = game.move_snake(true);
                    overlap = false;
                } else {
                    game = game.move_snake(false);
                }
                game.check_lose();

                game.timer = get_time();
            }


            game.snake.draw();

            if game.snake.head.check_overlap(&game.apple) {
                overlap = true;
                let location = 'find_open: loop {
                    let location = Block::random();
                    for block in game.snake.blocks.iter() {
                        if block.check_overlap(&location) {
                            continue 'find_open;
                        }
                    }

                    if location.check_overlap(&game.snake.head) {
                        continue 'find_open;
                    }

                    break 'find_open location;
                };
                game.apple = location;
            }

            game.apple.draw(RED);

            text_helper(None, 60, 300.0, 650.0, &format!("SCORE {}", game.snake.blocks.len()));


            grid();
        }
        
        next_frame().await
    }
}

fn grid() {
    for i in 0..=20 {
        let i: i32 = i*30;
        draw_rectangle(i as f32-1.0, 0.0, 2.0, 600.0, GREY);
        draw_rectangle(0.0, i as f32-1.0, 600.0, 2.0, GREY);
    }
    //draw_rectangle(0.0, 600.0, 600.0, 2.0, GREY);
}
