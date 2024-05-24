use macroquad::{color::BLACK, text::draw_text, time::get_fps, window::screen_height};

pub struct Diagnostics {
    counter: i64,
    last_fps: i32,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            counter: 0,
            last_fps: 0,
        }
    }

    pub fn on_loop(&mut self) {
        self.counter += 1;
        if self.counter % 50 == 0 {
            self.last_fps = get_fps();
        }

        draw_text(
            &format!("fps: {}", self.last_fps),
            20.0,
            screen_height() - 20.0,
            20.0,
            BLACK,
        );
    }
}
