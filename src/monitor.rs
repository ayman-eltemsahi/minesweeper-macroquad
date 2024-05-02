use macroquad::{color::BLACK, text::draw_text};

pub fn draw_fps(val: i32) {
    draw_text(&format!("fps: {}", val), 20.0, 20.0, 20.0, BLACK);
}
