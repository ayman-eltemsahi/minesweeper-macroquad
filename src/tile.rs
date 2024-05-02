use macroquad::{
    color::{Color, BLACK, LIGHTGRAY, RED, SKYBLUE, WHITE},
    math::Vec2,
    shapes::draw_rectangle,
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

pub const HIDDEN_COLOR: Color = SKYBLUE;
pub const BOMB_COLOR: Color = WHITE;
pub const BOMB_BACKGROUND_COLOR: Color = RED;
pub const NO_BOMB_COLOR: Color = LIGHTGRAY;
pub const TEXT_COLOR: Color = BLACK;
pub const TEXT_FONT_SIZE: f32 = 30.0;

#[derive(Debug)]
pub struct Tile {
    pub has_bomb: bool,
    pub is_hidden: bool,
    pub num_bombs_around: i32,
}

impl Tile {
    pub fn new(has_bomb: bool) -> Tile {
        Tile {
            has_bomb,
            is_hidden: true,
            num_bombs_around: 0,
        }
    }

    pub fn update_num_bombs_around(&mut self, num_bombs_around: i32) {
        self.num_bombs_around = num_bombs_around;
    }

    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32, explosion_texture: &Texture2D) {
        let color = match self.is_hidden {
            true => HIDDEN_COLOR,
            false => match self.has_bomb {
                true => BOMB_BACKGROUND_COLOR,
                false => NO_BOMB_COLOR,
            },
        };

        draw_rectangle(x, y, w, h, color);

        if !self.is_hidden && self.has_bomb {
            let s = if w < h { w } else { h };
            draw_texture_ex(
                explosion_texture,
                x + (w - s) / 2.0,
                y + (h - s) / 2.0,
                BOMB_COLOR,
                Tile::get_texture_params(s),
            );
        }

        if self.num_bombs_around > 0 && !self.is_hidden && !self.has_bomb {
            let val = self.num_bombs_around.to_string();
            draw_text(&val, x + h / 2.0, y + w / 2.0, TEXT_FONT_SIZE, TEXT_COLOR);
        }
    }

    fn get_texture_params(size: f32) -> DrawTextureParams {
        DrawTextureParams {
            dest_size: Option::Some(Vec2 { x: size, y: size }),
            source: None,
            rotation: 0.0,
            pivot: None,
            flip_x: false,
            flip_y: false,
        }
    }
}
