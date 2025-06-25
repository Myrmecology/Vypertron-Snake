//! Utility Functions and Helpers for Vypertron-Snake
//! 
//! This module provides essential utility functions, mathematical operations,
//! and helper systems that support all aspects of the game.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use std::collections::HashMap;
use std::f32::consts::PI;
use rand::prelude::*;

// ===============================
// MATHEMATICAL UTILITIES
// ===============================

/// Mathematical utility functions
pub struct MathUtils;

impl MathUtils {
    /// Convert grid coordinates to world position
    pub fn grid_to_world(grid_pos: Vec2, grid_size: f32) -> Vec2 {
        Vec2::new(
            grid_pos.x * grid_size,
            grid_pos.y * grid_size,
        )
    }
    
    /// Convert world position to grid coordinates
    pub fn world_to_grid(world_pos: Vec2, grid_size: f32) -> Vec2 {
        Vec2::new(
            (world_pos.x / grid_size).round(),
            (world_pos.y / grid_size).round(),
        )
    }
    
    /// Check if a grid position is within bounds
    pub fn is_within_bounds(pos: Vec2, grid_width: u32, grid_height: u32) -> bool {
        pos.x >= 0.0 && pos.x < grid_width as f32 &&
        pos.y >= 0.0 && pos.y < grid_height as f32
    }
    
    /// Calculate distance between two grid positions
    pub fn grid_distance(pos1: Vec2, pos2: Vec2) -> f32 {
        ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2)).sqrt()
    }
    
    /// Calculate Manhattan distance (grid-based distance)
    pub fn manhattan_distance(pos1: Vec2, pos2: Vec2) -> f32 {
        (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
    }
    
    /// Normalize an angle to 0-2π range
    pub fn normalize_angle(angle: f32) -> f32 {
        let mut normalized = angle % (2.0 * PI);
        if normalized < 0.0 {
            normalized += 2.0 * PI;
        }
        normalized
    }
    
    /// Linear interpolation between two values
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }
    
    /// Smooth interpolation (ease in/out)
    pub fn smooth_step(t: f32) -> f32 {
        let clamped = t.clamp(0.0, 1.0);
        clamped * clamped * (3.0 - 2.0 * clamped)
    }
    
    /// Bounce easing function
    pub fn bounce_ease_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }
    
    /// Elastic easing function
    pub fn elastic_ease_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let p = 0.3;
            let s = p / 4.0;
            2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * PI) / p).sin() + 1.0
        }
    }
    
    /// Generate random position within grid bounds
    pub fn random_grid_position(grid_width: u32, grid_height: u32, rng: &mut impl Rng) -> Vec2 {
        Vec2::new(
            rng.gen_range(1.0..(grid_width as f32 - 1.0)),
            rng.gen_range(1.0..(grid_height as f32 - 1.0)),
        )
    }
    
    /// Check if two grid positions are adjacent
    pub fn are_adjacent(pos1: Vec2, pos2: Vec2) -> bool {
        let dx = (pos1.x - pos2.x).abs();
        let dy = (pos1.y - pos2.y).abs();
        (dx == 1.0 && dy == 0.0) || (dx == 0.0 && dy == 1.0)
    }
    
    /// Get direction vector from one position to another
    pub fn direction_between(from: Vec2, to: Vec2) -> Vec2 {
        let diff = to - from;
        if diff.length() > 0.0 {
            diff.normalize()
        } else {
            Vec2::ZERO
        }
    }
    
    /// Wrap position around grid boundaries (for teleport mechanics)
    pub fn wrap_position(pos: Vec2, grid_width: u32, grid_height: u32) -> Vec2 {
        Vec2::new(
            if pos.x < 0.0 { grid_width as f32 - 1.0 } 
            else if pos.x >= grid_width as f32 { 0.0 } 
            else { pos.x },
            if pos.y < 0.0 { grid_height as f32 - 1.0 } 
            else if pos.y >= grid_height as f32 { 0.0 } 
            else { pos.y },
        )
    }
}

// ===============================
// COLOR UTILITIES
// ===============================

/// Color manipulation and theme utilities
pub struct ColorUtils;

