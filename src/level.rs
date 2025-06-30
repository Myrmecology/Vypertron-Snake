use macroquad::prelude::*; // Needed for draw_text and LIGHTGRAY

pub struct Level {
    pub number: u32,
    pub speed: f32,
}

impl Level {
    pub fn new() -> Self {
        Self {
            number: 1,
            speed: 0.15, // Base snake movement delay
        }
    }

    pub fn update(&mut self, score: u32) {
        self.number = score / 5 + 1;

        // Each level slightly increases the speed (caps at 0.05)
        self.speed = 0.15_f32.max(0.15 - (self.number - 1) as f32 * 0.01);
    }

    pub fn draw(&self) {
        let text = format!("Level: {}", self.number);
        draw_text(&text, 20.0, 70.0, 28.0, LIGHTGRAY);
    }
}

