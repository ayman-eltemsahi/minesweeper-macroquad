use std::collections::VecDeque;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    game_textures::GameTextures,
    grid::Grid,
    messages::{write_game_over, write_remaining_mines, write_time, write_you_win},
    tile::{Tile, TileState},
    utils::current_time_seconds,
    vector2::Vector2,
};

const NEIGHBORS: &[Vector2<i32>] = &[
    Vector2::new(1, 0),
    Vector2::new(-1, 0),
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(1, 1),
    Vector2::new(-1, -1),
    Vector2::new(1, -1),
    Vector2::new(-1, 1),
];

fn rand_num(from: i32, to: i32, rng: &mut ThreadRng, pred: impl Fn(i32) -> bool) -> usize {
    loop {
        let index = rng.gen_range(from..to);
        if pred(index) {
            return index as usize;
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    NotStarted,
    Playing,
    GameOver,
    GameWon,
}

#[derive(Debug)]
pub struct Game {
    pub dimensions: Vector2<i32>,
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
    pub async fn new(grid: Grid) -> Game {
        let textures = GameTextures::new().await;

        Game {
            dimensions: Vector2::new(0, 0),
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
        self.dimensions = Vector2::new(rows, cols);
        self.start_time = current_time_seconds();
        self.state = GameState::Playing;

        let mut rng = rand::thread_rng();

        self.tiles = vec![Default::default(); (rows * cols) as usize];

        (0..num_of_mines).for_each(|_| {
            let index = rand_num(0, rows * cols, &mut rng, |idx| {
                !self.tiles[idx as usize].has_mine
            });
            self.tiles[index].has_mine = true;
        });

        let initial_mines_count = self.tiles.iter().filter(|tile| tile.has_mine).count() as i32;

        self.initial_mines_count = initial_mines_count;
        self.marked_mines_count = 0;
        self.update_mines_count();
    }

    pub fn end(&mut self, state: GameState) {
        self.state = state;
        self.end_time = current_time_seconds();
    }

    pub fn make_move(&mut self, pos: Vector2<f32>) {
        if self.state != GameState::Playing {
            return;
        }

        let (pos, index) = match self.resolve_tile_position(pos) {
            Some(value) => value,
            None => return,
        };

        let tile = &mut self.tiles[index];
        if tile.state == TileState::Flagged || (tile.state == TileState::Revealed && tile.has_mine)
        {
            return;
        }

        if tile.state == TileState::Hidden {
            tile.state = TileState::Revealed;

            if tile.has_mine {
                println!("Game over!");
                self.end(GameState::GameOver);
            } else {
                self.clear_empty_neighbours(pos);
            }
        } else {
            self.click_on_shown_tile(pos);
        }

        if self.has_won() {
            self.end(GameState::GameWon);
        }
    }

    pub fn has_won(&self) -> bool {
        self.tiles.iter().all(|tile| match tile.has_mine {
            true => tile.state == TileState::Flagged,
            false => tile.state == TileState::Revealed,
        })
    }

    pub fn click_on_shown_tile(&mut self, pos: Vector2<i32>) {
        let index = self.get_index(pos);
        let tile = &mut self.tiles[index];

        if tile.num_mines_around == 0 || !self.is_tile_cleared(pos) {
            return;
        }

        self.clear_empty_neighbours(pos);
    }

    pub fn mark_tile(&mut self, pos: Vector2<f32>) {
        if self.state != GameState::Playing {
            eprintln!("Game is not in playing state");
            return;
        }

        let (_, index) = match self.resolve_tile_position(pos) {
            Some(value) => value,
            None => return,
        };

        let tile = &mut self.tiles[index];
        if tile.state == TileState::Revealed {
            return;
        }

        tile.state = match tile.state {
            TileState::Flagged => TileState::Hidden,
            _ => TileState::Flagged,
        };

        self.marked_mines_count += match tile.state {
            TileState::Flagged => 1,
            _ => -1,
        };

        if self.has_won() {
            self.end(GameState::GameWon);
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    fn resolve_tile_position(&mut self, pos: Vector2<f32>) -> Option<(Vector2<i32>, usize)> {
        let tile_size = self.get_tile_size();
        let transformed = pos.sub(self.grid.body.pos());
        if transformed.x < 0.0 || transformed.y < 0.0 {
            return None;
        }

        let scaled = transformed.flip().scale(1.0 / tile_size);
        let result = Vector2::new(scaled.x as i32, scaled.y as i32);

        if self.within_bounds(result) {
            Some((result, self.get_index(result)))
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

        let margin = self.grid.body.pos();

        for i in 0..self.dimensions.x {
            for j in 0..self.dimensions.y {
                let pos = Vector2::new(i, j);
                let index = self.get_index(pos);
                let flipped_pos: Vector2<f32> = pos.flip().into();

                self.tiles[index].draw(
                    margin.add(flipped_pos.scale(tile_size)),
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

    fn clear_empty_neighbours(&mut self, pos: Vector2<i32>) {
        let mut q: VecDeque<Vector2<i32>> = VecDeque::new();
        q.push_back(pos);

        while let Some(pos) = q.pop_front() {
            for neighbour_diff in NEIGHBORS {
                let new_pos = pos.add(*neighbour_diff);

                if !self.within_bounds(new_pos) {
                    continue;
                }

                let other_tile_index = self.get_index(new_pos);
                let other_tile = &mut self.tiles[other_tile_index];

                if other_tile.has_mine || other_tile.state != TileState::Hidden {
                    continue;
                }

                other_tile.state = TileState::Revealed;

                if other_tile.num_mines_around == 0 {
                    q.push_back(new_pos);
                }
            }
        }
    }

    fn update_mines_count(&mut self) {
        for i in 0..self.dimensions.x {
            for j in 0..self.dimensions.y {
                let pos = Vector2::new(i, j);
                let count = self.count_mines_around(pos);
                let index = self.get_index(pos);
                self.tiles[index].update_num_mines_around(count);
            }
        }
    }

    fn count_mines_around(&self, pos: Vector2<i32>) -> i32 {
        NEIGHBORS
            .iter()
            .map(|neighbour_diff| pos.add(*neighbour_diff))
            .filter(|pos| self.within_bounds(*pos) && self.tiles[self.get_index(*pos)].has_mine)
            .count() as i32
    }

    fn is_tile_cleared(&self, pos: Vector2<i32>) -> bool {
        let index = self.get_index(pos);
        if self.tiles[index].has_mine {
            return false;
        }

        let count: i32 = NEIGHBORS
            .iter()
            .map(|neighbour_diff| pos.add(*neighbour_diff))
            .filter(|pos| self.within_bounds(*pos))
            .map(|pos| {
                let other = &self.tiles[self.get_index(pos)];

                match (other.has_mine, &other.state) {
                    (true, TileState::Flagged) | (true, TileState::Revealed) => 1,
                    (false, TileState::Flagged) => 10000000, // wrong marking
                    _ => 0,
                }
            })
            .sum();

        self.tiles[index].num_mines_around == count
    }

    fn get_tile_size(&self) -> f32 {
        self.grid
            .body
            .screen_size()
            .div(self.dimensions.flip().into())
            .min_component()
    }

    fn within_bounds(&self, coord: Vector2<i32>) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.dimensions.x && coord.y < self.dimensions.y
    }

    fn get_index(&self, pos: Vector2<i32>) -> usize {
        (pos.x as usize * self.dimensions.y as usize) + pos.y as usize
    }
}