impl ColorUtils {
    /// Create color from HSV values
    /// FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r_prime, g_prime, b_prime) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        Color::srgb(r_prime + m, g_prime + m, b_prime + m)
    }
    
    /// Interpolate between two colors
    /// FIXED: Updated color access methods for Bevy 0.14
    pub fn lerp_color(color1: Color, color2: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let c1 = color1.to_srgba();
        let c2 = color2.to_srgba();
        
        Color::srgba(
            MathUtils::lerp(c1.red, c2.red, t),
            MathUtils::lerp(c1.green, c2.green, t),
            MathUtils::lerp(c1.blue, c2.blue, t),
            MathUtils::lerp(c1.alpha, c2.alpha, t),
        )
    }
    
    /// Create pulsing color effect
    /// FIXED: Updated color access methods for Bevy 0.14
    pub fn pulse_color(base_color: Color, pulse_intensity: f32, time: f32) -> Color {
        let pulse = (time * 4.0).sin() * 0.5 + 0.5; // 0.0 to 1.0
        let intensity = pulse * pulse_intensity;
        let base = base_color.to_srgba();
        
        Color::srgba(
            (base.red + intensity).min(1.0),
            (base.green + intensity).min(1.0),
            (base.blue + intensity).min(1.0),
            base.alpha,
        )
    }
    
    /// Darken a color by a percentage
    /// FIXED: Updated color access methods for Bevy 0.14
    pub fn darken(color: Color, amount: f32) -> Color {
        let factor = 1.0 - amount.clamp(0.0, 1.0);
        let c = color.to_srgba();
        
        Color::srgba(
            c.red * factor,
            c.green * factor,
            c.blue * factor,
            c.alpha,
        )
    }
    
    /// Brighten a color by a percentage
    /// FIXED: Updated color access methods for Bevy 0.14
    pub fn brighten(color: Color, amount: f32) -> Color {
        let factor = 1.0 + amount.clamp(0.0, 1.0);
        let c = color.to_srgba();
        
        Color::srgba(
            (c.red * factor).min(1.0),
            (c.green * factor).min(1.0),
            (c.blue * factor).min(1.0),
            c.alpha,
        )
    }
    
    /// Get character color by ID
    /// FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    pub fn get_character_color(character_id: u32) -> Color {
        match character_id {
            1 => Color::srgb(0.2, 0.8, 0.2), // Verdant Viper - Green
            2 => Color::srgb(0.1, 0.5, 1.0), // Electric Eel - Blue
            3 => Color::srgb(1.0, 0.2, 0.1), // Crimson Crusher - Red
            4 => Color::srgb(1.0, 0.8, 0.0), // Golden Guardian - Gold
            _ => Color::srgb(0.5, 0.5, 0.5), // Default - Gray
        }
    }
    
    /// Get level theme color palette
    /// FIXED: Changed all Color::rgb to Color::srgb for Bevy 0.14
    /// FIXED: Added missing match arms for Cyber, Shadow, and Cosmic themes
    pub fn get_theme_palette(theme: &LevelTheme) -> ThemeColorPalette {
        match theme {
            LevelTheme::Classic => ThemeColorPalette {
                primary: Color::srgb(0.2, 0.6, 0.2),
                secondary: Color::srgb(0.4, 0.2, 0.1),
                accent: Color::srgb(1.0, 1.0, 0.2),
                background: Color::srgb(0.1, 0.3, 0.1),
            },
            LevelTheme::Digital => ThemeColorPalette {
                primary: Color::srgb(0.0, 0.8, 1.0),
                secondary: Color::srgb(0.2, 0.2, 0.3),
                accent: Color::srgb(1.0, 0.0, 1.0),
                background: Color::srgb(0.05, 0.05, 0.15),
            },
            LevelTheme::Forest => ThemeColorPalette {
                primary: Color::srgb(0.1, 0.5, 0.1),
                secondary: Color::srgb(0.3, 0.15, 0.05),
                accent: Color::srgb(0.8, 0.8, 0.2),
                background: Color::srgb(0.05, 0.2, 0.05),
            },
            LevelTheme::Desert => ThemeColorPalette {
                primary: Color::srgb(0.8, 0.6, 0.3),
                secondary: Color::srgb(0.6, 0.3, 0.1),
                accent: Color::srgb(1.0, 0.8, 0.0),
                background: Color::srgb(0.3, 0.2, 0.1),
            },
            LevelTheme::Ocean => ThemeColorPalette {
                primary: Color::srgb(0.1, 0.3, 0.6),
                secondary: Color::srgb(0.0, 0.2, 0.4),
                accent: Color::srgb(0.0, 0.8, 0.8),
                background: Color::srgb(0.0, 0.1, 0.3),
            },
            LevelTheme::Volcano => ThemeColorPalette {
                primary: Color::srgb(0.8, 0.2, 0.0),
                secondary: Color::srgb(0.4, 0.1, 0.0),
                accent: Color::srgb(1.0, 0.6, 0.0),
                background: Color::srgb(0.2, 0.05, 0.0),
            },
            LevelTheme::Ice => ThemeColorPalette {
                primary: Color::srgb(0.7, 0.9, 1.0),
                secondary: Color::srgb(0.4, 0.6, 0.8),
                accent: Color::srgb(0.9, 0.9, 1.0),
                background: Color::srgb(0.2, 0.3, 0.4),
            },
            LevelTheme::Space => ThemeColorPalette {
                primary: Color::srgb(0.3, 0.3, 0.4),
                secondary: Color::srgb(0.1, 0.1, 0.2),
                accent: Color::srgb(0.8, 0.8, 1.0),
                background: Color::srgb(0.02, 0.02, 0.05),
            },
            LevelTheme::NeonCity => ThemeColorPalette {
                primary: Color::srgb(0.8, 0.0, 0.8),
                secondary: Color::srgb(0.2, 0.0, 0.2),
                accent: Color::srgb(0.0, 1.0, 1.0),
                background: Color::srgb(0.1, 0.0, 0.1),
            },
            LevelTheme::FinalBoss => ThemeColorPalette {
                primary: Color::srgb(0.6, 0.0, 0.0),
                secondary: Color::srgb(0.3, 0.0, 0.0),
                accent: Color::srgb(1.0, 0.0, 0.0),
                background: Color::srgb(0.1, 0.0, 0.0),
            },
            // FIXED: Added missing theme variants
            LevelTheme::Cyber => ThemeColorPalette {
                primary: Color::srgb(0.0, 1.0, 0.5),
                secondary: Color::srgb(0.1, 0.1, 0.1),
                accent: Color::srgb(0.0, 1.0, 1.0),
                background: Color::srgb(0.0, 0.05, 0.0),
            },
            LevelTheme::Shadow => ThemeColorPalette {
                primary: Color::srgb(0.3, 0.3, 0.4),
                secondary: Color::srgb(0.1, 0.1, 0.15),
                accent: Color::srgb(0.6, 0.5, 0.8),
                background: Color::srgb(0.05, 0.05, 0.1),
            },
            LevelTheme::Cosmic => ThemeColorPalette {
                primary: Color::srgb(0.5, 0.0, 0.8),
                secondary: Color::srgb(0.2, 0.0, 0.3),
                accent: Color::srgb(1.0, 0.5, 0.8),
                background: Color::srgb(0.1, 0.0, 0.2),
            },
        }
    }
}

