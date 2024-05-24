use macroquad::{
    math::Vec2,
    shapes::draw_rectangle,
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{game_textures::GameTextures, vector2::Vector2};

mod consts {
    use macroquad::color::{Color, BLACK, LIGHTGRAY, RED, SKYBLUE, WHITE};

    pub const HIDDEN_COLOR: Color = SKYBLUE;
    pub const MINE_COLOR: Color = WHITE;
    pub const MINE_BACKGROUND_COLOR: Color = RED;
    pub const FLAG_BACKGROUND_COLOR: Color = HIDDEN_COLOR;
    pub const NO_MINE_COLOR: Color = LIGHTGRAY;
    pub const TEXT_COLOR: Color = BLACK;

    pub const DIGITS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    Hidden,
    Flagged,
    Revealed,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub has_mine: bool,
    pub state: TileState,
    pub num_mines_around: i32,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            has_mine: false,
            state: TileState::Hidden,
            num_mines_around: 0,
        }
    }
}

impl Tile {
    pub fn update_num_mines_around(&mut self, num_mines_around: i32) {
        self.num_mines_around = num_mines_around;
    }

    pub fn draw(&self, pos: Vector2<f32>, size: f32, textures: &GameTextures) {
        let color = match self.state {
            TileState::Hidden => consts::HIDDEN_COLOR,
            TileState::Flagged => consts::FLAG_BACKGROUND_COLOR,
            TileState::Revealed if self.has_mine => consts::MINE_BACKGROUND_COLOR,
            _ => consts::NO_MINE_COLOR,
        };

        draw_rectangle(pos.x, pos.y, size, size, color);

        if let Some(texture) = match self.state {
            TileState::Flagged => Some(&textures.flag),
            TileState::Revealed if self.has_mine => Some(&textures.bomb),
            _ => None,
        } {
            draw_texture_ex(
                texture,
                pos.x,
                pos.y,
                consts::MINE_COLOR,
                Tile::get_texture_params(size),
            );
        }

        if self.num_mines_around > 0 && self.state == TileState::Revealed && !self.has_mine {
            let text_pos = pos
                .add_val(size / 2.0)
                .add(Vector2::new(-size / 5.0, size / 5.0));
            draw_text(
                consts::DIGITS[self.num_mines_around as usize],
                text_pos.x,
                text_pos.y,
                size,
                consts::TEXT_COLOR,
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
