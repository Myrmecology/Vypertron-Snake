//! Global Resources for Vypertron-Snake
//! 
//! This module defines all the global resources used throughout the game.
//! Resources are singleton data structures that can be accessed from any system.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===============================
// CORE GAME RESOURCES
// ===============================

/// High score tracking and persistence
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct HighScoreResource {
    /// Overall high score across all levels
    pub global_high_score: u32,
    /// High scores per level (index 0 = level 1)
    pub level_high_scores: [u32; 10],
    /// High scores per character per level [character_id][level_index]
    pub character_level_scores: [[u32; 10]; 4],
    /// Player name for high score
    pub player_name: String,
    /// Total games played
    pub games_played: u32,
    /// Total time played (seconds)
    pub total_time_played: f32,
    /// Achievements unlocked
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

/// Game achievement system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
    pub unlock_date: Option<String>,
}

/// Game settings and configuration
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// Music volume (0.0 to 1.0)
    pub music_volume: f32,
    /// Sound effects volume (0.0 to 1.0)
    pub sfx_volume: f32,
    /// Window resolution
    pub resolution: (f32, f32),
    /// Fullscreen mode
    pub fullscreen: bool,
    /// VSync enabled
    pub vsync: bool,
    /// Snake movement speed multiplier
    pub snake_speed_multiplier: f32,
    /// Show grid lines
    pub show_grid: bool,
    /// Difficulty mode
    pub difficulty: DifficultyMode,
    /// Control scheme
    pub controls: ControlScheme,
    /// Language setting
    pub language: String,
    /// Accessibility options
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

/// Difficulty modes affecting gameplay
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DifficultyMode {
    Easy,    // Slower speed, more forgiving
    Normal,  // Standard gameplay
    Hard,    // Faster speed, less margin for error
    Insane,  // Maximum challenge
}

/// Control scheme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Accessibility settings for inclusive gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// High contrast mode
    pub high_contrast: bool,
    /// Color blind friendly palette
    pub colorblind_friendly: bool,
    /// Reduced motion effects
    pub reduced_motion: bool,
    /// Larger UI elements
    pub large_ui: bool,
    /// Audio cues for visual elements
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

/// Level management and progression
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct LevelManager {
    /// Current level (1-10)
    pub current_level: u32,
    /// Level definitions for all 10 levels
    pub level_definitions: [LevelDefinition; 10],
    /// Unlocked levels (true = unlocked)
    pub unlocked_levels: [bool; 10],
    /// Level completion status
    pub completed_levels: [bool; 10],
    /// Best completion time per level
    pub best_times: [f32; 10],
    /// Current level start time
    pub level_start_time: f32,
}

impl Default for LevelManager {
    fn default() -> Self {
        Self {
            current_level: 1,
            level_definitions: LevelDefinition::create_all_levels(),
            unlocked_levels: {
                let mut unlocked = [false; 10];
                unlocked[0] = true; // Level 1 starts unlocked
                unlocked
            },
            completed_levels: [false; 10],
            best_times: [f32::INFINITY; 10],
            level_start_time: 0.0,
        }
    }
}

/// Definition for individual levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelDefinition {
    /// Level number (1-10)
    pub level_number: u32,
    /// Level name
    pub name: String,
    /// Level description
    pub description: String,
    /// Visual theme
    pub theme: crate::components::LevelTheme,
    /// Snake starting speed
    pub starting_speed: f32,
    /// Speed increase per food eaten
    pub speed_increase: f32,
    /// Maximum speed for this level
    pub max_speed: f32,
    /// Grid dimensions (width, height)
    pub grid_size: (u32, u32),
    /// Wall layout pattern
    pub wall_pattern: WallPattern,
    /// Special mechanics for this level
    pub special_mechanics: Vec<SpecialMechanic>,
    /// Target score for completion
    pub target_score: u32,
    /// Time limit (None = no limit)
    pub time_limit: Option<f32>,
}