/// Color palette for level themes
#[derive(Debug, Clone)]
pub struct ThemeColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
}

// ===============================
// ANIMATION UTILITIES
// ===============================

/// Animation and easing utilities
pub struct AnimationUtils;

impl AnimationUtils {
    /// Apply easing function to animation progress
    pub fn apply_easing(progress: f32, easing_type: &EasingType) -> f32 {
        match easing_type {
            EasingType::Linear => progress,
            EasingType::EaseIn => progress * progress,
            EasingType::EaseOut => 1.0 - (1.0 - progress).powi(2),
            EasingType::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress).powi(2)
                }
            },
            EasingType::Bounce => MathUtils::bounce_ease_out(progress),
            EasingType::Elastic => MathUtils::elastic_ease_out(progress),
        }
    }
    
    /// Create bouncy scale animation
    pub fn bouncy_scale(base_scale: f32, time: f32, frequency: f32, amplitude: f32) -> f32 {
        base_scale + (time * frequency).sin() * amplitude
    }
    
    /// Create floating animation (vertical movement)
    pub fn floating_offset(time: f32, amplitude: f32, frequency: f32) -> f32 {
        (time * frequency).sin() * amplitude
    }
    
    /// Create rotation animation
    pub fn rotation_animation(time: f32, speed: f32) -> f32 {
        time * speed * 2.0 * PI
    }
    
    /// Pulse animation (0.0 to 1.0)
    pub fn pulse(time: f32, frequency: f32) -> f32 {
        (time * frequency).sin() * 0.5 + 0.5
    }
    
    /// Wave animation for multiple objects
    pub fn wave_offset(base_time: f32, index: f32, frequency: f32, phase_offset: f32) -> f32 {
        ((base_time * frequency) + (index * phase_offset)).sin()
    }
}

