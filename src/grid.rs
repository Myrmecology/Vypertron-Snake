use macroquad::prelude::*;

pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;
pub const CELL_SIZE: f32 = 40.0;



/// Offsets to center the grid in the window
pub fn get_offset() -> Vec2 {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let grid_w = GRID_WIDTH as f32 * CELL_SIZE;
    let grid_h = GRID_HEIGHT as f32 * CELL_SIZE;

    let offset_x = (screen_w - grid_w) / 2.0;
    let offset_y = (screen_h - grid_h) / 2.0;

    vec2(offset_x, offset_y)
}

pub fn draw_grid() {
    let offset = get_offset();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let px = x as f32 * CELL_SIZE + offset.x;
            let py = y as f32 * CELL_SIZE + offset.y;

            draw_rectangle_lines(px, py, CELL_SIZE, CELL_SIZE, 1.0, DARKGRAY);
        }
    }
}

