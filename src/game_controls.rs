use crate::button::Button;

pub const TOP_MARGIN: f32 = 100.0;
pub const LEFT_MARGIN: f32 = 190.0;
pub const PADDING: f32 = 10.0;

pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

pub struct GameControls {
    beginner_button: Button,
    intermediate_button: Button,
    expert_button: Button,
}

impl GameControls {
    pub fn new() -> GameControls {
        let full_height = Button::get_height() + PADDING;

        GameControls {
            beginner_button: Button::new("Beginner", LEFT_MARGIN, TOP_MARGIN),
            intermediate_button: Button::new("Intermediate", LEFT_MARGIN, TOP_MARGIN + full_height),
            expert_button: Button::new("Expert", LEFT_MARGIN, TOP_MARGIN + 2.0 * full_height),
        }
    }

    pub fn draw(&self) {
        self.beginner_button.draw();
        self.intermediate_button.draw();
        self.expert_button.draw();
    }

    pub fn handle_input(&self, pos: (f32, f32)) -> Option<GameLevel> {
        if self.beginner_button.intersects(pos) {
            Some(GameLevel::Beginner)
        } else if self.intermediate_button.intersects(pos) {
            Some(GameLevel::Intermediate)
        } else if self.expert_button.intersects(pos) {
            Some(GameLevel::Expert)
        } else {
            None
        }
    }
}
