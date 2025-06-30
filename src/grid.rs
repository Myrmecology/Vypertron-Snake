use macroquad::prelude::*;
use crate::themes::Theme;

// Grid size constants
pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;
pub const CELL_SIZE: f32 = 32.0;

pub fn get_offset() -> Vec2 {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let grid_w = GRID_WIDTH as f32 * CELL_SIZE;
    let grid_h = GRID_HEIGHT as f32 * CELL_SIZE;

    Vec2::new(
        (screen_w - grid_w) / 2.0,
        (screen_h - grid_h) / 2.0,
    )
}

pub fn draw_grid(color: Color) {
    let offset = get_offset();

    for x in 0..=GRID_WIDTH {
        draw_line(
            offset.x + x as f32 * CELL_SIZE,
            offset.y,
            offset.x + x as f32 * CELL_SIZE,
            offset.y + GRID_HEIGHT as f32 * CELL_SIZE,
            1.0,
            color,
        );
    }

    for y in 0..=GRID_HEIGHT {
        draw_line(
            offset.x,
            offset.y + y as f32 * CELL_SIZE,
            offset.x + GRID_WIDTH as f32 * CELL_SIZE,
            offset.y + y as f32 * CELL_SIZE,
            1.0,
            color,
        );
    }
}


