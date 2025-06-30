use macroquad::prelude::*;

/// Represents a glowing, moving background snake segment
pub struct GhostSnake {
    pub pos: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub color: Color,
}

impl GhostSnake {
    pub fn new_random(screen_w: f32, screen_h: f32) -> Self {
        let mut rng = ::rand::thread_rng();

        let pos = vec2(
            ::rand::Rng::gen_range(&mut rng, 0.0..screen_w),
            ::rand::Rng::gen_range(&mut rng, 0.0..screen_h),
        );

        let angle = ::rand::Rng::gen_range(&mut rng, 0.0..std::f32::consts::TAU);
        let direction = vec2(angle.cos(), angle.sin());

        let speed = ::rand::Rng::gen_range(&mut rng, 1.0..2.5);

        let color = Color::new(
            ::rand::Rng::gen_range(&mut rng, 0.2..1.0),
            ::rand::Rng::gen_range(&mut rng, 0.2..1.0),
            ::rand::Rng::gen_range(&mut rng, 0.2..1.0),
            0.35,
        );

        Self {
            pos,
            direction,
            speed,
            color,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.direction * self.speed;

        let screen_w = screen_width();
        let screen_h = screen_height();

        if self.pos.x > screen_w { self.pos.x = 0.0; }
        if self.pos.y > screen_h { self.pos.y = 0.0; }
        if self.pos.x < 0.0 { self.pos.x = screen_w; }
        if self.pos.y < 0.0 { self.pos.y = screen_h; }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 6.0, self.color);
    }
}




