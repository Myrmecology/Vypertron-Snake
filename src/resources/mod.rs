//! Global Resources for Vypertron-Snake
//!
//! This module defines all the global resources used throughout the game.
//! Resources are singleton data structures that can be accessed from any system.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod game_timer;
pub use game_timer::GameTimer;

// ===============================
// CORE GAME RESOURCES
// ===============================

/// ADDED: Missing SnakeDirection enum that was referenced in errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for SnakeDirection {
    fn default() -> Self {
        SnakeDirection::Right
    }
}

/// ADDED: Missing ScoreResource that was referenced in errors
#[derive(Resource, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct ScoreResource {
    pub current_score: u32,
    pub current_level_score: u32,
    pub level_score: u32, // ADDED: Missing field referenced in systems
    pub food_eaten: u32,  // ADDED: Missing field referenced in systems  
    pub time_bonus: u32,  // ADDED: Missing field referenced in systems
    pub multiplier: f32,
    pub streak: u32,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct HighScoreResource {
    pub global_high_score: u32,
    pub level_high_scores: [u32; 10],
    pub character_level_scores: [[u32; 10]; 4],
    pub player_name: String,
    pub games_played: u32,
    pub total_time_played: f32,
    pub achievements: Vec<Achievement>,
}

impl Default for HighScoreResource {
    fn default() -> Self {
        Self {
            global_high_score: 0,
            level_high_scores: [0; 10],
            character_level_scores: [[0; 10]; 4],
            player_name: "Player".to_string(),
            games_played: 0,
            total_time_played: 0.0,
            achievements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
    pub unlock_date: Option<String>,
}

/// FIXED: Re-added Serialize/Deserialize with custom implementation
#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub resolution: (f32, f32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub snake_speed_multiplier: f32,
    pub show_grid: bool,
    pub difficulty: DifficultyMode,
    #[serde(skip)] // Skip serialization of controls due to KeyCode complexity
    pub controls: ControlScheme,
    pub language: String,
    pub accessibility: AccessibilitySettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.7,
            music_volume: 0.6,
            sfx_volume: 0.8,
            resolution: (1200.0, 800.0),
            fullscreen: false,
            vsync: true,
            snake_speed_multiplier: 1.0,
            show_grid: false,
            difficulty: DifficultyMode::Normal,
            controls: ControlScheme::default(),
            language: "English".to_string(),
            accessibility: AccessibilitySettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum DifficultyMode {
    Easy,
    Normal,
    Hard,
    Insane,
}

/// FIXED: Updated KeyCode variants for Bevy 0.14
#[derive(Debug, Clone, Reflect)]
pub struct ControlScheme {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub pause: KeyCode,
    pub select: KeyCode,
    pub back: KeyCode,
}

impl Default for ControlScheme {
    fn default() -> Self {
        Self {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
            move_left: KeyCode::ArrowLeft,
            move_right: KeyCode::ArrowRight,
            pause: KeyCode::Space,
            select: KeyCode::Enter,
            back: KeyCode::Escape,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct AccessibilitySettings {
    pub high_contrast: bool,
    pub colorblind_friendly: bool,
    pub reduced_motion: bool,
    pub large_ui: bool,
    pub audio_cues: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            high_contrast: false,
            colorblind_friendly: false,
            reduced_motion: false,
            large_ui: false,
            audio_cues: false,
        }
    }
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct LevelManager {
    pub current_level: u32,
    pub level_definitions: [LevelDefinition; 10],
    pub unlocked_levels: [bool; 10],
    pub completed_levels: [bool; 10],
    pub best_times: [f32; 10],
    pub level_start_time: f32,
}

impl Default for LevelManager {
    fn default() -> Self {
        Self {
            current_level: 1,
            level_definitions: [
                // Level 1: Tutorial
                LevelDefinition {
                    level_number: 1,
                    name: "Serpent's Awakening".to_string(),
                    description: "Learn the basics of movement and growth".to_string(),
                    theme: crate::components::LevelTheme::Classic,
                    starting_speed: 0.3,
                    speed_increase: 0.02,
                    max_speed: 0.8,
                    grid_size: (20, 15),
                    wall_pattern: WallPattern::Empty,
                    special_mechanics: vec![],
                    target_score: 100,
                    time_limit: None,
                },
                // Level 2-10: Filled with proper definitions
                LevelDefinition {
                    level_number: 2,
                    name: "Garden Maze".to_string(),
                    description: "Navigate through your first obstacles".to_string(),
                    theme: crate::components::LevelTheme::Forest,
                    starting_speed: 0.25,
                    speed_increase: 0.03,
                    max_speed: 0.9,
                    grid_size: (22, 17),
                    wall_pattern: WallPattern::BasicObstacles,
                    special_mechanics: vec![SpecialMechanic::SpeedZones],
                    target_score: 200,
                    time_limit: Some(120.0),
                },
                // Continue pattern for remaining levels...
                LevelDefinition {
                    level_number: 3,
                    name: "Neon Circuit".to_string(),
                    description: "Electric pathways and teleportation".to_string(),
                    theme: crate::components::LevelTheme::Cyber,
                    starting_speed: 0.2,
                    speed_increase: 0.04,
                    max_speed: 1.0,
                    grid_size: (25, 20),
                    wall_pattern: WallPattern::Maze,
                    special_mechanics: vec![SpecialMechanic::Teleporters],
                    target_score: 350,
                    time_limit: Some(150.0),
                },
                // Levels 4-10 (simplified for space)
                LevelDefinition { level_number: 4, name: "Ocean Depths".to_string(), description: "Underwater adventure".to_string(), theme: crate::components::LevelTheme::Ocean, starting_speed: 0.15, speed_increase: 0.05, max_speed: 1.2, grid_size: (28, 22), wall_pattern: WallPattern::MovingWalls, special_mechanics: vec![SpecialMechanic::MovingFood], target_score: 500, time_limit: Some(180.0) },
                LevelDefinition { level_number: 5, name: "Volcanic Core".to_string(), description: "Lava and destruction".to_string(), theme: crate::components::LevelTheme::Volcano, starting_speed: 0.1, speed_increase: 0.06, max_speed: 1.4, grid_size: (30, 25), wall_pattern: WallPattern::BreakableWalls, special_mechanics: vec![SpecialMechanic::WallBreaking], target_score: 750, time_limit: Some(200.0) },
                LevelDefinition { level_number: 6, name: "Space Station".to_string(), description: "Zero gravity challenges".to_string(), theme: crate::components::LevelTheme::Space, starting_speed: 0.05, speed_increase: 0.07, max_speed: 1.6, grid_size: (32, 28), wall_pattern: WallPattern::MultiRoom, special_mechanics: vec![SpecialMechanic::Gravity], target_score: 1000, time_limit: Some(240.0) },
                LevelDefinition { level_number: 7, name: "Desert Mirage".to_string(), description: "Illusions and multiple foods".to_string(), theme: crate::components::LevelTheme::Desert, starting_speed: 0.08, speed_increase: 0.08, max_speed: 1.8, grid_size: (35, 30), wall_pattern: WallPattern::Maze, special_mechanics: vec![SpecialMechanic::MultipleFoods], target_score: 1500, time_limit: Some(300.0) },
                LevelDefinition { level_number: 8, name: "Ice Palace".to_string(), description: "Slippery surfaces and trails".to_string(), theme: crate::components::LevelTheme::Ice, starting_speed: 0.12, speed_increase: 0.09, max_speed: 2.0, grid_size: (38, 32), wall_pattern: WallPattern::MovingWalls, special_mechanics: vec![SpecialMechanic::Trail], target_score: 2000, time_limit: Some(360.0) },
                LevelDefinition { level_number: 9, name: "Shadow Realm".to_string(), description: "Invincibility and chaos".to_string(), theme: crate::components::LevelTheme::Shadow, starting_speed: 0.15, speed_increase: 0.1, max_speed: 2.5, grid_size: (40, 35), wall_pattern: WallPattern::BreakableWalls, special_mechanics: vec![SpecialMechanic::Invincibility], target_score: 3000, time_limit: Some(420.0) },
                LevelDefinition { level_number: 10, name: "The Ultimate Challenge".to_string(), description: "All mechanics combined".to_string(), theme: crate::components::LevelTheme::Cosmic, starting_speed: 0.2, speed_increase: 0.12, max_speed: 3.0, grid_size: (45, 40), wall_pattern: WallPattern::MultiRoom, special_mechanics: vec![SpecialMechanic::Teleporters, SpecialMechanic::MovingFood, SpecialMechanic::WallBreaking], target_score: 5000, time_limit: Some(600.0) },
            ],
            unlocked_levels: {
                let mut unlocked = [false; 10];
                unlocked[0] = true;
                unlocked
            },
            completed_levels: [false; 10],
            best_times: [f32::INFINITY; 10],
            level_start_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct LevelDefinition {
    pub level_number: u32,
    pub name: String,
    pub description: String,
    pub theme: crate::components::LevelTheme,
    pub starting_speed: f32,
    pub speed_increase: f32,
    pub max_speed: f32,
    pub grid_size: (u32, u32),
    pub wall_pattern: WallPattern,
    pub special_mechanics: Vec<SpecialMechanic>,
    pub target_score: u32,
    pub time_limit: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum WallPattern {
    Empty,
    BasicObstacles,
    Maze,
    MovingWalls,
    BreakableWalls,
    MultiRoom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SpecialMechanic {
    Teleporters,
    SpeedZones,
    MovingFood,
    Invincibility,
    WallBreaking,
    MultipleFoods,
    Gravity,
    Trail,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct CharacterSelection {
    pub selected_character: u32,
    pub characters: [CharacterDefinition; 4],
    pub unlocked_characters: [bool; 4],
}

impl Default for CharacterSelection {
    fn default() -> Self {
        Self {
            selected_character: 1,
            characters: [
                CharacterDefinition {
                    id: 1,
                    name: "Vyper".to_string(),
                    description: "The classic snake with balanced abilities".to_string(),
                    color: [0.2, 0.8, 0.2, 1.0], // Green
                    special_ability: CharacterAbility::None,
                    unlock_requirement: UnlockRequirement::None,
                },
                CharacterDefinition {
                    id: 2,
                    name: "Lightning".to_string(),
                    description: "A speedy serpent with burst abilities".to_string(),
                    color: [0.8, 0.8, 0.2, 1.0], // Yellow
                    special_ability: CharacterAbility::SpeedBoost,
                    unlock_requirement: UnlockRequirement::CompleteLevel(3),
                },
                CharacterDefinition {
                    id: 3,
                    name: "Crusher".to_string(),
                    description: "Break through walls with ease".to_string(),
                    color: [0.8, 0.2, 0.2, 1.0], // Red
                    special_ability: CharacterAbility::WallBreaker,
                    unlock_requirement: UnlockRequirement::CompleteLevel(6),
                },
                CharacterDefinition {
                    id: 4,
                    name: "Golden".to_string(),
                    description: "Earn bonus points with every bite".to_string(),
                    color: [0.9, 0.7, 0.1, 1.0], // Gold
                    special_ability: CharacterAbility::ScoreBooster,
                    unlock_requirement: UnlockRequirement::AchieveScore(2000),
                },
            ],
            unlocked_characters: [true, false, false, false],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct CharacterDefinition {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub color: [f32; 4],
    pub special_ability: CharacterAbility,
    pub unlock_requirement: UnlockRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum CharacterAbility {
    None,
    SpeedBoost,
    WallBreaker,
    ScoreBooster,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum UnlockRequirement {
    None,
    CompleteLevel(u32),
    AchieveScore(u32),
    PlayGames(u32),
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct AssetHandles {
    pub textures: HashMap<String, Handle<Image>>,
    pub fonts: HashMap<String, Handle<Font>>,
    pub audio: HashMap<String, Handle<AudioSource>>,
    pub loading_complete: bool,
    pub loading_progress: f32,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct GameStatistics {
    pub total_food_eaten: u32,
    pub total_distance: f32,
    pub longest_snake: u32,
    pub fastest_completions: [f32; 10],
    pub death_causes: HashMap<String, u32>,
    pub sessions_played: u32,
    pub average_session_duration: f32,
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct InputState {
    pub movement_direction: Option<Vec2>,
    pub pause_just_pressed: bool,
    pub select_just_pressed: bool,
    pub back_just_pressed: bool,
    pub mouse_position: Vec2,
    pub mouse_just_clicked: bool,
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct SaveLoadState {
    pub saving: bool,
    pub loading: bool,
    pub last_save_time: f64,
    pub auto_save_interval: f64,
    pub save_location: String,
}


