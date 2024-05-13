use std::collections::VecDeque;

use macroquad::{
    color::BLACK,
    text::draw_text,
    texture::{load_texture, Texture2D},
    window::{screen_height, screen_width},
};
use rand::Rng;

use crate::{
    game_state::GameState,
    messages::{write_game_over, write_you_win},
    tile::Tile,
    utils::{current_time_seconds, get_time_diff},
};

pub const TILE_SIZE: f32 = 15.0;
pub const SCREEN_MARGIN: f32 = 30.0;
pub const SCREEN_BOTTOM_MARGIN: f32 = 30.0;
pub const MINES_RATIO: f32 = 0.3;

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
    pub start_time: i64,
    pub end_time: i64,
    tiles: Vec<Tile>,

    initial_mines_count: i32,
    marked_mines_count: i32,

    explosion_texture: Texture2D,
    flag_texture: Texture2D,

    state: GameState,
}

impl Game {
    pub async fn random_game() -> Game {
        let explosion_texture: Texture2D = load_texture("textures/explosion.png").await.unwrap();
        let flag_texture: Texture2D = load_texture("textures/flag.png").await.unwrap();

        Game {
            rows: 0,
            cols: 0,
            tile_size: TILE_SIZE,
            tiles: Vec::new(),
            start_time: 0,
            end_time: 0,
            initial_mines_count: 0,
            marked_mines_count: 0,

            state: GameState::NotStarted,

            explosion_texture,
            flag_texture,
        }
    }

    pub fn start(&mut self, rows: i32, cols: i32) {
        self.rows = rows;
        self.cols = cols;
        self.start_time = current_time_seconds();
        self.state = GameState::Playing;

        let mut rng = rand::thread_rng();

        let mut tiles = Vec::new();
        for _ in 0..rows * cols {
            let has_mine = rng.gen::<f32>() < MINES_RATIO;
            tiles.push(Tile::new(has_mine));
        }

        let initial_mines_count = tiles.iter().filter(|tile| tile.has_mine).count() as i32;

        self.initial_mines_count = initial_mines_count;
        self.marked_mines_count = 0;
        self.tiles = tiles;
        self.update_mines_count();
    }

    pub fn end(&mut self, state: GameState) {
        self.state = state;
        self.end_time = current_time_seconds();
    }

    pub fn make_move(&mut self, pos: (f32, f32)) {
        if self.state != GameState::Playing {
            eprintln!("Game is not in playing state");
            return;
        }

        let (i, j, index) = match self.resolve_tile_position(pos) {
            Some(value) => value,
            None => return,
        };

        let tile = &mut self.tiles[index];
        if tile.is_hidden && tile.is_marked {
            return;
        }

        if !tile.is_hidden && tile.has_mine {
            return;
        }

        if tile.is_hidden {
            tile.is_hidden = false;

            if tile.has_mine {
                println!("Game over!");
                self.end(GameState::GameOver);
            } else {
                self.clear_empty_neighbours(i, j);
            }
        } else {
            self.click_on_shown_tile(i, j);
        }

        if self.has_won() {
            self.end(GameState::GameWon);
        }
    }

    pub fn has_won(&self) -> bool {
        self.tiles
            .iter()
            .all(|tile| (!tile.has_mine && !tile.is_hidden) || (tile.has_mine && tile.is_marked))
    }

    pub fn click_on_shown_tile(&mut self, i: i32, j: i32) {
        let index = self.get_index(i, j);
        let tile = &mut self.tiles[index];

        if tile.num_mines_around == 0 || !self.is_tile_cleared(i, j) {
            return;
        }

        self.clear_empty_neighbours(i, j);
    }

