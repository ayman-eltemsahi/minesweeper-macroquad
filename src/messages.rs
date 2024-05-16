use macroquad::{
    color::{BLACK, GREEN, RED},
    text::draw_text,
};

use crate::{grid::GridSection, utils::get_time_diff};

pub const FONT_SIZE: f32 = 20.0;

pub fn write_game_over(grid: &GridSection) {
    draw_text(
        "You lost :(",
        grid.x() + grid.w() / 2.0 - 10.0,
        grid.y() + grid.h() / 2.0,
        FONT_SIZE * 1.2,
        RED,
    );
}

pub fn write_you_win(grid: &GridSection) {
    draw_text(
        "You Win!",
        grid.x() + grid.w() / 2.0 - 10.0,
        grid.y() + grid.h() / 2.0,
        FONT_SIZE * 1.2,
        GREEN,
    );
}

pub fn write_remaining_mines(count: i32, grid: &GridSection) {
    draw_text(
        &format!("Mines: {}", count),
        grid.x(),
        grid.y() + grid.h() / 2.0,
        FONT_SIZE,
        BLACK,
    );
}

pub fn write_time(start_time: i64, end_time: i64, grid: &GridSection) {
    let (mins, secs) = get_time_diff(start_time, end_time);
    draw_text(
        &format!("{:02}:{:02}", mins, secs),
        grid.x() + grid.w() - FONT_SIZE * 2.0,
        grid.y() + grid.h() / 2.0,
        FONT_SIZE,
        BLACK,
    );
}
