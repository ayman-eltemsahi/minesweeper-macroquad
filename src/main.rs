mod config;
mod game;
mod game_controls;
mod game_textures;
mod messages;
mod mouse;
mod tile;
mod utils;
mod vector2;

use config::Config;
use game::{Game, GameState};
use macroquad::prelude::{clear_background, next_frame, WHITE};
use mouse::{is_mouse_left_btn_pressed, is_mouse_right_btn_pressed};

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut game = Game::new().await;

    let config = Config::new();
    let controls = game_controls::GameControls::new(&config);

    loop {
        clear_background(WHITE);

        match game.get_state() {
            GameState::NotStarted | GameState::GameOver | GameState::GameWon => {
                controls.draw();
                if let Some(pos) = is_mouse_left_btn_pressed() {
                    match controls.handle_input(pos) {
                        Some((index, _level)) => {
                            let level = &config.levels[index];
                            game.start(level.rows, level.cols, level.mines);
                        }
                        _ => {}
                    }
                }
            }
            GameState::Playing => {
                if let Some(pos) = is_mouse_left_btn_pressed() {
                    game.make_move(pos);
                } else if let Some(pos) = is_mouse_right_btn_pressed() {
                    game.mark_tile(pos);
                }
            }
        }

        game.draw();
        next_frame().await
    }
}