/// Wall layout patterns for level variety
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WallPattern {
    /// Just boundary walls
    Empty,
    /// Simple obstacles
    BasicObstacles,
    /// Maze-like layout
    Maze,
    /// Moving obstacles
    MovingWalls,
    /// Breakable wall clusters
    BreakableWalls,
    /// Complex multi-room layout
    MultiRoom,
}

/// Special gameplay mechanics per level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialMechanic {
    /// Teleport portals
    Teleporters,
    /// Speed zones that change snake speed
    SpeedZones,
    /// Food that moves around
    MovingFood,
    /// Temporary invincibility power-up
    Invincibility,
    /// Snake can break through certain walls
    WallBreaking,
    /// Multiple food spawns simultaneously
    MultipleFoods,
    /// Gravity affects snake movement
    Gravity,
    /// Snake leaves a temporary trail
    Trail,
}

impl LevelDefinition {
    /// Create all 10 level definitions
    pub fn create_all_levels() -> [LevelDefinition; 10] {
        [
            // Level 1: Classic Introduction
            LevelDefinition {
                level_number: 1,
                name: "Garden of Beginnings".to_string(),
                description: "A peaceful start to your journey".to_string(),
                theme: crate::components::LevelTheme::Classic,
                starting_speed: 3.0,
                speed_increase: 0.1,
                max_speed: 6.0,
                grid_size: (30, 20),
                wall_pattern: WallPattern::Empty,
                special_mechanics: vec![],
                target_score: 100,
                time_limit: None,
            },
            // Level 2: Digital Realm
            LevelDefinition {
                level_number: 2,
                name: "Digital Nexus".to_string(),
                description: "Enter the cyber world".to_string(),
                theme: crate::components::LevelTheme::Digital,
                starting_speed: 3.5,
                speed_increase: 0.15,
                max_speed: 7.0,
                grid_size: (32, 22),
                wall_pattern: WallPattern::BasicObstacles,
                special_mechanics: vec![SpecialMechanic::SpeedZones],
                target_score: 200,
                time_limit: None,
            },
            // Level 3: Forest Maze
            LevelDefinition {
                level_number: 3,
                name: "Whispering Woods".to_string(),
                description: "Navigate the natural labyrinth".to_string(),
                theme: crate::components::LevelTheme::Forest,
                starting_speed: 4.0,
                speed_increase: 0.2,
                max_speed: 8.0,
                grid_size: (34, 24),
                wall_pattern: WallPattern::Maze,
                special_mechanics: vec![SpecialMechanic::MovingFood],
                target_score: 350,
                time_limit: Some(180.0), // 3 minutes
            },
            // Level 4: Desert Storms
            LevelDefinition {
                level_number: 4,
                name: "Sandstorm Arena".to_string(),
                description: "Survive the shifting sands".to_string(),
                theme: crate::components::LevelTheme::Desert,
                starting_speed: 4.5,
                speed_increase: 0.25,
                max_speed: 9.0,
                grid_size: (36, 26),
                wall_pattern: WallPattern::MovingWalls,
                special_mechanics: vec![SpecialMechanic::Teleporters, SpecialMechanic::Trail],
                target_score: 500,
                time_limit: Some(240.0), // 4 minutes
            },
            // Level 5: Ocean Depths
            LevelDefinition {
                level_number: 5,
                name: "Abyssal Current".to_string(),
                description: "Dive into aquatic challenges".to_string(),
                theme: crate::components::LevelTheme::Ocean,
                starting_speed: 5.0,
                speed_increase: 0.3,
                max_speed: 10.0,
                grid_size: (38, 28),
                wall_pattern: WallPattern::BreakableWalls,
                special_mechanics: vec![SpecialMechanic::WallBreaking, SpecialMechanic::MultipleFoods],
                target_score: 750,
                time_limit: Some(300.0), // 5 minutes
            },
            // Level 6: Volcanic Forge
            LevelDefinition {
                level_number: 6,
                name: "Molten Core".to_string(),
                description: "Feel the heat of creation".to_string(),
                theme: crate::components::LevelTheme::Volcano,
                starting_speed: 5.5,
                speed_increase: 0.35,
                max_speed: 12.0,
                grid_size: (40, 30),
                wall_pattern: WallPattern::MultiRoom,
                special_mechanics: vec![SpecialMechanic::SpeedZones, SpecialMechanic::Invincibility],
                target_score: 1000,
                time_limit: Some(360.0), // 6 minutes
            },
            // Level 7: Frozen Tundra
            LevelDefinition {
                level_number: 7,
                name: "Crystal Caverns".to_string(),
                description: "Navigate the slippery ice".to_string(),
                theme: crate::components::LevelTheme::Ice,
                starting_speed: 6.0,
                speed_increase: 0.4,
                max_speed: 14.0,
                grid_size: (42, 32),
                wall_pattern: WallPattern::Maze,
                special_mechanics: vec![SpecialMechanic::Gravity, SpecialMechanic::MovingFood, SpecialMechanic::Trail],
                target_score: 1500,
                time_limit: Some(420.0), // 7 minutes
            },
            // Level 8: Space Station
            LevelDefinition {
                level_number: 8,
                name: "Stellar Observatory".to_string(),
                description: "Zero gravity, infinite possibilities".to_string(),
                theme: crate::components::LevelTheme::Space,
                starting_speed: 6.5,
                speed_increase: 0.45,
                max_speed: 16.0,
                grid_size: (44, 34),
                wall_pattern: WallPattern::MovingWalls,
                special_mechanics: vec![SpecialMechanic::Teleporters, SpecialMechanic::Gravity, SpecialMechanic::MultipleFoods],
                target_score: 2000,
                time_limit: Some(480.0), // 8 minutes
            },
            // Level 9: Neon Metropolis
            LevelDefinition {
                level_number: 9,
                name: "Neon Nights".to_string(),
                description: "The city that never sleeps".to_string(),
                theme: crate::components::LevelTheme::NeonCity,
                starting_speed: 7.0,
                speed_increase: 0.5,
                max_speed: 18.0,
                grid_size: (46, 36),
                wall_pattern: WallPattern::MultiRoom,
                special_mechanics: vec![SpecialMechanic::SpeedZones, SpecialMechanic::WallBreaking, SpecialMechanic::Invincibility, SpecialMechanic::Trail],
                target_score: 3000,
                time_limit: Some(540.0), // 9 minutes
            },
            // Level 10: Final Boss
            LevelDefinition {
                level_number: 10,
                name: "Vypertron's Lair".to_string(),
                description: "Face the ultimate challenge".to_string(),
                theme: crate::components::LevelTheme::FinalBoss,
                starting_speed: 8.0,
                speed_increase: 0.6,
                max_speed: 20.0,
                grid_size: (50, 40),
                wall_pattern: WallPattern::MultiRoom,
                special_mechanics: vec![
                    SpecialMechanic::Teleporters,
                    SpecialMechanic::SpeedZones,
                    SpecialMechanic::MovingFood,
                    SpecialMechanic::WallBreaking,
                    SpecialMechanic::MultipleFoods,
                    SpecialMechanic::Gravity,
                    SpecialMechanic::Trail,
                ],
                target_score: 5000,
                time_limit: Some(600.0), // 10 minutes
            },
        ]
    }
}