// ===============================
// SCORING UTILITIES
// ===============================

/// Scoring calculation utilities
pub struct ScoreUtils;

impl ScoreUtils {
    /// Calculate base score for food pickup
    pub fn calculate_food_score(food_type: &FoodType, level: u32) -> u32 {
        let base_score = match food_type {
            FoodType::Normal => 10,
            FoodType::Bonus => 25,
            FoodType::Speed => 15,
            FoodType::Golden => 100,
        };
        
        // Multiply by level for progressive scoring
        base_score * level
    }
    
    /// Calculate time bonus
    pub fn calculate_time_bonus(elapsed_time: f32, target_time: f32) -> u32 {
        if elapsed_time < target_time {
            let bonus_ratio = (target_time - elapsed_time) / target_time;
            (bonus_ratio * 500.0) as u32
        } else {
            0
        }
    }
    
    /// Calculate length bonus
    pub fn calculate_length_bonus(snake_length: u32) -> u32 {
        if snake_length > 3 {
            (snake_length - 3) * 5
        } else {
            0
        }
    }
    
    /// Calculate level completion score
    pub fn calculate_level_completion_score(
        level: u32,
        base_score: u32,
        time_bonus: u32,
        length_bonus: u32,
        character_multiplier: f32,
    ) -> u32 {
        let total_base = base_score + time_bonus + length_bonus;
        let level_multiplier = 1.0 + (level as f32 * 0.1);
        
        (total_base as f32 * level_multiplier * character_multiplier) as u32
    }
    
    /// Get character score multiplier
    pub fn get_character_multiplier(character_id: u32) -> f32 {
        match character_id {
            1 => 1.0,    // Verdant Viper - Balanced
            2 => 0.9,    // Electric Eel - Fast but less score
            3 => 1.1,    // Crimson Crusher - Damage dealing bonus
            4 => 1.5,    // Golden Guardian - Score specialist
            _ => 1.0,
        }
    }
    
    /// Calculate high score ranking
    pub fn get_score_rank(score: u32) -> ScoreRank {
        if score >= 50000 {
            ScoreRank::Legendary
        } else if score >= 25000 {
            ScoreRank::Master
        } else if score >= 10000 {
            ScoreRank::Expert
        } else if score >= 5000 {
            ScoreRank::Advanced
        } else if score >= 1000 {
            ScoreRank::Intermediate
        } else {
            ScoreRank::Beginner
        }
    }
}

/// Score ranking system
#[derive(Debug, Clone, PartialEq)]
pub enum ScoreRank {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Master,
    Legendary,
}

impl ScoreRank {
    pub fn get_title(&self) -> &'static str {
        match self {
            ScoreRank::Beginner => "Snake Sprout",
            ScoreRank::Intermediate => "Garden Crawler",
            ScoreRank::Advanced => "Serpent Seeker",
            ScoreRank::Expert => "Viper Virtuoso",
            ScoreRank::Master => "Python Prodigy",
            ScoreRank::Legendary => "Vypertron Vanquisher",
        }
    }
    
    /// FIXED: Changed all Color::rgb to Color::srgb for Bevy 0.14
    pub fn get_color(&self) -> Color {
        match self {
            ScoreRank::Beginner => Color::srgb(0.6, 0.6, 0.6),    // Gray
            ScoreRank::Intermediate => Color::srgb(0.4, 0.8, 0.4), // Green
            ScoreRank::Advanced => Color::srgb(0.4, 0.4, 1.0),     // Blue
            ScoreRank::Expert => Color::srgb(0.8, 0.4, 1.0),       // Purple
            ScoreRank::Master => Color::srgb(1.0, 0.6, 0.0),       // Orange
            ScoreRank::Legendary => Color::srgb(1.0, 0.8, 0.0),    // Gold
        }
    }
}

