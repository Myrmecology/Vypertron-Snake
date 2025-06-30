use macroquad::prelude::*;

pub struct LevelTracker {
    pub level: usize,
    pub score: usize,
    pub score_to_next: usize,
    pub in_game: bool,
}

impl LevelTracker {
    pub fn new() -> Self {
        Self {
            level: 1,
            score: 0,
            score_to_next: 5,
            in_game: false,
        }
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
        if self.score >= self.score_to_next {
            self.level += 1;
            self.score_to_next += 5;
        }
    }

    pub fn next_level(&mut self) {
        self.level += 1;
        self.score = 0;
        self.score_to_next += 5;
    }

    pub fn reset(&mut self) {
        self.level = 1;
        self.score = 0;
        self.score_to_next = 5;
        self.in_game = false;
    }

    pub fn draw(&self) {
        let text = format!("Level: {}  Score: {}", self.level, self.score);
        draw_text(&text, 20.0, 70.0, 28.0, LIGHTGRAY);
    }
}





