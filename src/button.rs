use macroquad::{
    color::{Color, BLACK},
    math::vec2,
    ui::{root_ui, widgets},
};

pub const WIDTH: f32 = 190.0;
pub const HEIGHT: f32 = 30.0;

pub struct Button {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Button {
    pub fn new(text: &str, x: f32, y: f32) -> Button {
        Button {
            text: text.to_string(),
            x,
            y,
            w: WIDTH,
            h: HEIGHT,
            color: BLACK,
        }
    }

    pub fn draw(&self) {
        widgets::Button::new(self.text.as_str())
            .size(vec2(self.w, self.h))
            .position(vec2(self.x, self.y))
            .ui(&mut root_ui());
    }

    pub fn intersects(&self, pos: (f32, f32)) -> bool {
        pos.0 >= self.x && pos.0 <= self.x + self.w && pos.1 >= self.y && pos.1 <= self.y + self.h
    }

    pub fn get_height() -> f32 {
        HEIGHT
    }
}