// ===============================
// SAVE/LOAD UTILITIES
// ===============================

/// Save and load utilities for cross-platform persistence
pub struct SaveUtils;

impl SaveUtils {
    /// Serialize game data to JSON string
    pub fn serialize_game_data<T: serde::Serialize>(data: &T) -> Result<String, String> {
        serde_json::to_string_pretty(data)
            .map_err(|e| format!("Serialization error: {}", e))
    }
    
    /// Deserialize game data from JSON string
    pub fn deserialize_game_data<T: for<'de> serde::Deserialize<'de>>(
        json_str: &str
    ) -> Result<T, String> {
        serde_json::from_str(json_str)
            .map_err(|e| format!("Deserialization error: {}", e))
    }
    
    /// Get save file path for desktop
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_save_path() -> std::path::PathBuf {
        crate::desktop::get_save_directory().join("vypertron_save.json")
    }
    
    /// Save data to local storage (web) or file (desktop)
    pub fn save_game_data<T: serde::Serialize>(data: &T, _key: &str) -> Result<(), String> {
        let json_str = Self::serialize_game_data(data)?;
        
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_storage::{LocalStorage, Storage};
            LocalStorage::set(_key, json_str)
                .map_err(|e| format!("Failed to save to localStorage: {:?}", e))
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let save_path = Self::get_save_path();
            std::fs::write(&save_path, json_str)
                .map_err(|e| format!("Failed to write save file: {}", e))
        }
    }
    
    /// Load data from local storage (web) or file (desktop)
    pub fn load_game_data<T: for<'de> serde::Deserialize<'de>>(_key: &str) -> Result<T, String> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_storage::{LocalStorage, Storage};
            let json_str: String = LocalStorage::get(_key)
                .map_err(|e| format!("Failed to load from localStorage: {:?}", e))?;
            Self::deserialize_game_data(&json_str)
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let save_path = Self::get_save_path();
            let json_str = std::fs::read_to_string(&save_path)
                .map_err(|e| format!("Failed to read save file: {}", e))?;
            Self::deserialize_game_data(&json_str)
        }
    }
}

// ===============================
// INPUT UTILITIES
// ===============================

/// Input handling utilities
pub struct InputUtils;

impl InputUtils {
    /// Convert arrow keys to direction vector
    pub fn arrow_keys_to_direction(
        keyboard_input: &Res<ButtonInput<KeyCode>>,
        control_scheme: &ControlScheme,
    ) -> Option<Vec2> {
        if keyboard_input.just_pressed(control_scheme.move_up) {
            Some(Vec2::new(0.0, 1.0))
        } else if keyboard_input.just_pressed(control_scheme.move_down) {
            Some(Vec2::new(0.0, -1.0))
        } else if keyboard_input.just_pressed(control_scheme.move_left) {
            Some(Vec2::new(-1.0, 0.0))
        } else if keyboard_input.just_pressed(control_scheme.move_right) {
            Some(Vec2::new(1.0, 0.0))
        } else {
            None
        }
    }
    
    /// Check if pause key was pressed
    pub fn is_pause_pressed(
        keyboard_input: &Res<ButtonInput<KeyCode>>,
        control_scheme: &ControlScheme,
    ) -> bool {
        keyboard_input.just_pressed(control_scheme.pause)
    }
    
    /// Check if select key was pressed
    pub fn is_select_pressed(
        keyboard_input: &Res<ButtonInput<KeyCode>>,
        control_scheme: &ControlScheme,
    ) -> bool {
        keyboard_input.just_pressed(control_scheme.select)
    }
    
