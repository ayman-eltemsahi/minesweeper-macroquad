use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};

use crate::vector2::Vector2;

pub struct Mouse {
    left: bool,
    right: bool,

    left_key_down: bool,
    left_key_up: bool,
    right_key_down: bool,
    right_key_up: bool,

    left_key_down_pos: Vector2<f32>,
    left_key_up_pos: Vector2<f32>,
    right_key_down_pos: Vector2<f32>,
    right_key_up_pos: Vector2<f32>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            left: false,
            right: false,
            left_key_down: false,
            left_key_up: false,
            right_key_down: false,
            right_key_up: false,
            left_key_down_pos: Vector2::new(0.0, 0.0),
            left_key_up_pos: Vector2::new(0.0, 0.0),
            right_key_down_pos: Vector2::new(0.0, 0.0),
            right_key_up_pos: Vector2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        let left = is_mouse_button_down(MouseButton::Left);
        let right = is_mouse_button_down(MouseButton::Right);

        self.left_key_down = !self.left && left;
        self.left_key_up = self.left && !left;

        self.right_key_down = !self.right && right;
        self.right_key_up = self.right && !right;

        self.left = left;
        self.right = right;

        let (x, y) = mouse_position();
        let pos = Vector2::new(x, y);

        if self.left_key_down {
            self.left_key_down_pos = pos;
        }
        if self.left_key_up {
            self.left_key_up_pos = pos;
        }

        if self.right_key_down {
            self.right_key_down_pos = pos;
        }
        if self.right_key_up {
            self.right_key_up_pos = pos;
        }
    }

    pub fn is_left_key_up_same_pos(&self) -> Option<Vector2<f32>> {
        if self.left_key_up && self.left_key_down_pos == self.left_key_up_pos {
            Some(self.left_key_up_pos)
        } else {
            None
        }
    }

    pub fn is_right_key_up_same_pos(&self) -> Option<Vector2<f32>> {
        if self.right_key_up && self.right_key_down_pos == self.right_key_up_pos {
            Some(self.right_key_up_pos)
        } else {
            None
        }
    }
}
