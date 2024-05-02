mod game;
mod monitor;
mod tile;
mod utils;

use game::Game;
use macroquad::{
    prelude::{clear_background, next_frame, WHITE},
    time::get_fps,
};
use monitor::draw_fps;

#[macroquad::main("BasicShapes")]
async fn main() {
    let game = Game::random_game(20, 20);

    let mut counter = 0;

    let mut last_fps = 0;

    loop {
        counter += 1;

        clear_background(WHITE);

        game.draw();

        if counter % 20 == 0 {
            last_fps = get_fps();
        }
        draw_fps(last_fps);
        next_frame().await
    }
}
