use macroquad::{
    color::{BLACK, GREEN, RED},
    text::draw_text,
    window::screen_width,
};

use crate::utils::get_time_diff;

pub const FONT_SIZE: f32 = 20.0;

const HEADER_HIGHT: f32 = 60.0;

pub fn write_game_over() {
    draw_text(
        "You lost :(",
        screen_width() / 2.0 - 20.0,
        HEADER_HIGHT / 2.0,
        FONT_SIZE * 1.2,
        RED,
    );
}

pub fn write_you_win() {
    draw_text(
        "You Win!",
        screen_width() / 2.0 - 20.0,
        HEADER_HIGHT / 2.0,
        FONT_SIZE * 1.2,
        GREEN,
    );
}

pub fn write_remaining_mines(count: i32) {
    draw_text(
        &format!("Mines: {}", count),
        10.0,
        HEADER_HIGHT / 2.0,
        FONT_SIZE,
        BLACK,
    );
}

pub fn write_time(start_time: i64, end_time: i64) {
    let (mins, secs) = get_time_diff(start_time, end_time);

    draw_text(
        &format!("{:02}:{:02}", mins, secs),
        screen_width() - FONT_SIZE * 3.0,
        HEADER_HIGHT / 2.0,
        FONT_SIZE,
        BLACK,
    );
}
