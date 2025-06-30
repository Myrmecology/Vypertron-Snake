use macroquad::prelude::*;
use crate::grid::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, get_offset};
use crate::snake::{Snake, Segment, Direction};
use ::rand::thread_rng;
use ::rand::prelude::Rng;

pub struct CpuSnake {
    pub body: Vec<Segment>,
    pub dir: Direction,
    pub move_timer: f32,
    pub move_delay: f32,
}

impl CpuSnake {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..GRID_WIDTH);
        let y = rng.gen_range(0..GRID_HEIGHT);

        Self {
            body: vec![Segment { x, y }],
            dir: Direction::Left,
            move_timer: 0.0,
            move_delay: 0.25,
        }
    }

    pub fn update(&mut self, level: usize) {
        self.move_timer += get_frame_time();

        // Increase speed as level increases
        self.move_delay = (0.25 - level as f32 * 0.01).max(0.05);

        if self.move_timer >= self.move_delay {
            self.move_timer = 0.0;

            let mut rng = thread_rng();
            let mut new_dir = self.dir;

            if rng.gen_bool(0.3) {
                new_dir = match rng.gen_range(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                };
            }

            self.dir = new_dir;
            let mut new_head = self.body[0];

            match self.dir {
                Direction::Up => new_head.y -= 1,
                Direction::Down => new_head.y += 1,
                Direction::Left => new_head.x -= 1,
                Direction::Right => new_head.x += 1,
            }

            if new_head.x >= 0 && new_head.x < GRID_WIDTH && new_head.y >= 0 && new_head.y < GRID_HEIGHT {
                self.body.insert(0, new_head);
                self.body.pop();
            }
        }
    }

    pub fn draw(&self) {
        let offset = get_offset();

        for (i, segment) in self.body.iter().enumerate() {
            let color = if i == 0 { RED } else { DARKGRAY };

            draw_rectangle(
                offset.x + segment.x as f32 * CELL_SIZE,
                offset.y + segment.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }

    pub fn check_collision(&self, snake: &Snake) -> bool {
        let cpu_head = self.body[0];
        snake.body.contains(&cpu_head)
    }
}







