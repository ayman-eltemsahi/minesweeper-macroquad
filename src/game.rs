use std::collections::VecDeque;

use rand::Rng;

use crate::{
    game_state::GameState,
    game_textures::GameTextures,
    grid::Grid,
    messages::{write_game_over, write_remaining_mines, write_time, write_you_win},
    tile::Tile,
    utils::current_time_seconds,
};

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

    state: GameState,

    grid: Grid,

    textures: GameTextures,
}

impl Game {
    pub async fn random_game(grid: Grid) -> Game {
        let textures = GameTextures::new().await;

        Game {
            rows: 0,
            cols: 0,
            tile_size: 0.0,
            tiles: Vec::new(),
            start_time: 0,
            end_time: 0,
            initial_mines_count: 0,
            marked_mines_count: 0,
            grid,

            state: GameState::NotStarted,

            textures,
        }
    }

    pub fn start(&mut self, rows: i32, cols: i32, num_of_mines: i32) {
        self.rows = rows;
        self.cols = cols;
        self.start_time = current_time_seconds();
        self.state = GameState::Playing;

        let mut rng = rand::thread_rng();

        let mut tiles = Vec::new();
        for _ in 0..rows * cols {
            tiles.push(Tile::new(false));
        }

        for _ in 0..num_of_mines {
            let mut index = rng.gen_range(0..(rows * cols) as usize);
            while tiles[index].has_mine {
                index = rng.gen_range(0..(rows * cols) as usize);
            }

            tiles[index].has_mine = true;
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
        let tile_size = self.get_tile_size();
        let x = pos.0 - self.grid.body.x();
        let y = pos.1 - self.grid.body.y();
        let j = (x / tile_size) as i32;
        let i = (y / tile_size) as i32;

        if x >= 0.0 && y >= 0.0 && self.within_bounds(i, j) {
            Some((i, j, self.get_index(i, j)))
        } else {
            None
        }
    }

    pub fn draw(&self) {
        if self.state == GameState::NotStarted {
            return;
        }

        self.draw_tiles();

        if self.state != GameState::NotStarted {
            self.write_time();
            self.write_remaining_mines();
        }

        match self.state {
            GameState::GameOver => write_game_over(&self.grid.header),
            GameState::GameWon => write_you_win(&self.grid.header),
            _ => {}
        }
    }

    fn draw_tiles(&self) {
        let tile_size = self.get_tile_size();

        let margin_x = self.grid.body.x();
        let margin_y = self.grid.body.y();

        for i in 0..self.rows {
            for j in 0..self.cols {
                let index = self.get_index(i, j);
                self.tiles[index].draw(
                    margin_x + ((j as f32) * tile_size),
                    margin_y + ((i as f32) * tile_size),
                    tile_size - 1.0,
                    tile_size - 1.0,
                    &self.textures,
                );
            }
        }
    }

    fn write_remaining_mines(&self) {
        write_remaining_mines(
            self.initial_mines_count - self.marked_mines_count,
            &self.grid.header,
        );
    }

    fn write_time(&self) {
        let end_time = match self.state {
            GameState::GameOver | GameState::GameWon => self.end_time,
            _ => current_time_seconds(),
        };

        write_time(self.start_time, end_time, &self.grid.header);
    }

    fn clear_empty_neighbours(&mut self, i: i32, j: i32) {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((i, j));

        while let Some((i, j)) = q.pop_front() {
            for (dx, dy) in NEIGHBORS {
                let (x, y) = (i + *dx, j + *dy);
                if !self.within_bounds(x, y) {
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
        NEIGHBORS
            .iter()
            .map(|(dx, dy)| (i + *dx, j + *dy))
            .filter(|(x, y)| self.within_bounds(*x, *y))
            .filter(|(x, y)| self.tiles[self.get_index(*x, *y)].has_mine)
            .count() as i32
    }

    fn is_tile_cleared(&self, i: i32, j: i32) -> bool {
        let index = self.get_index(i, j);
        if self.tiles[index].has_mine {
            return false;
        }

        let count: i32 = NEIGHBORS
            .iter()
            .map(|(dx, dy)| (i + *dx, j + *dy))
            .filter(|(x, y)| self.within_bounds(*x, *y))
            .map(|(x, y)| {
                let other = &self.tiles[self.get_index(x, y)];

                match (other.has_mine, other.is_hidden, other.is_marked) {
                    (true, false, _) => 1,
                    (true, true, true) => 1,
                    (false, true, true) => 10000000, // wrong marking
                    _ => 0,
                }
            })
            .sum();

        self.tiles[index].num_mines_around == count
    }

    fn get_tile_size(&self) -> f32 {
        let tile_width = (self.grid.body.w()) / self.cols as f32;
        let tile_height = (self.grid.body.h()) / self.rows as f32;

        tile_width.min(tile_height)
    }

    fn within_bounds(&self, i: i32, j: i32) -> bool {
        i >= 0 && j >= 0 && i < self.rows && j < self.cols
    }

    fn get_index(&self, i: i32, j: i32) -> usize {
        (i * self.cols + j) as usize
    }
}
