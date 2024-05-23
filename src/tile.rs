use macroquad::{
    color::{Color, BLACK, LIGHTGRAY, RED, SKYBLUE, WHITE},
    math::Vec2,
    shapes::draw_rectangle,
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{game_textures::GameTextures, vector2::Vector2};

pub const HIDDEN_COLOR: Color = SKYBLUE;
pub const MINE_COLOR: Color = WHITE;
pub const MINE_BACKGROUND_COLOR: Color = RED;
pub const FLAG_BACKGROUND_COLOR: Color = HIDDEN_COLOR;
pub const NO_MINE_COLOR: Color = LIGHTGRAY;
pub const TEXT_COLOR: Color = BLACK;

const DIGITS: &'static [&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[derive(Debug)]
pub struct Tile {
    pub has_mine: bool,
    pub is_hidden: bool,
    pub is_marked: bool,
    pub num_mines_around: i32,
}

impl Tile {
    pub fn new(has_mine: bool) -> Tile {
        Tile {
            has_mine,
            is_hidden: true,
            is_marked: false,
            num_mines_around: 0,
        }
    }

    pub fn update_num_mines_around(&mut self, num_mines_around: i32) {
        self.num_mines_around = num_mines_around;
    }

    pub fn draw(&self, pos: Vector2<f32>, size: f32, textures: &GameTextures) {
        let color = match self.is_hidden {
            true => match self.is_marked {
                true => FLAG_BACKGROUND_COLOR,
                false => HIDDEN_COLOR,
            },
            false => match self.has_mine {
                true => MINE_BACKGROUND_COLOR,
                false => NO_MINE_COLOR,
            },
        };

        draw_rectangle(pos.x, pos.y, size, size, color);

        let texture = if !self.is_hidden && self.has_mine {
            Some(&textures.bomb)
        } else if self.is_hidden && self.is_marked {
            Some(&textures.flag)
        } else {
            None
        };

        if let Some(texture) = texture {
            draw_texture_ex(
                texture,
                pos.x,
                pos.y,
                MINE_COLOR,
                Tile::get_texture_params(size),
            );
        }

        if self.num_mines_around > 0 && !self.is_hidden && !self.has_mine {
            let text_pos = pos
                .add_val(size / 2.0)
                .add(Vector2::new(-size / 5.0, size / 5.0));
            draw_text(
                DIGITS[self.num_mines_around as usize],
                text_pos.x,
                text_pos.y,
                size,
                TEXT_COLOR,
            );
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
