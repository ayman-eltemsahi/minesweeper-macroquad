use std::collections::VecDeque;

use macroquad::{
    texture::{load_texture, Texture2D},
    window::{screen_height, screen_width},
};
use rand::Rng;

use crate::tile::Tile;

pub const TILE_SIZE: f32 = 15.0;
pub const SCREEN_MARGIN: f32 = 30.0;
pub const BOMBS_RATIO: f32 = 0.3;

const NEIGHBORS: &'static [(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

#[derive(Debug)]
pub struct Game {
    pub rows: i32,
    pub cols: i32,
    pub tile_size: f32,
    tiles: Vec<Tile>,
    explosion_texture: Texture2D,
}

impl Game {
    pub async fn random_game(rows: i32, cols: i32) -> Game {
        let explosion_texture: Texture2D = load_texture("textures/explosion.png").await.unwrap();

        let mut rng = rand::thread_rng();

        let mut tiles = Vec::new();
        for _ in 0..rows * cols {
            let has_bomb = rng.gen::<f32>() < BOMBS_RATIO;
            tiles.push(Tile::new(has_bomb));
        }

        let mut game = Game {
            rows,
            cols,
            tile_size: TILE_SIZE,
            tiles,
            explosion_texture,
        };

        game.update_bombs_count();
        return game;
    }

    pub fn handle_left_mouse_click(&mut self, pos: (f32, f32)) {
        let (tile_width, tile_height) = self.get_tile_size();

        let x = pos.0 - SCREEN_MARGIN;
        let y = pos.1 - SCREEN_MARGIN;

        let j = (x / tile_width) as i32;
        let i = (y / tile_height) as i32;

        // check if out of bounds
        if i < 0 || i >= self.rows || j < 0 || j >= self.cols {
            return;
        }

        let index = (i * self.rows + j) as usize;
        // check if the tile has been clicked before
        if !self.tiles[index].is_hidden {
            return;
        }

        self.handle_click_on_tile(i, j);
    }

    pub fn draw(&self) {
        let (tile_width, tile_height) = self.get_tile_size();

        for i in 0..self.rows {
            for j in 0..self.cols {
                let index = (i * self.rows + j) as usize;
                self.tiles[index].draw(
                    SCREEN_MARGIN + j as f32 * (tile_width),
                    SCREEN_MARGIN + i as f32 * (tile_height),
                    tile_width - 1.0,
                    tile_height - 1.0,
                    &self.explosion_texture,
                );
            }
        }
    }

    fn handle_click_on_tile(&mut self, i: i32, j: i32) {
        let index = self.get_index(i, j);
        let tile = &mut self.tiles[index];
        tile.is_hidden = false;

        if tile.has_bomb {
            println!("Game over!");
        } else {
            self.clear_empty_neighbours(i, j);
        }
    }

    fn clear_empty_neighbours(&mut self, i: i32, j: i32) {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((i, j));

        while let Some((i, j)) = q.pop_front() {
            for (dx, dy) in NEIGHBORS {
                let (x, y) = (i + *dx, j + *dy);
                if x < 0 || y < 0 || x >= self.cols || y >= self.rows {
                    continue;
                }

                let other_tile_index = self.get_index(x, y);
                let other_tile = &mut self.tiles[other_tile_index];

                if other_tile.has_bomb || !other_tile.is_hidden {
                    continue;
                }

                other_tile.is_hidden = false;

                if other_tile.num_bombs_around == 0 {
                    q.push_back((x, y));
                }
            }
        }
    }

    fn update_bombs_count(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let count = self.count_bombs_around(i, j);
                let index = self.get_index(i, j);
                self.tiles[index].update_num_bombs_around(count);
            }
        }
    }

    fn count_bombs_around(&self, i: i32, j: i32) -> i32 {
        let mut count = 0;

        for (dx, dy) in NEIGHBORS {
            let (x, y) = (i + *dx, j + *dy);
            if x < 0 || y < 0 || x >= self.cols || y >= self.rows {
                continue;
            }

            let other_tile_index = self.get_index(x, y);
            if self.tiles[other_tile_index].has_bomb {
                count += 1;
            }
        }

        return count;
    }

    fn get_tile_size(&self) -> (f32, f32) {
        let tile_width = (screen_width() - SCREEN_MARGIN * 2.0) / self.cols as f32;
        let tile_height = (screen_height() - SCREEN_MARGIN * 2.0) / self.rows as f32;
        (tile_width, tile_height)
    }

    fn get_index(&self, i: i32, j: i32) -> usize {
        (i * self.rows + j) as usize
    }
}
