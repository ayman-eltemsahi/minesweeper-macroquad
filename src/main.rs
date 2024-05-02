mod game;
mod monitor;
mod mouse;
mod tile;

use game::Game;
use macroquad::{
    prelude::{clear_background, next_frame, WHITE},
    time::get_fps,
};
use monitor::draw_fps;
use mouse::Mouse;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut game = Game::random_game(20, 20).await;

    let mut counter: i64 = 0;

    let mut last_fps = 0;
    let mut mouse = Mouse::new();

    loop {
        counter += 1;

        clear_background(WHITE);
        mouse.update();

        if let Some(pos) = mouse.is_left_key_up_same_pos() {
            println!("Left key up, pos: {:?}", pos);
            game.handle_left_mouse_click(pos);
        }

        game.draw();

        if counter % 20 == 0 {
            last_fps = get_fps();
        }
        draw_fps(last_fps);
        next_frame().await
    }
}
