use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};

use crate::snake::{Segment, Snake};
use crate::grid::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, get_offset};
use crate::themes::Theme;

pub struct Food {
    pub position: Segment,
}

impl Food {
    pub fn new(snake: &Snake) -> Self {
        let mut food = Food {
            position: Segment { x: 0, y: 0 },
        };
        food.relocate(snake);
        food
    }

    pub fn relocate(&mut self, snake: &Snake) {
        let mut rng = thread_rng();
        loop {
            let pos = Segment {
                x: rng.gen_range(0..GRID_WIDTH),
                y: rng.gen_range(0..GRID_HEIGHT),
            };
            if !snake.is_at(pos) {
                self.position = pos;
                break;
            }
        }
    }

    pub fn draw(&self, theme: &Theme) {
        let offset = get_offset();
        draw_rectangle(
            offset.x + self.position.x as f32 * CELL_SIZE,
            offset.y + self.position.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            theme.food,
        );
    }
}






