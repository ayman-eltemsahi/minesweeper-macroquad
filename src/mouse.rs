use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};

use crate::vector2::Vector2;

pub fn is_mouse_left_btn_pressed() -> Option<Vector2<f32>> {
    is_mouse_pressed(MouseButton::Left)
}

pub fn is_mouse_right_btn_pressed() -> Option<Vector2<f32>> {
    is_mouse_pressed(MouseButton::Right)
}

fn is_mouse_pressed(button: MouseButton) -> Option<Vector2<f32>> {
    if is_mouse_button_pressed(button) {
        let (x, y) = mouse_position();
        Some(Vector2::new(x, y))
    } else {
        None
    }
}
