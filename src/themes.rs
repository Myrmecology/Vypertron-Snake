use macroquad::prelude::*;

pub struct Theme {
    pub snake_head: Color,
    pub snake_body: Color,
    pub food: Color,
    pub grid: Color,
    pub background: Color,
    pub ui_text: Color,
}

pub fn get_theme(level: usize) -> Theme {
    match level % 10 {
        1 => Theme {
            // Classic green snake theme
            snake_head: Color::new(0.0, 1.0, 0.0, 1.0),
            snake_body: Color::new(0.0, 0.7, 0.0, 1.0),
            food: Color::new(1.0, 0.2, 0.2, 1.0),
            grid: Color::new(0.2, 0.2, 0.2, 1.0),
            background: Color::new(0.05, 0.05, 0.05, 1.0),
            ui_text: Color::new(0.0, 1.0, 0.0, 1.0),
        },
        2 => Theme {
            // Sunset orange theme
            snake_head: Color::new(1.0, 0.6, 0.0, 1.0),
            snake_body: Color::new(0.8, 0.4, 0.0, 1.0),
            food: Color::new(1.0, 1.0, 0.0, 1.0),
            grid: Color::new(0.3, 0.2, 0.1, 1.0),
            background: Color::new(0.15, 0.05, 0.05, 1.0),
            ui_text: Color::new(1.0, 0.8, 0.4, 1.0),
        },
        3 => Theme {
            // Cyberpunk purple theme
            snake_head: Color::new(1.0, 0.0, 1.0, 1.0),
            snake_body: Color::new(0.6, 0.0, 0.8, 1.0),
            food: Color::new(0.0, 1.0, 1.0, 1.0),
            grid: Color::new(0.4, 0.0, 0.4, 1.0),
            background: Color::new(0.1, 0.0, 0.2, 1.0),
            ui_text: Color::new(1.0, 0.4, 1.0, 1.0),
        },
        4 => Theme {
            // Arctic ice theme
            snake_head: Color::new(0.4, 0.8, 1.0, 1.0),
            snake_body: Color::new(0.2, 0.6, 0.9, 1.0),
            food: Color::new(1.0, 0.6, 0.2, 1.0),
            grid: Color::new(0.3, 0.4, 0.5, 1.0),
            background: Color::new(0.05, 0.1, 0.15, 1.0),
            ui_text: Color::new(0.6, 0.9, 1.0, 1.0),
        },
        5 => Theme {
            // Royal gold theme
            snake_head: Color::new(1.0, 0.84, 0.0, 1.0),
            snake_body: Color::new(0.8, 0.64, 0.0, 1.0),
            food: Color::new(0.9, 0.9, 0.9, 1.0),
            grid: Color::new(0.5, 0.4, 0.0, 1.0),
            background: Color::new(0.2, 0.0, 0.1, 1.0),
            ui_text: Color::new(1.0, 0.9, 0.4, 1.0),
        },
        6 => Theme {
            // Neon pink theme
            snake_head: Color::new(1.0, 0.0, 0.5, 1.0),
            snake_body: Color::new(0.8, 0.0, 0.4, 1.0),
            food: Color::new(0.0, 1.0, 0.5, 1.0),
            grid: Color::new(0.3, 0.1, 0.2, 1.0),
            background: Color::new(0.1, 0.0, 0.1, 1.0),
            ui_text: Color::new(1.0, 0.4, 0.7, 1.0),
        },
        7 => Theme {
            // Matrix green theme
            snake_head: Color::new(0.5, 1.0, 0.0, 1.0),
            snake_body: Color::new(0.3, 0.8, 0.0, 1.0),
            food: Color::new(1.0, 0.0, 0.0, 1.0),
            grid: Color::new(0.0, 0.3, 0.0, 1.0),
            background: Color::new(0.0, 0.05, 0.0, 1.0),
            ui_text: Color::new(0.4, 1.0, 0.2, 1.0),
        },
        8 => Theme {
            // Fire and ice theme
            snake_head: Color::new(1.0, 0.2, 0.2, 1.0),
            snake_body: Color::new(0.2, 0.4, 1.0, 1.0),
            food: Color::new(1.0, 1.0, 0.0, 1.0),
            grid: Color::new(0.3, 0.3, 0.3, 1.0),
            background: Color::new(0.05, 0.05, 0.1, 1.0),
            ui_text: Color::new(1.0, 0.6, 0.4, 1.0),
        },
        9 => Theme {
            // Desert sand theme
            snake_head: Color::new(0.96, 0.87, 0.7, 1.0),
            snake_body: Color::new(0.76, 0.6, 0.42, 1.0),
            food: Color::new(0.9, 0.7, 0.0, 1.0),
            grid: Color::new(0.5, 0.4, 0.3, 1.0),
            background: Color::new(0.2, 0.15, 0.1, 1.0),
            ui_text: Color::new(1.0, 0.9, 0.6, 1.0),
        },
        0 => Theme {
            // Monochrome master theme
            snake_head: Color::new(1.0, 1.0, 1.0, 1.0),
            snake_body: Color::new(0.7, 0.7, 0.7, 1.0),
            food: Color::new(1.0, 0.0, 0.0, 1.0),
            grid: Color::new(0.2, 0.2, 0.2, 1.0),
            background: Color::new(0.0, 0.0, 0.0, 1.0),
            ui_text: Color::new(1.0, 1.0, 1.0, 1.0),
        },
        _ => Theme {
            // Fallback theme
            snake_head: Color::new(0.0, 1.0, 0.0, 1.0),
            snake_body: Color::new(0.0, 0.7, 0.0, 1.0),
            food: Color::new(1.0, 0.2, 0.2, 1.0),
            grid: Color::new(0.2, 0.2, 0.2, 1.0),
            background: Color::new(0.05, 0.05, 0.05, 1.0),
            ui_text: Color::new(0.0, 1.0, 0.0, 1.0),
        },
    }
}




