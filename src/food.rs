use macroquad::prelude::*;
use crate::grid::{CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, get_offset};
use crate::snake::Snake;

pub struct Food {
    pub position: IVec2,
}

impl Food {
    pub fn new(snake: &Snake) -> Self {
        let mut rng = ::rand::thread_rng();
        let mut pos;

        // Keep retrying until it spawns in a free cell
        loop {
            pos = IVec2::new(
                ::rand::Rng::gen_range(&mut rng, 0..GRID_WIDTH),
                ::rand::Rng::gen_range(&mut rng, 0..GRID_HEIGHT),
            );

            if !snake.is_at(pos) {
                break;
            }
        }

        Self { position: pos }
    }

    pub fn draw(&self) {
        let offset = get_offset();
        let px = self.position.x as f32 * CELL_SIZE + offset.x;
        let py = self.position.y as f32 * CELL_SIZE + offset.y;

        draw_rectangle(px, py, CELL_SIZE, CELL_SIZE, RED);
        draw_rectangle_lines(px, py, CELL_SIZE, CELL_SIZE, 1.5, ORANGE);
    }
}
