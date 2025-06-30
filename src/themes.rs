use macroquad::prelude::*;

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
            snake_head: SKYBLUE,
            snake_body: BLUE,
            food: ORANGE,
        },
        3 => Theme {
            background: Color::new(0.1, 0.05, 0.1, 1.0),
            grid: Color::new(0.2, 0.1, 0.2, 1.0),
            snake_head: PINK,
            snake_body: MAGENTA,
            food: YELLOW,
        },
        4 => Theme {
            background: DARKGREEN,
            grid: GREEN,
            snake_head: YELLOW,
            snake_body: LIME,
            food: RED,
        },
        5 => Theme {
            background: Color::new(0.2, 0.0, 0.3, 1.0),
            grid: VIOLET,
            snake_head: GOLD,
            snake_body: ORANGE,
            food: SKYBLUE,
        },
        6 => Theme {
            background: Color::new(0.0, 0.05, 0.1, 1.0),
            grid: Color::new(0.1, 0.1, 0.3, 1.0),
            snake_head: Color::new(0.3, 1.0, 0.9, 1.0),
            snake_body: Color::new(0.2, 0.6, 0.7, 1.0),
            food: ORANGE,
        },
        7 => Theme {
            background: DARKGRAY,
            grid: WHITE,
            snake_head: RED,
            snake_body: Color::new(0.5, 0.0, 0.0, 1.0),
            food: GOLD,
        },
        8 => Theme {
            background: Color::new(0.2, 0.0, 0.3, 1.0),
            grid: Color::new(0.5, 0.0, 0.7, 1.0),
            snake_head: MAGENTA,
            snake_body: PINK,
            food: GREEN,
        },
        9 => Theme {
            background: Color::new(0.0, 0.0, 0.3, 1.0),
            grid: SKYBLUE,
            snake_head: YELLOW,
            snake_body: WHITE,
            food: RED,
        },
        10 => Theme {
            background: BLACK,
            grid: WHITE,
            snake_head: LIME,
            snake_body: GREEN,
            food: RED,
        },
        _ => get_theme(10),
    }
}


