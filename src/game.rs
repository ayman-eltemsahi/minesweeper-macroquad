use macroquad::window::{screen_height, screen_width};
use rand::Rng;

use crate::tile::Tile;

pub const TILE_SIZE: f32 = 15.0;
pub const SCREEN_MARGIN: f32 = 30.0;
pub const BOMBS_RATIO: f32 = 0.3;

#[derive(Debug)]
pub struct Game {
    pub rows: i32,
    pub cols: i32,
    pub tile_size: f32,
    tiles: Vec<Tile>,
}

impl Game {
    pub fn random_game(rows: i32, cols: i32) -> Game {
        let mut rng = rand::thread_rng();

        let mut tiles = Vec::new();
        for _ in 0..rows * cols {
            let has_bomb = rng.gen::<f32>() < BOMBS_RATIO;
            tiles.push(Tile::new(has_bomb));
        }

        Game {
            rows,
            cols,
            tile_size: TILE_SIZE,
            tiles,
        }
    }

    pub fn draw(&self) {
        let tile_width = (screen_width() - SCREEN_MARGIN * 2.0) / self.cols as f32;
        let tile_height = (screen_height() - SCREEN_MARGIN * 2.0) / self.rows as f32;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let index = (i * self.cols + j) as usize;
                self.tiles[index].draw(
                    SCREEN_MARGIN + j as f32 * (tile_width),
                    SCREEN_MARGIN + i as f32 * (tile_height),
                    tile_width - 1.0,
                    tile_height - 1.0,
                );
            }
        }
    }
}
