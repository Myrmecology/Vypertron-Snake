use macroquad::prelude::*;
use crate::grid::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, get_offset};
use crate::themes::Theme;

#[derive(Clone, Copy, PartialEq)]
pub struct Segment {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<Segment>,
    pub dir: Direction,
    pub grow_tail: bool,
    pub move_timer: f32,
    pub move_delay: f32,
    pub position: Segment,
}

impl Snake {
    pub fn new() -> Self {
        let start_x = GRID_WIDTH / 2;
        let start_y = GRID_HEIGHT / 2;

        Self {
            body: vec![Segment { x: start_x, y: start_y }],
            dir: Direction::Right,
            grow_tail: false,
            move_timer: 0.0,
            move_delay: 0.15,
            position: Segment { x: start_x, y: start_y },
        }
    }

    pub fn update(&mut self) {
        self.handle_input();

        self.move_timer += get_frame_time();
        if self.move_timer >= self.move_delay {
            self.move_timer = 0.0;

            let mut new_head = self.body[0];

            match self.dir {
                Direction::Up => new_head.y -= 1,
                Direction::Down => new_head.y += 1,
                Direction::Left => new_head.x -= 1,
                Direction::Right => new_head.x += 1,
            }

            self.position = new_head;

            self.body.insert(0, new_head);

            if !self.grow_tail {
                self.body.pop();
            } else {
                self.grow_tail = false;
            }
        }
    }

    pub fn draw(&self, theme: &Theme) {
        let offset = get_offset();

        for (i, segment) in self.body.iter().enumerate() {
            let color = if i == 0 { theme.snake_head } else { theme.snake_body };

            draw_rectangle(
                offset.x + segment.x as f32 * CELL_SIZE,
                offset.y + segment.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }

    pub fn grow(&mut self) {
        self.grow_tail = true;
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) && self.dir != Direction::Down {
            self.dir = Direction::Up;
        } else if is_key_pressed(KeyCode::Down) && self.dir != Direction::Up {
            self.dir = Direction::Down;
        } else if is_key_pressed(KeyCode::Left) && self.dir != Direction::Right {
            self.dir = Direction::Left;
        } else if is_key_pressed(KeyCode::Right) && self.dir != Direction::Left {
            self.dir = Direction::Right;
        }
    }

    pub fn is_dead(&self) -> bool {
        let head = self.body[0];

        // Check wall collision
        if head.x < 0 || head.x >= GRID_WIDTH || head.y < 0 || head.y >= GRID_HEIGHT {
            return true;
        }

        // Check self collision
        self.body[1..].contains(&head)
    }

    pub fn is_at(&self, segment: Segment) -> bool {
        self.body.contains(&segment)
    }
}





