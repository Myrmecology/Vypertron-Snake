use macroquad::prelude::*;
use crate::grid::{CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, get_offset};

pub struct Snake {
    pub position: IVec2,
    pub direction: IVec2,
    pub move_timer: f32,
    pub move_delay: f32,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            position: IVec2::new(GRID_WIDTH / 2, GRID_HEIGHT / 2),
            direction: IVec2::new(1, 0),
            move_timer: 0.0,
            move_delay: 0.15,
        }
    }

    pub fn update(&mut self) {
        // Handle input
        if is_key_pressed(KeyCode::Right) && self.direction != IVec2::new(-1, 0) {
            self.direction = IVec2::new(1, 0);
        }
        if is_key_pressed(KeyCode::Left) && self.direction != IVec2::new(1, 0) {
            self.direction = IVec2::new(-1, 0);
        }
        if is_key_pressed(KeyCode::Up) && self.direction != IVec2::new(0, 1) {
            self.direction = IVec2::new(0, -1);
        }
        if is_key_pressed(KeyCode::Down) && self.direction != IVec2::new(0, -1) {
            self.direction = IVec2::new(0, 1);
        }

        self.move_timer += get_frame_time();
        if self.move_timer >= self.move_delay {
            self.move_timer = 0.0;
            let new_pos = self.position + self.direction;

            // Prevent leaving the grid bounds
            if new_pos.x >= 0 && new_pos.x < GRID_WIDTH &&
               new_pos.y >= 0 && new_pos.y < GRID_HEIGHT {
                self.position = new_pos;
            }
        }
    }

    pub fn draw(&self) {
        let offset = get_offset();
        let px = self.position.x as f32 * CELL_SIZE + offset.x;
        let py = self.position.y as f32 * CELL_SIZE + offset.y;

        draw_rectangle(px, py, CELL_SIZE, CELL_SIZE, GREEN);
        draw_rectangle_lines(px, py, CELL_SIZE, CELL_SIZE, 2.0, WHITE);
    }
}

