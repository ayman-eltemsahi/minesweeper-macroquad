use macroquad::{
    color::{Color, BLACK, LIGHTGRAY, SKYBLUE},
    shapes::draw_rectangle,
};

pub const HIDDEN_COLOR: Color = SKYBLUE;
pub const BOMB_COLOR: Color = BLACK;
pub const NO_BOMB_COLOR: Color = LIGHTGRAY;

#[derive(Debug)]
pub struct Tile {
    pub has_bomb: bool,
    pub is_hidden: bool,
}

impl Tile {
    pub fn new(has_bomb: bool) -> Tile {
        Tile {
            has_bomb,
            is_hidden: false,
        }
    }

    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32) {
        let color = match self.is_hidden {
            true => HIDDEN_COLOR,
            false => match self.has_bomb {
                true => BOMB_COLOR,
                false => NO_BOMB_COLOR,
            },
        };

        draw_rectangle(x, y, w, h, color);
    }
}
