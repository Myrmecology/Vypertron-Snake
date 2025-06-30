use macroquad::prelude::*;

mod grid;
mod effects;
mod snake;
mod food;

use effects::GhostSnake;
use snake::Snake;
use food::Food;

enum GameState {
    Title,
    Playing,
    GameOver,
}

#[macroquad::main("Vypertron-Snake")]
async fn main() {
    let mut state = GameState::Title;

    let mut ghost_snakes = (0..18)
        .map(|_| GhostSnake::new_random(screen_width(), screen_height()))
        .collect::<Vec<_>>();

    let mut snake = Snake::new();
    let mut food = Food::new(&snake);

    loop {
        clear_background(BLACK);

        match state {
            GameState::Title => {
                for ghost in &mut ghost_snakes {
                    ghost.update();
                    ghost.draw();
                }

                draw_title_screen();

                if is_key_pressed(KeyCode::Enter) {
                    snake = Snake::new();
                    food = Food::new(&snake);
                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
                grid::draw_grid();

                snake.update();
                snake.draw();

                food.draw();

                // Check for collision with food
                if snake.position == food.position {
                    snake.grow();
                    food = Food::new(&snake);
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::GameOver;
                }
            }

            GameState::GameOver => {
                draw_text("Game Over", 100.0, 200.0, 40.0, RED);
                draw_text("Press R to restart", 100.0, 250.0, 28.0, WHITE);

                if is_key_pressed(KeyCode::R) {
                    state = GameState::Title;
                }
            }
        }

        next_frame().await;
    }
}

fn draw_title_screen() {
    let time = get_time() as f32;
    let pulse = (time.sin() * 0.5 + 0.5) * 0.5 + 0.5;

    let title_font_size = 64.0;
    let prompt_font_size = 28.0;

    let title_text = "VYPERTRON-SNAKE";
    let prompt_text = "Press ENTER to start";

    let title_width = measure_text(title_text, None, title_font_size as u16, 1.0).width;
    let prompt_width = measure_text(prompt_text, None, prompt_font_size as u16, 1.0).width;

    let screen_w = screen_width();
    let screen_h = screen_height();

    draw_text(
        title_text,
        (screen_w - title_width) / 2.0,
        screen_h * 0.4,
        title_font_size,
        Color::new(pulse, 1.0 - pulse * 0.3, pulse * 0.1, 1.0),
    );

    draw_text(
        prompt_text,
        (screen_w - prompt_width) / 2.0,
        screen_h * 0.5,
        prompt_font_size,
        GRAY,
    );
}







