//! # 🐍⚡ Vypertron-Snake
//! 
//! A premium Snake game built with 100% Rust and Bevy Engine.
//! 
//! ## Features
//! 
//! - **10 Unique Levels** with distinct themes and progressive difficulty
//! - **4 Selectable Characters** with unique visual styles
//! - **Cinematic Cutscenes** between levels for immersive storytelling
//! - **NES-Style Home Screen** with animated title snake
//! - **Custom Audio Experience** with BeepBox music and Rust-generated SFX
//! - **High Score Tracking** with persistent storage
//! - **Pause/Resume System** for flexible gameplay
//! - **Explosive Death Effects** when your snake meets its end
//! - **Cross-Platform** deployment (Desktop + Web Browser)
//! 
//! ## Architecture
//! 
//! Vypertron-Snake is built using Bevy's Entity-Component-System (ECS) architecture,
//! providing clean separation of concerns and excellent performance:
//! 
//! - **States**: Game state management (Home, Playing, Paused, etc.)
//! - **Systems**: Game logic and behavior (movement, collision, audio, etc.)
//! - **Components**: Data containers for entities (Snake, Food, UI, etc.)
//! - **Resources**: Global game data (Score, Settings, Timer, etc.)
//! - **Levels**: Progressive level definitions and themes
//! - **Audio**: Music and sound effect management
//! - **Utils**: Helper functions and utilities
//! 
//! ## Usage
//! 
//! ```rust
//! use bevy::prelude::*;
//! use vypertron_snake::GamePlugin;
//! 
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(GamePlugin)
//!         .run();
//! }
//! ```
//! 
//! ## Game Loop
//! 
//! 1. **Home Screen**: Classic NES-inspired title with animated snake
//! 2. **Character Selection**: Choose from 4 unique snake characters
//! 3. **Gameplay**: Progressive levels with increasing difficulty
//! 4. **Cutscenes**: Story elements between levels
//! 5. **Level Completion**: Victory effects and progression
//! 6. **Game Over**: Explosive death effects and high score updates
//! 
//! ## Technical Excellence
//! 
//! - **Memory Safety**: 100% Rust implementation with zero unsafe code
//! - **Performance**: Optimized for both desktop and web deployment
//! - **Modularity**: Clean, maintainable code architecture
//! - **Cross-Platform**: WebAssembly ready for browser deployment
//! - **Professional Polish**: AAA-level game feel and presentation

// Re-export Bevy prelude for convenience
pub use bevy::prelude::*;

// Main game plugin module
pub mod game_plugin;

// Re-export the main GamePlugin for easy access
pub use game_plugin::GamePlugin;

// Re-export all public types from our game plugin
pub use game_plugin::{
    GameState,
    PauseState,
    SnakeDirection,
    GameTimer,
    ScoreResource,
};

// Module declarations for all our game systems
// These will be implemented in subsequent files

/// Game state management and transitions
pub mod states {
    pub use crate::game_plugin::states::*;
}

/// Core game systems (movement, collision, input, etc.)
pub mod systems {
    pub use crate::game_plugin::systems::*;
}

/// ECS components for game entities
pub mod components {
    pub use crate::game_plugin::components::*;
}

/// Global game resources and data
pub mod resources {
    pub use crate::game_plugin::resources::*;
}

/// Level definitions and progression
pub mod levels {
    pub use crate::game_plugin::levels::*;
}

/// Audio system for music and sound effects
pub mod audio {
    pub use crate::game_plugin::audio::*;
}

/// Utility functions and helpers
pub mod utils {
    pub use crate::game_plugin::utils::*;
}

// Crate-level constants
/// Game version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Game title
pub const GAME_TITLE: &str = "🐍⚡ Vypertron-Snake";

/// Default window width
pub const DEFAULT_WINDOW_WIDTH: f32 = 1200.0;

/// Default window height  
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// Grid size for snake movement (pixels)
pub const GRID_SIZE: f32 = 20.0;

/// Number of levels in the game
pub const TOTAL_LEVELS: u32 = 10;

/// Number of selectable characters
pub const CHARACTER_COUNT: u32 = 4;

/// Default snake speed (moves per second)
pub const DEFAULT_SNAKE_SPEED: f32 = 5.0;

/// Score multiplier per level
pub const LEVEL_SCORE_MULTIPLIER: u32 = 100;

// Platform-specific configurations
#[cfg(target_arch = "wasm32")]
pub mod web {
    //! Web-specific functionality and optimizations
    
    /// Initialize web-specific features
    pub fn init() {
        // Set up console panic hook for better error reporting
        console_error_panic_hook::set_once();
        
        // Initialize wee_alloc as global allocator for smaller WASM size
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
    
    /// Get high scores from browser local storage
    pub fn get_stored_high_scores() -> Vec<u32> {
        // Implementation will use gloo-storage
        vec![]
    }
    
    /// Save high scores to browser local storage
    pub fn save_high_scores(scores: Vec<u32>) {
        // Implementation will use gloo-storage
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
        // Ensure save directory exists
        let save_dir = get_save_directory();
        if !save_dir.exists() {
            std::fs::create_dir_all(&save_dir).ok();
        }
    }
}