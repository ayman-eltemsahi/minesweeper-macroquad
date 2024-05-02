use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};

pub struct Mouse {
    left: bool,
    right: bool,

    left_key_down: bool,
    left_key_up: bool,
    right_key_down: bool,
    right_key_up: bool,

    left_key_down_pos: (f32, f32),
    left_key_up_pos: (f32, f32),
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            left: false,
            right: false,
            left_key_down: false,
            left_key_up: false,
            right_key_down: false,
            right_key_up: false,
            left_key_down_pos: (0.0, 0.0),
            left_key_up_pos: (0.0, 0.0),
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

        let pos = mouse_position();
        if self.left_key_down {
            self.left_key_down_pos = pos;
        }

        if self.left_key_up {
            self.left_key_up_pos = pos;
        }
    }

    #[allow(dead_code)]
    pub fn is_left_key_down(&self) -> bool {
        self.left_key_down
    }

    #[allow(dead_code)]
    pub fn is_left_key_up(&self) -> bool {
        self.left_key_up
    }

    pub fn is_left_key_up_same_pos(&self) -> Option<(f32, f32)> {
        if self.left_key_up && self.left_key_down_pos == self.left_key_up_pos {
            Some(self.left_key_up_pos)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_left_key_down_pos(&self) -> (f32, f32) {
        self.left_key_down_pos
    }

    #[allow(dead_code)]
    pub fn get_left_key_up_pos(&self) -> (f32, f32) {
        self.left_key_up_pos
    }
}
