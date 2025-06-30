use macroquad::prelude::*;

#[derive(Clone)]
pub struct Theme {
    pub background: Color,
    pub grid: Color,
    pub snake_head: Color,
    pub snake_body: Color,
    pub food: Color,
}

pub fn get_theme(level: u32) -> Theme {
    match level {
        1 => Theme {
            background: BLACK,
            grid: DARKGRAY,
            snake_head: GREEN,
            snake_body: Color::new(0.0, 0.6, 0.0, 1.0),
            food: RED,
        },
        2 => Theme {
            background: Color::new(0.05, 0.05, 0.1, 1.0),
            grid: Color::new(0.1, 0.1, 0.2, 1.0),
            snake_head: Color::new(0.1, 0.8, 0.9, 1.0),
            snake_body: Color::new(0.1, 0.5, 0.7, 1.0),
            food: ORANGE,
        },
        3 => Theme {
            background: Color::new(0.1, 0.05, 0.1, 1.0),
            grid: Color::new(0.2, 0.1, 0.2, 1.0),
            snake_head: PINK,
            snake_body: Color::new(0.6, 0.3, 0.6, 1.0),
            food: YELLOW,
        },
        4 => Theme {
            background: DARKGREEN,
            grid: GREEN,
            snake_head: GOLD,
            snake_body: YELLOW,
            food: RED,
        },
        5..=10 => Theme {
            background: DARKPURPLE,
            grid: VIOLET,
            snake_head: SKYBLUE,
            snake_body: BLUE,
            food: MAGENTA,
        },
        _ => get_theme(10),
    }
}
