use macroquad::window::{screen_height, screen_width};

use crate::coordinate::Coordinate;

const GRID_COLUMNS: f32 = 12.0;

#[derive(Debug)]
pub struct GridSection {
    pub start: f32,
    pub cols: f32,
    pub padding_top: f32,
    pub padding_left: f32,
}

#[derive(Debug)]
pub struct Grid {
    pub header: GridSection,
    pub body: GridSection,
    pub footer: GridSection,
}

impl GridSection {
    pub fn pos(&self) -> Coordinate {
        let x = self.padding_left;

        let h = screen_height();
        let y = self.padding_top + (self.start / GRID_COLUMNS) * h;

        Coordinate { x, y }
    }

    pub fn screen_size(&self) -> Coordinate {
        let x = screen_width() - self.padding_left * 2.0;

        let h = screen_height();
        let y = (self.cols / GRID_COLUMNS * h) - (self.padding_top * 2.0);

        Coordinate { x, y }
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            header: GridSection {
                start: 0.0,
                cols: 1.0,
                padding_top: 5.0,
                padding_left: 20.0,
            },
            body: GridSection {
                start: 1.0,
                cols: 10.0,
                padding_top: 0.0,
                padding_left: 20.0,
            },
            footer: GridSection {
                start: 11.0,
                cols: 1.0,
                padding_top: 5.0,
                padding_left: 20.0,
            },
        }
    }
}
