#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

pub const fn coord(x: f32, y: f32) -> Coordinate {
    Coordinate { x, y }
}

impl Coordinate {
    pub fn add(&self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_val(&self, val: f32) -> Coordinate {
        Coordinate {
            x: self.x + val,
            y: self.y + val,
        }
    }

    pub fn sub(&self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mult_val(&self, val: f32) -> Coordinate {
        Coordinate {
            x: self.x * val,
            y: self.y * val,
        }
    }

    pub fn div(&self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    pub fn div_val(&self, val: f32) -> Coordinate {
        Coordinate {
            x: self.x / val,
            y: self.y / val,
        }
    }

    pub fn flip(&self) -> Coordinate {
        Coordinate {
            x: self.y,
            y: self.x,
        }
    }

    pub fn min_component(&self) -> f32 {
        self.x.min(self.y)
    }
}