/// Character selection and customization
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSelection {
    /// Currently selected character (1-4)
    pub selected_character: u32,
    /// Character definitions
    pub characters: [CharacterDefinition; 4],
    /// Unlock status for each character
    pub unlocked_characters: [bool; 4],
}

impl Default for CharacterSelection {
    fn default() -> Self {
        Self {
            selected_character: 1,
            characters: CharacterDefinition::create_all_characters(),
            unlocked_characters: [true, false, false, false], // Only first character unlocked initially
        }
    }
}

/// Definition for playable characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDefinition {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub color: [f32; 4], // RGBA
    pub special_ability: CharacterAbility,
    pub unlock_requirement: UnlockRequirement,
}

/// Special abilities for each character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterAbility {
    /// No special ability - balanced gameplay
    None,
    /// Move slightly faster
    SpeedBoost,
    /// Can break through one wall per level
    WallBreaker,
    /// Bonus score multiplier
    ScoreBooster,
}

/// Requirements to unlock characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockRequirement {
    /// Unlocked from start
    None,
    /// Complete a certain level
    CompleteLevel(u32),
    /// Achieve a target score
    AchieveScore(u32),
    /// Play a certain number of games
    PlayGames(u32),
}

impl CharacterDefinition {
    /// Create all 4 character definitions
    pub fn create_all_characters() -> [CharacterDefinition; 4] {
        [
            CharacterDefinition {
                id: 1,
                name: "Verdant Viper".to_string(),
                description: "The classic green snake - balanced and reliable".to_string(),
                color: [0.2, 0.8, 0.2, 1.0], // Green
                special_ability: CharacterAbility::None,
                unlock_requirement: UnlockRequirement::None,
            },
            CharacterDefinition {
                id: 2,
                name: "Electric Eel".to_string(),
                description: "Lightning fast with a speed boost ability".to_string(),
                color: [0.1, 0.5, 1.0, 1.0], // Electric Blue
                special_ability: CharacterAbility::SpeedBoost,
                unlock_requirement: UnlockRequirement::CompleteLevel(3),
            },
            CharacterDefinition {
                id: 3,
                name: "Crimson Crusher".to_string(),
                description: "Break through walls with fiery determination".to_string(),
                color: [1.0, 0.2, 0.1, 1.0], // Fire Red
                special_ability: CharacterAbility::WallBreaker,
                unlock_requirement: UnlockRequirement::CompleteLevel(6),
            },
            CharacterDefinition {
                id: 4,
                name: "Golden Guardian".to_string(),
                description: "Earn bonus points with every move".to_string(),
                color: [1.0, 0.8, 0.0, 1.0], // Golden
                special_ability: CharacterAbility::ScoreBooster,
                unlock_requirement: UnlockRequirement::AchieveScore(10000),
            },
        ]
    }
}

