mod game;
mod game_controls;
mod game_state;
mod game_textures;
mod grid;
mod messages;
mod monitor;
mod mouse;
mod tile;
mod utils;

use game::Game;
use game_controls::GameLevel;
use game_state::GameState;
use macroquad::{
    prelude::{clear_background, next_frame, WHITE},
    time::get_fps,
};
use monitor::draw_fps;
use mouse::Mouse;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut game = Game::random_game(grid::Grid::new()).await;

    let mut counter: i64 = 0;

    let mut last_fps = 0;
    let mut mouse = Mouse::new();

    let mut state = GameState::NotStarted;
    let controls = game_controls::GameControls::new();

    loop {
        counter += 1;

        clear_background(WHITE);
        mouse.update();

        match state {
            GameState::NotStarted | GameState::GameOver | GameState::GameWon => {
                if state != GameState::GameOver || state == GameState::GameOver {
                    game.draw();
                }

                controls.draw();
                if let Some(pos) = mouse.is_left_key_up_same_pos() {
                    match controls.handle_input(pos) {
                        Some(GameLevel::Beginner) => {
                            game.start(9, 9, 10);
                            state = GameState::Playing;
                        }
                        Some(GameLevel::Intermediate) => {
                            game.start(16, 16, 40);
                            state = GameState::Playing;
                        }
                        Some(GameLevel::Expert) => {
                            game.start(16, 30, 99);
                            state = GameState::Playing;
                        }
                        _ => {}
                    }
                }
            }
            GameState::Playing => {
                if let Some(pos) = mouse.is_left_key_up_same_pos() {
                    println!("Left key up, pos: {:?}", pos);
                    game.make_move(pos);
                } else if let Some(pos) = mouse.is_right_key_up_same_pos() {
                    println!("Right key up, pos: {:?}", pos);
                    game.mark_tile(pos);
                }

                game.draw();
                state = game.get_state();
            }
        }

        if counter % 20 == 0 {
            last_fps = get_fps();
        }
        draw_fps(last_fps);
        next_frame().await
    }
}
