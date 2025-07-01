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
    pub color_head: Color,
    pub color_body: Color,
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
            color_head: RED,
            color_body: DARKGRAY,
        }
    }

    pub fn new_with_colors(head_color: Color, body_color: Color) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..GRID_WIDTH);
        let y = rng.gen_range(0..GRID_HEIGHT);

        Self {
            body: vec![Segment { x, y }],
            dir: Direction::Left,
            move_timer: 0.0,
            move_delay: 0.25,
            color_head: head_color,
            color_body: body_color,
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
            let color = if i == 0 { self.color_head } else { self.color_body };

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

// New manager struct to handle multiple CPU snakes
pub struct CpuSnakeManager {
    pub snakes: Vec<CpuSnake>,
    current_level: usize,
}

impl CpuSnakeManager {
    pub fn new() -> Self {
        Self {
            snakes: vec![CpuSnake::new()],
            current_level: 1,
        }
    }

    pub fn update(&mut self, level: usize) {
        // Check if we need to add more snakes
        if level != self.current_level {
            self.current_level = level;
            self.adjust_snake_count(level);
        }

        // Update all snakes
        for snake in &mut self.snakes {
            snake.update(level);
        }
    }

    fn adjust_snake_count(&mut self, level: usize) {
        // Calculate how many snakes we should have
        let target_count = self.calculate_snake_count(level);
        
        // Add snakes if needed
        while self.snakes.len() < target_count {
            let snake = match self.snakes.len() {
                1 => CpuSnake::new_with_colors(ORANGE, Color::new(0.5, 0.3, 0.0, 1.0)),
                2 => CpuSnake::new_with_colors(PURPLE, DARKPURPLE),
                3 => CpuSnake::new_with_colors(BLUE, DARKBLUE),
                4 => CpuSnake::new_with_colors(YELLOW, GOLD),
                _ => CpuSnake::new_with_colors(PINK, MAGENTA),
            };
            self.snakes.push(snake);
        }

        // Remove snakes if needed (when level decreases, though this won't happen in normal play)
        while self.snakes.len() > target_count && self.snakes.len() > 1 {
            self.snakes.pop();
        }
    }

    fn calculate_snake_count(&self, level: usize) -> usize {
        match level {
            1..=4 => 1,
            5..=9 => 2,
            10..=14 => 3,
            15..=19 => 4,
            _ => 5, // Cap at 5 CPU snakes for playability
        }
    }

    pub fn draw(&self) {
        for snake in &self.snakes {
            snake.draw();
        }
    }

    pub fn reset(&mut self) {
        self.snakes.clear();
        self.snakes.push(CpuSnake::new());
        self.current_level = 1;
    }
}