/// Asset loading and management
#[derive(Resource, Debug, Default)]
pub struct AssetHandles {
    /// Texture handles organized by category
    pub textures: HashMap<String, Handle<Image>>,
    /// Font handles
    pub fonts: HashMap<String, Handle<Font>>,
    /// Audio handles  
    pub audio: HashMap<String, Handle<AudioSource>>,
    /// Loading state
    pub loading_complete: bool,
    /// Loading progress (0.0 to 1.0)
    pub loading_progress: f32,
}

/// Game statistics and analytics
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameStatistics {
    /// Total food eaten across all games
    pub total_food_eaten: u32,
    /// Total distance traveled (grid units)
    pub total_distance: f32,
    /// Longest snake achieved
    pub longest_snake: u32,
    /// Fastest level completion times
    pub fastest_completions: [f32; 10],
    /// Death statistics by cause
    pub death_causes: HashMap<String, u32>,
    /// Play session data
    pub sessions_played: u32,
    /// Average session duration
    pub average_session_duration: f32,
}

/// Input state tracking
#[derive(Resource, Debug, Default)]
pub struct InputState {
    /// Current movement direction
    pub movement_direction: Option<Vec2>,
    /// Whether pause was just pressed
    pub pause_just_pressed: bool,
    /// Whether select was just pressed
    pub select_just_pressed: bool,
    /// Whether back was just pressed
    pub back_just_pressed: bool,
    /// Mouse position
    pub mouse_position: Vec2,
    /// Whether mouse was just clicked
    pub mouse_just_clicked: bool,
}

/// Save/Load system state
#[derive(Resource, Debug, Default)]
pub struct SaveLoadState {
    /// Whether save is in progress
    pub saving: bool,
    /// Whether load is in progress
    pub loading: bool,
    /// Last save timestamp
    pub last_save_time: f64,
    /// Auto-save interval (seconds)
    pub auto_save_interval: f64,
    /// Save file path (desktop) or storage key (web)
    pub save_location: String,
}