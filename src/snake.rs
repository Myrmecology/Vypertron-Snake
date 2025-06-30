use macroquad::prelude::*;
use crate::grid::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, get_offset};
use crate::themes::Theme;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Segment {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
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
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.handle_input();

        self.move_timer += delta_time;
        if self.move_timer >= self.move_delay {
            self.move_timer = 0.0;
            self.move_snake();
        }
    }

    fn move_snake(&mut self) {
        let mut new_head = self.body[0];

        match self.dir {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.body.insert(0, new_head);

        if !self.grow_tail {
            self.body.pop();
        } else {
            self.grow_tail = false;
        }
    }

    pub fn draw(&self, theme: &Theme) {
        let offset = get_offset();

        for (i, segment) in self.body.iter().enumerate() {
            let color = if i == 0 { 
                theme.snake_head 
            } else { 
                theme.snake_body 
            };

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

    fn handle_input(&mut self) {
        let new_dir = self.get_new_direction();
        if let Some(dir) = new_dir {
            self.dir = dir;
        }
    }

    fn get_new_direction(&self) -> Option<Direction> {
        if is_key_pressed(KeyCode::Up) && self.dir != Direction::Down {
            Some(Direction::Up)
        } else if is_key_pressed(KeyCode::Down) && self.dir != Direction::Up {
            Some(Direction::Down)
        } else if is_key_pressed(KeyCode::Left) && self.dir != Direction::Right {
            Some(Direction::Left)
        } else if is_key_pressed(KeyCode::Right) && self.dir != Direction::Left {
            Some(Direction::Right)
        } else {
            None
        }
    }

    pub fn is_dead(&self) -> bool {
        let head = self.head();

        // Check wall collision
        if head.x < 0 || head.x >= GRID_WIDTH || head.y < 0 || head.y >= GRID_HEIGHT {
            return true;
        }

        // Check self collision - skip the head itself
        self.body.iter().skip(1).any(|&segment| segment == head)
    }

    pub fn is_at(&self, position: Segment) -> bool {
        self.body.contains(&position)
    }

    pub fn head(&self) -> Segment {
        self.body[0]
    }

    pub fn position(&self) -> Segment {
        self.head()
    }

    // Additional utility methods
    pub fn length(&self) -> usize {
        self.body.len()
    }

    pub fn reset(&mut self) {
        let start_x = GRID_WIDTH / 2;
        let start_y = GRID_HEIGHT / 2;
        
        self.body.clear();
        self.body.push(Segment { x: start_x, y: start_y });
        self.dir = Direction::Right;
        self.grow_tail = false;
        self.move_timer = 0.0;
    }
}









