use macroquad::window::{screen_height, screen_width};

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
    pub fn get_x(&self) -> f32 {
        self.padding_left
    }

    pub fn get_y(&self) -> f32 {
        let h = screen_height();
        return self.padding_top + (self.start / GRID_COLUMNS) * h;
    }

    pub fn w(&self) -> f32 {
        screen_width() - self.padding_left * 2.0
    }

    pub fn h(&self) -> f32 {
        let h = screen_height();
        (self.cols / GRID_COLUMNS * h) - (self.padding_top * 2.0)
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
