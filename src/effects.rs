use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};
use crate::grid::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, get_offset};
use crate::snake::Segment;

use lazy_static::lazy_static;
use std::sync::Mutex;

const SNAKE_COUNT: usize = 5;
const MAX_LENGTH: usize = 25;

#[derive(Clone)]
pub struct MovingSnake {
    pub body: Vec<Segment>,
    pub color: Color,
    pub direction: Direction,
    pub timer: f32,
    pub delay: f32,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl MovingSnake {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let start_x = rng.gen_range(0..GRID_WIDTH as i32);
        let start_y = rng.gen_range(0..GRID_HEIGHT as i32);

        let color = Color::new(
            rng.gen_range(0.5..1.0),
            rng.gen_range(0.5..1.0),
            rng.gen_range(0.5..1.0),
            1.0,
        );

        let dir = match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        };

        Self {
            body: vec![Segment { x: start_x, y: start_y }],
            color,
            direction: dir,
            timer: 0.0,
            delay: rng.gen_range(0.08..0.15),
        }
    }

    pub fn update(&mut self) {
        self.timer += get_frame_time();
        if self.timer < self.delay {
            return;
        }
        self.timer = 0.0;

        let mut new_head = self.body[0];
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Wrap around
        if new_head.x < 0 {
            new_head.x = GRID_WIDTH as i32 - 1;
        } else if new_head.x >= GRID_WIDTH as i32 {
            new_head.x = 0;
        }

        if new_head.y < 0 {
            new_head.y = GRID_HEIGHT as i32 - 1;
        } else if new_head.y >= GRID_HEIGHT as i32 {
            new_head.y = 0;
        }

        self.body.insert(0, new_head);
        if self.body.len() > MAX_LENGTH {
            self.body.pop();
        }
    }

    pub fn draw(&self) {
        let offset = get_offset();
        for segment in &self.body {
            draw_rectangle(
                offset.x + segment.x as f32 * CELL_SIZE,
                offset.y + segment.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                self.color,
            );
        }
    }
}

lazy_static! {
    pub static ref MOVING_SNAKES: Mutex<Vec<MovingSnake>> = Mutex::new(Vec::new());
}

pub fn update_moving_snakes() {
    let mut snakes = MOVING_SNAKES.lock().unwrap();

    if snakes.len() < SNAKE_COUNT {
        snakes.push(MovingSnake::new());
    }

    for snake in snakes.iter_mut() {
        snake.update();
    }
}

pub fn draw_moving_snakes() {
    let snakes = MOVING_SNAKES.lock().unwrap();
    for snake in snakes.iter() {
        snake.draw();
    }
}