    /// Check if back key was pressed
    pub fn is_back_pressed(
        keyboard_input: &Res<ButtonInput<KeyCode>>,
        control_scheme: &ControlScheme,
    ) -> bool {
        keyboard_input.just_pressed(control_scheme.back)
    }
    
    /// Get opposite direction (for preventing 180-degree turns)
    pub fn get_opposite_direction(direction: Vec2) -> Vec2 {
        Vec2::new(-direction.x, -direction.y)
    }
    
    /// Check if new direction is valid (not opposite to current)
    pub fn is_valid_direction_change(current: Vec2, new: Vec2) -> bool {
        // Allow change if not moving opposite direction
        let opposite = Self::get_opposite_direction(current);
        (new - opposite).length() > 0.1
    }
}

// ===============================
// RANDOM UTILITIES
// ===============================

/// Random generation utilities with game-specific functions
pub struct RandomUtils;

impl RandomUtils {
    /// Generate random food type based on level and probabilities
    pub fn random_food_type(level: u32, rng: &mut impl Rng) -> FoodType {
        let random_value = rng.gen::<f32>();
        
        // Adjust probabilities based on level
        let bonus_chance = 0.15 + (level as f32 * 0.01);
        let speed_chance = 0.05 + (level as f32 * 0.005);
        let golden_chance = 0.02 + (level as f32 * 0.002);
        
        if random_value < golden_chance {
            FoodType::Golden
        } else if random_value < golden_chance + speed_chance {
            FoodType::Speed
        } else if random_value < golden_chance + speed_chance + bonus_chance {
            FoodType::Bonus
        } else {
            FoodType::Normal
        }
    }
    
    /// Generate random safe position (not occupied by snake or walls)
    pub fn random_safe_position(
        grid_width: u32,
        grid_height: u32,
        occupied_positions: &[Vec2],
        rng: &mut impl Rng,
    ) -> Option<Vec2> {
        let max_attempts = 100;
        
        for _ in 0..max_attempts {
            let pos = MathUtils::random_grid_position(grid_width, grid_height, rng);
            
            // Check if position is not occupied
            if !occupied_positions.iter().any(|&occupied| {
                MathUtils::grid_distance(pos, occupied) < 1.0
            }) {
                return Some(pos);
            }
        }
        
        None // Couldn't find safe position
    }
    
    /// Generate random color variation
    /// FIXED: Updated color access methods for Bevy 0.14
    pub fn random_color_variation(base_color: Color, variation: f32, rng: &mut impl Rng) -> Color {
        let var = variation.clamp(0.0, 1.0);
        let base = base_color.to_srgba();
        
        Color::srgba(
            (base.red + rng.gen_range(-var..var)).clamp(0.0, 1.0),
            (base.green + rng.gen_range(-var..var)).clamp(0.0, 1.0),
            (base.blue + rng.gen_range(-var..var)).clamp(0.0, 1.0),
            base.alpha,
        )
    }
}

// ===============================
// DEBUGGING UTILITIES
// ===============================

/// Debugging and development utilities
#[cfg(debug_assertions)]
pub struct DebugUtils;

#[cfg(debug_assertions)]
impl DebugUtils {
    /// Print grid state for debugging
    pub fn print_grid_state(
        grid_width: u32,
        grid_height: u32,
        snake_positions: &[Vec2],
        food_positions: &[Vec2],
        wall_positions: &[Vec2],
    ) {
        println!("=== GRID STATE ===");
        for y in (0..grid_height).rev() {
            for x in 0..grid_width {
                let pos = Vec2::new(x as f32, y as f32);
                
                if snake_positions.contains(&pos) {
                    print!("S");
                } else if food_positions.contains(&pos) {
                    print!("F");
                } else if wall_positions.contains(&pos) {
                    print!("W");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("==================");
    }
    
    /// Log performance metrics
    pub fn log_performance_metrics(frame_time: f32, entity_count: usize) {
        if frame_time > 0.016 { // More than 16ms (60 FPS)
            warn!("Frame time: {:.3}ms, Entities: {}", frame_time * 1000.0, entity_count);
        }
    }
}