use macroquad::prelude::*;

pub struct Score {
    pub value: u32,
}

impl Score {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn add(&mut self, amount: u32) {
        self.value += amount;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn draw(&self) {
        let text = format!("Score: {}", self.value);
        draw_text(&text, 20.0, 40.0, 32.0, YELLOW);
    }
}