    pub fn mark_tile(&mut self, pos: (f32, f32)) {
        if self.state != GameState::Playing {
            eprintln!("Game is not in playing state");
            return;
        }

        let (_, _, index) = match self.resolve_tile_position(pos) {
            Some(value) => value,
            None => return,
        };

        let tile = &mut self.tiles[index];
        if !tile.is_hidden {
            return;
        }

        tile.is_marked = !tile.is_marked;
        self.marked_mines_count += match tile.is_marked {
            true => 1,
            false => -1,
        };

        if self.has_won() {
            self.end(GameState::GameWon);
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    fn resolve_tile_position(&mut self, pos: (f32, f32)) -> Option<(i32, i32, usize)> {
        let (tile_width, tile_height) = self.get_tile_size();
        let x = pos.0 - SCREEN_MARGIN;
        let y = pos.1 - SCREEN_MARGIN;
        let j = (x / tile_width) as i32;
        let i = (y / tile_height) as i32;
        if i < 0 || i >= self.rows || j < 0 || j >= self.cols {
            None
        } else {
            Some((i, j, self.get_index(i, j)))
        }
    }

    pub fn draw(&self) {
        self.draw_tiles();
        self.write_time();
        self.write_remaining_mines();

        match self.state {
            GameState::GameOver => write_game_over(),
            GameState::GameWon => write_you_win(),
            _ => {}
        }
    }

    fn draw_tiles(&self) {
        let (tile_width, tile_height) = self.get_tile_size();

        for i in 0..self.rows {
            for j in 0..self.cols {
                let index = (i * self.cols + j) as usize;
                self.tiles[index].draw(
                    SCREEN_MARGIN + j as f32 * (tile_width),
                    SCREEN_MARGIN + i as f32 * (tile_height),
                    tile_width - 1.0,
                    tile_height - 1.0,
                    &self.explosion_texture,
                    &self.flag_texture,
                );
            }
        }
    }

    fn write_remaining_mines(&self) {
        draw_text(
            &format!(
                "Remaining mines: {}",
                self.initial_mines_count - self.marked_mines_count
            ),
            20.0,
            screen_height() - SCREEN_BOTTOM_MARGIN,
            20.0,
            BLACK,
        );
    }

    fn write_time(&self) {
        let end_time = match self.state {
            GameState::GameOver | GameState::GameWon => self.end_time,
            _ => current_time_seconds(),
        };

        let (mins, secs) = get_time_diff(self.start_time, end_time);
        draw_text(
            &format!("Remaining time: {:02}:{:02}", mins, secs),
            20.0,
            screen_height() - SCREEN_BOTTOM_MARGIN + 20.0,
            20.0,
            BLACK,
        );
    }

    fn clear_empty_neighbours(&mut self, i: i32, j: i32) {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((i, j));

        while let Some((i, j)) = q.pop_front() {
            for (dx, dy) in NEIGHBORS {
                let (x, y) = (i + *dx, j + *dy);
                if x < 0 || y < 0 || x >= self.rows || y >= self.cols {
                    continue;
                }

                let other_tile_index = self.get_index(x, y);
                let other_tile = &mut self.tiles[other_tile_index];

                if other_tile.has_mine || !other_tile.is_hidden || other_tile.is_marked {
                    continue;
                }

                other_tile.is_hidden = false;

                if other_tile.num_mines_around == 0 {
                    q.push_back((x, y));
                }
            }
        }
    }

    fn update_mines_count(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let count = self.count_mines_around(i, j);
                let index = self.get_index(i, j);
                self.tiles[index].update_num_mines_around(count);
            }
        }
    }

    fn count_mines_around(&self, i: i32, j: i32) -> i32 {
        let mut count = 0;

        for (dx, dy) in NEIGHBORS {
            let (x, y) = (i + *dx, j + *dy);
            if x < 0 || y < 0 || x >= self.rows || y >= self.cols {
                continue;
            }

            let other_tile_index = self.get_index(x, y);
            if self.tiles[other_tile_index].has_mine {
                count += 1;
            }
        }

        return count;
    }

    fn is_tile_cleared(&self, i: i32, j: i32) -> bool {
        let index = self.get_index(i, j);
        if self.tiles[index].has_mine {
            return false;
        }

        let mut count = self.tiles[index].num_mines_around;

        for (dx, dy) in NEIGHBORS {
            let (x, y) = (i + *dx, j + *dy);
            if x < 0 || y < 0 || x >= self.rows || y >= self.cols {
                continue;
            }

            let other_tile_index = self.get_index(x, y);
            let other = &self.tiles[other_tile_index];

            count -= match (other.has_mine, other.is_hidden, other.is_marked) {
                (true, false, _) => 1,
                (true, true, true) => 1,
                (false, true, true) => 10000000, // wrong marking
                _ => 0,
            }
        }

        count == 0
    }

    fn get_tile_size(&self) -> (f32, f32) {
        let tile_width = (screen_width() - SCREEN_MARGIN * 2.0) / self.cols as f32;
        let tile_height =
            (screen_height() - SCREEN_MARGIN * 2.0 - SCREEN_BOTTOM_MARGIN) / self.rows as f32;

        let min = tile_width.min(tile_height);
        (min, min)
    }

    fn get_index(&self, i: i32, j: i32) -> usize {
        (i * self.cols + j) as usize
    }
}
