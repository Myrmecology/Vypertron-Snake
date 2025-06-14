//! # 🐍⚡ Vypertron-Snake
//! 
//! A premium Snake game built entirely in Rust using the Bevy engine.
//! 
//! ## Features
//! 
//! - 10 Unique Levels with distinct themes and increasing difficulty
//! - 4 Selectable Characters with unique visual styles
//! - Cinematic Cutscenes between levels
//! - NES-Style Home Screen with animated title snake
//! - Custom Music (BeepBox) + Rust-generated SFX
//! - High Score Tracking (Persistent)
//! - Pause/Resume System
//! - Explosive Death Effects
//! - Cross-Platform: Desktop + Web Browser
//! 
//! ## Architecture
//! - States: Game state transitions (e.g., Home, Playing, Paused)
//! - Systems: Game behavior (e.g., movement, collision, audio)
//! - Components: Entity data (Snake, Food, UI, etc.)
//! - Resources: Global runtime data (e.g., score, timer)
//! - Levels: Scene definitions and theming
//! - Audio: Music/SFX loading and playback
//! - Utils: Reusable helpers and math
//! 
//! ## Game Flow
//! 1. Home Screen → 2. Character Select → 3. Levels + Cutscenes → 4. Game Over or Victory

// === Bevy + Core Re-exports ===
pub use bevy::prelude::*;

// === Modules ===
pub mod game_plugin;
pub mod states;
pub mod systems;
pub mod components;
pub mod resources;
pub mod levels;
pub mod audio;
pub mod utils;

// === Plugin Re-export ===
pub use game_plugin::GamePlugin;

// === Constants ===
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GAME_TITLE: &str = "🐍⚡ Vypertron-Snake";
pub const DEFAULT_WINDOW_WIDTH: f32 = 1200.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;
pub const GRID_SIZE: f32 = 20.0;
pub const TOTAL_LEVELS: u32 = 10;
pub const CHARACTER_COUNT: u32 = 4;
pub const DEFAULT_SNAKE_SPEED: f32 = 5.0;
pub const LEVEL_SCORE_MULTIPLIER: u32 = 100;

// === Platform-Specific Modules ===
#[cfg(target_arch = "wasm32")]
pub mod web {
    //! Web-specific functionality and optimizations

    use console_error_panic_hook;

    /// Initialize web-specific features
    pub fn init() {
        console_error_panic_hook::set_once();
    }

    /// Get high scores from browser local storage
    pub fn get_stored_high_scores() -> Vec<u32> {
        // TODO: Implement using gloo-storage crate
        vec![]
    }

    /// Save high scores to browser local storage
    pub fn save_high_scores(_scores: Vec<u32>) {
        // TODO: Implement using gloo-storage crate
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod desktop {
    //! Desktop-specific functionality and optimizations

    use std::path::PathBuf;

    /// Get the save directory for game data
    pub fn get_save_directory() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("vypertron-snake")
    }

    /// Initialize desktop-specific features
    pub fn init() {
        let save_dir = get_save_directory();
        if !save_dir.exists() {
            std::fs::create_dir_all(&save_dir).ok();
        }
    }
}
