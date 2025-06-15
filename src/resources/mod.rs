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

/// FIXED: Removed Serialize/Deserialize from GameSettings due to KeyCode serialization issues in Bevy 0.14
#[derive(Resource, Debug, Clone, Reflect)]
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

/// FIXED: Removed Serialize/Deserialize from ControlScheme due to KeyCode serialization issues in Bevy 0.14
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
            level_definitions: todo!(),
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

/// FIXED: Added PartialEq to SpecialMechanic to fix comparison errors
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
            characters: todo!(),
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

// ===============================
// STATE TRANSITION EVENTS
// ===============================

/// FIXED: Added StateTransitionEvent definition that was missing
#[derive(Event, Debug, Clone)]
pub enum StateTransitionEvent {
    GameStart,
    GameOver { 
        final_score: u32,
        cause: String,
    },
    LevelComplete { 
        score: u32,
        level: u32,
    },
    PauseGame,
    ResumeGame,
    ReturnToMenu,
}


