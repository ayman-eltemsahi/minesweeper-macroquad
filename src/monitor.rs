use macroquad::{color::BLACK, text::draw_text, window::screen_width};

pub fn draw_fps(val: i32) {
    draw_text(&format!("fps: {}", val), screen_width() - 100.0, 20.0, 20.0, BLACK);
}
