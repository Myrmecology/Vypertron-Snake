use macroquad::prelude::*;
use grid::draw_grid;
use snake::Snake;
use food::Food;
use cpu_snake::CpuSnake;
use effects::draw_moving_snakes;
use level::LevelTracker;
use themes::get_theme;

mod grid;
mod snake;
mod food;
mod cpu_snake;
mod effects;
mod level;
mod themes;

#[macroquad::main("Vypertron-Snake")]
async fn main() {
    let mut snake = Snake::new();
    let mut cpu_snake = CpuSnake::new();
    let mut food = Food::new(&snake);
    let mut level_tracker = LevelTracker::new();
    let mut score = 0;

    loop {
        match level_tracker.in_game {
            false => {
                clear_background(BLACK);
                
                // Draw animated background effects
                draw_moving_snakes();
                
                // Calculate center position for title
                let title = "VYPERTRON SNAKE";
                let title_size = 80.0;
                let title_width = measure_text(title, None, title_size as u16, 1.0).width;
                let title_x = (screen_width() - title_width) / 2.0;
                let title_y = screen_height() / 2.0 - 100.0;
                
                // Draw title with glow effect
                for i in 0..3 {
                    let alpha = 0.3 - (i as f32 * 0.1);
                    let offset = i as f32 * 2.0;
                    draw_text(
                        title, 
                        title_x - offset, 
                        title_y - offset, 
                        title_size, 
                        Color::new(0.0, 1.0, 0.0, alpha)
                    );
                }
                draw_text(title, title_x, title_y, title_size, GREEN);
                
                // Draw start prompt (also centered)
                let prompt = "Press SPACE to start";
                let prompt_size = 32.0;
                let prompt_width = measure_text(prompt, None, prompt_size as u16, 1.0).width;
                let prompt_x = (screen_width() - prompt_width) / 2.0;
                let prompt_y = title_y + 80.0;
                
                // Pulsing effect for prompt
                let pulse = (get_time() * 4.0).sin() * 0.3 + 0.7;
                draw_text(
                    prompt, 
                    prompt_x, 
                    prompt_y, 
                    prompt_size, 
                    Color::new(0.8, 0.8, 0.8, pulse as f32)
                );
                
                // Draw last score if game over
                if score > 0 {
                    let score_text = format!("Last Score: {}", score);
                    let score_width = measure_text(&score_text, None, 24, 1.0).width;
                    let score_x = (screen_width() - score_width) / 2.0;
                    draw_text(&score_text, score_x, prompt_y + 50.0, 24.0, YELLOW);
                }

                if is_key_pressed(KeyCode::Space) {
                    snake = Snake::new();
                    cpu_snake = CpuSnake::new();
                    food = Food::new(&snake);
                    level_tracker.reset();
                    level_tracker.in_game = true;
                    score = 0;
                }
            }
            true => {
                let theme = get_theme(level_tracker.level.try_into().unwrap());
                
                // Clear background with theme color
                clear_background(theme.background);

                // Draw UI elements
                let level_text = format!("LEVEL {}", level_tracker.level);
                let level_width = measure_text(&level_text, None, 36, 1.0).width;
                let level_x = (screen_width() - level_width) / 2.0;
                draw_text(&level_text, level_x, 30.0, 36.0, theme.ui_text);
                
                // Draw score (tail counter)
                let score_text = format!("TAILS: {}", score);
                draw_text(&score_text, 20.0, 30.0, 24.0, theme.ui_text);
                
                // Draw speed indicator
                let speed = 1.0 + (level_tracker.level - 1) as f32 * 0.1;
                let speed_text = format!("SPEED: {:.1}x", speed);
                let speed_width = measure_text(&speed_text, None, 24, 1.0).width;
                draw_text(&speed_text, screen_width() - speed_width - 20.0, 30.0, 24.0, theme.ui_text);

                // Draw grid with theme color
                draw_grid(theme.grid);

                let delta_time = get_frame_time();
                snake.update(delta_time);
                cpu_snake.update(level_tracker.level);

                if snake.is_dead() || cpu_snake.check_collision(&snake) {
                    level_tracker.in_game = false;
                }

                if snake.head() == food.position {
                    snake.grow();
                    food.relocate(&snake);
                    score += 1;
                    
                    // Only advance level every 5 foods
                    if score % 5 == 0 {
                        level_tracker.next_level();
                        cpu_snake = CpuSnake::new();
                    }
                }

                snake.draw(&theme);
                food.draw(&theme);
                cpu_snake.draw();
            }
        }

        next_frame().await;
    }
}













