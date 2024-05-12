use core::fmt;

use macroquad::{
    math::vec2,
    ui::{root_ui, widgets},
    window::{screen_height, screen_width},
};

pub const WIDTH: f32 = 190.0;
pub const HEIGHT: f32 = 30.0;
pub const PADDING: f32 = 10.0;

#[derive(Debug, Clone)]
pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

impl fmt::Display for GameLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct GameControls {
    buttons: Vec<GameLevel>,
}

impl GameControls {
    pub fn new() -> GameControls {
        let buttons = vec![
            GameLevel::Beginner,
            GameLevel::Intermediate,
            GameLevel::Expert,
        ];

        GameControls { buttons }
    }

    pub fn draw(&self) {
        let full_height = HEIGHT + PADDING;
        let (top_margin, left_margin) = self.get_margins();

        let mut ui = root_ui();
        self.buttons.iter().enumerate().for_each(|(i, button)| {
            let y = top_margin + (i as f32) * full_height;
            widgets::Button::new(button.to_string())
                .size(vec2(WIDTH, HEIGHT))
                .position(vec2(left_margin, y))
                .ui(&mut ui);
        });
    }

    pub fn handle_input(&self, pos: (f32, f32)) -> Option<GameLevel> {
        let full_height = HEIGHT + PADDING;
        let (top_margin, left_margin) = self.get_margins();

        self.buttons
            .iter()
            .enumerate()
            .find(|(i, _)| {
                let x = left_margin;
                let y = top_margin + (*i as f32) * full_height;

                GameControls::intersects(x, y, pos)
            })
            .map(|(_, button)| button.clone())
    }

    fn get_margins(&self) -> (f32, f32) {
        let full_height = HEIGHT + PADDING;
        let all_buttons_height = (self.buttons.len() as f32) * full_height;
        let top_margin = (screen_height() - all_buttons_height) / 2.0;
        let left_margin = (screen_width() - WIDTH) / 2.0;
        (top_margin, left_margin)
    }

    fn intersects(x: f32, y: f32, pos: (f32, f32)) -> bool {
        pos.0 >= x && pos.0 <= x + WIDTH && pos.1 >= y && pos.1 <= y + HEIGHT
    }
}
