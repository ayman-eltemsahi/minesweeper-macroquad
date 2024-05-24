mod config;
mod diagnostics;
mod game;
mod game_controls;
mod game_textures;
mod grid;
mod messages;
mod mouse;
mod tile;
mod utils;
mod vector2;

use config::Config;
use diagnostics::Diagnostics;
use game::{Game, GameState};
use macroquad::prelude::{clear_background, next_frame, WHITE};
use mouse::Mouse;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut game = Game::new(grid::Grid::new()).await;

    let mut diagnostics = Diagnostics::new();
    let mut mouse = Mouse::new();

    let config = Config::new();
    let controls = game_controls::GameControls::new(&config);

    loop {
        clear_background(WHITE);
        mouse.update();

        match game.get_state() {
            GameState::NotStarted | GameState::GameOver | GameState::GameWon => {
                controls.draw();
                if let Some(pos) = mouse.is_left_key_up_same_pos() {
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
                if let Some(pos) = mouse.is_left_key_up_same_pos() {
                    game.make_move(pos);
                } else if let Some(pos) = mouse.is_right_key_up_same_pos() {
                    game.mark_tile(pos);
                }
            }
        }

        game.draw();
        diagnostics.on_loop();
        next_frame().await
    }
}
