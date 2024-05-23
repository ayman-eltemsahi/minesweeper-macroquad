use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Add<T, Output = T> + Copy,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    T: std::cmp::PartialOrd<T>,
{
    pub const fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }

    pub fn add(&self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_val(&self, val: T) -> Vector2<T> {
        Vector2 {
            x: self.x + val,
            y: self.y + val,
        }
    }

    pub fn sub(&self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mult_val(&self, val: T) -> Vector2<T> {
        Vector2 {
            x: self.x * val,
            y: self.y * val,
        }
    }

    pub fn div(&self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    pub fn div_val(&self, val: T) -> Vector2<T> {
        Vector2 {
            x: self.x / val,
            y: self.y / val,
        }
    }

    pub fn flip(&self) -> Vector2<T> {
        Vector2 {
            x: self.y,
            y: self.x,
        }
    }

    pub fn min_component(&self) -> T {
        if self.x < self.y {
            self.x
        } else {
            self.y
        }
    }
}

impl From<Vector2<i32>> for Vector2<f32> {
    fn from(v: Vector2<i32>) -> Vector2<f32> {
        Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}
