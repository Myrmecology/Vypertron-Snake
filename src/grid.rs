use macroquad::prelude::*;

pub const GRID_WIDTH: i32 = 40;
pub const GRID_HEIGHT: i32 = 30;
pub const CELL_SIZE: f32 = 20.0;

pub fn get_offset() -> Vec2 {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let grid_pixel_width = GRID_WIDTH as f32 * CELL_SIZE;
    let grid_pixel_height = GRID_HEIGHT as f32 * CELL_SIZE;

    let offset_x = (screen_width - grid_pixel_width) / 2.0;
    let offset_y = (screen_height - grid_pixel_height) / 2.0 + 40.0; // Added offset for UI elements

    vec2(offset_x, offset_y)
}

pub fn draw_grid(color: Color) {
    let offset = get_offset();

    // Draw grid lines with the specified color
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

    // Draw border around the grid for better visibility
    draw_rectangle_lines(
        offset.x - 2.0,
        offset.y - 2.0,
        GRID_WIDTH as f32 * CELL_SIZE + 4.0,
        GRID_HEIGHT as f32 * CELL_SIZE + 4.0,
        2.0,
        color,
    );
}

pub fn is_within_grid(x: i32, y: i32) -> bool {
    x >= 0 && x < GRID_WIDTH && y >= 0 && y < GRID_HEIGHT
}

// Optional: Add this helper function for dynamic sizing based on screen
pub fn get_grid_info() -> String {
    format!("Grid: {}x{} ({}x{}px)", GRID_WIDTH, GRID_HEIGHT, 
            GRID_WIDTH as f32 * CELL_SIZE, GRID_HEIGHT as f32 * CELL_SIZE)
}





