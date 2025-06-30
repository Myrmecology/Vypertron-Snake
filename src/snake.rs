use macroquad::prelude::*;
use crate::grid::{CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, get_offset};

pub struct Snake {
    pub position: IVec2,
    pub direction: IVec2,
    pub move_timer: f32,
    pub move_delay: f32,
    pub body: Vec<IVec2>,
    pub grow_next_move: bool,
    pub is_dead: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            position: IVec2::new(GRID_WIDTH / 2, GRID_HEIGHT / 2),
            direction: IVec2::new(1, 0),
            move_timer: 0.0,
            move_delay: 0.15,
            body: vec![],
            grow_next_move: false,
            is_dead: false,
        }
    }

    pub fn update(&mut self) {
        if self.is_dead {
            return;
        }

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

            // Insert head position into body
            self.body.insert(0, self.position);

            if !self.grow_next_move && !self.body.is_empty() {
                self.body.pop();
            } else {
                self.grow_next_move = false;
            }

            let new_pos = self.position + self.direction;

            // Check bounds
            if new_pos.x < 0 || new_pos.x >= GRID_WIDTH || new_pos.y < 0 || new_pos.y >= GRID_HEIGHT {
                self.is_dead = true;
                return;
            }

            // Check self-collision
            if self.body.contains(&new_pos) {
                self.is_dead = true;
                return;
            }

            self.position = new_pos;
        }
    }

    pub fn grow(&mut self) {
        self.grow_next_move = true;
    }

    pub fn draw(&self) {
        let offset = get_offset();

        // Body
        for (i, segment) in self.body.iter().enumerate() {
            let px = segment.x as f32 * CELL_SIZE + offset.x;
            let py = segment.y as f32 * CELL_SIZE + offset.y;
            let shade = 0.5 + (i as f32 / self.body.len().max(1) as f32) * 0.5;
            draw_rectangle(px, py, CELL_SIZE, CELL_SIZE, Color::new(0.0, shade, 0.0, 1.0));
        }

        // Head
        let hx = self.position.x as f32 * CELL_SIZE + offset.x;
        let hy = self.position.y as f32 * CELL_SIZE + offset.y;
        draw_rectangle(hx, hy, CELL_SIZE, CELL_SIZE, GREEN);
        draw_rectangle_lines(hx, hy, CELL_SIZE, CELL_SIZE, 2.0, WHITE);
    }

    pub fn is_at(&self, pos: IVec2) -> bool {
        self.position == pos || self.body.contains(&pos)
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
}



