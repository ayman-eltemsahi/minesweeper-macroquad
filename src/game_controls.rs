use macroquad::{
    math::vec2,
    ui::{root_ui, widgets},
    window::{screen_height, screen_width},
};

use crate::{config::Config, vector2::Vector2};

pub const WIDTH: f32 = 190.0;
pub const HEIGHT: f32 = 30.0;
pub const PADDING: f32 = 10.0;

type ButtonLabel = String;
type ButtonIndex = usize;

#[derive(Debug)]
pub struct GameControls {
    buttons: Vec<String>,
}

impl GameControls {
    pub fn new(config: &Config) -> Self {
        let buttons = config
            .levels
            .iter()
            .map(|level| level.name.clone())
            .collect();

        GameControls { buttons }
    }

    pub fn draw(&self) {
        let full_height = HEIGHT + PADDING;
        let (top_margin, left_margin) = self.calculate_margins();

        let mut ui = root_ui();
        self.buttons.iter().enumerate().for_each(|(i, button)| {
            let y = top_margin + (i as f32) * full_height;
            widgets::Button::new(button.clone())
                .size(vec2(WIDTH, HEIGHT))
                .position(vec2(left_margin, y))
                .ui(&mut ui);
        });
    }

    pub fn handle_input(&self, pos: Vector2<f32>) -> Option<(ButtonIndex, ButtonLabel)> {
        let full_height = HEIGHT + PADDING;
        let (top_margin, left_margin) = self.calculate_margins();

        self.buttons
            .iter()
            .enumerate()
            .find(|(i, _)| {
                let x = left_margin;
                let y = top_margin + (*i as f32) * full_height;

                GameControls::intersects(x, y, pos)
            })
            .map(|(index, button)| (index, button.clone()))
    }

    fn calculate_margins(&self) -> (f32, f32) {
        let full_height = HEIGHT + PADDING;
        let all_buttons_height = (self.buttons.len() as f32) * full_height;
        let top_margin = (screen_height() - all_buttons_height) / 2.0;
        let left_margin = (screen_width() - WIDTH) / 2.0;
        (top_margin, left_margin)
    }

    fn intersects(x: f32, y: f32, pos: Vector2<f32>) -> bool {
        pos.x >= x && pos.x <= x + WIDTH && pos.y >= y && pos.y <= y + HEIGHT
    }
}
