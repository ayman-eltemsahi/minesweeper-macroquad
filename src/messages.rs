use macroquad::{
    color::BLACK,
    text::{draw_text_ex, TextParams},
    time::get_time,
    window::{screen_height, screen_width},
};

pub fn write_game_over() {
    draw_text_ex(
        "Game over",
        screen_width() / 2.0 - 200.0,
        screen_height() / 2.0 - 20.0,
        TextParams {
            font_size: 100,
            font_scale: get_time().sin() as f32 / 20.0 + 1.0,
            color: BLACK,
            ..Default::default()
        },
    );
}
