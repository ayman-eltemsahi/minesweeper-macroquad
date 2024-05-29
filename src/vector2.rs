use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    pub fn add(&self, other: Vector2<T>) -> Self
    where
        T: Add<T, Output = T> + Copy,
    {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_val(&self, val: T) -> Self
    where
        T: Add<T, Output = T> + Copy,
    {
        Vector2 {
            x: self.x + val,
            y: self.y + val,
        }
    }

    pub fn sub(&self, other: Vector2<T>) -> Self
    where
        T: Sub<T, Output = T> + Copy,
    {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(&self, val: T) -> Self
    where
        T: Mul<T, Output = T> + Copy,
    {
        Vector2 {
            x: self.x * val,
            y: self.y * val,
        }
    }

    pub fn div(&self, other: Vector2<T>) -> Self
    where
        T: Div<T, Output = T> + Copy,
    {
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    pub fn min_component(&self) -> T
    where
        T: PartialOrd + Copy,
    {
        if self.x < self.y {
            self.x
        } else {
            self.y
        }
    }
}

impl From<Vector2<i32>> for Vector2<f32> {
    fn from(v: Vector2<i32>) -> Self {
        Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

impl From<Vector2<f32>> for Vector2<i32> {
    fn from(v: Vector2<f32>) -> Self {
        Vector2 {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}
