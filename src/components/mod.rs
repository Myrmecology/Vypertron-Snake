//! ECS Components for Vypertron-Snake
//! 
//! This module defines all the components used in the game's Entity-Component-System architecture.
//! Components are pure data containers that define what an entity is or has.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ===============================
// CORE GAME COMPONENTS
// ===============================

/// The main snake entity - represents the player
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct Snake {
    /// Current movement direction
    pub direction: Vec2,
    /// Movement speed (grid units per second)
    pub speed: f32,
    /// Time since last movement
    pub move_timer: f32,
    /// Snake length (number of segments)
    pub length: u32,
    /// Whether the snake is alive
    pub is_alive: bool,
    /// Selected character ID (1-4)
    pub character_id: u32,
    /// Current level
    pub level: u32,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0), // Start moving right
            speed: 5.0, // 5 moves per second
            move_timer: 0.0,
            length: 3, // Start with 3 segments
            is_alive: true,
            character_id: 1, // Default to character 1
            level: 1,
        }
    }
}

/// Individual snake body segments
#[derive(Component, Debug, Clone, Reflect)]
pub struct SnakeSegment {
    /// Position in the snake (0 = head, higher = further back)
    pub segment_index: u32,
    /// Grid position
    pub grid_position: Vec2,
    /// Visual scale for animations
    pub scale: f32,
    /// Rotation for smooth turning animations
    pub rotation: f32,
}

/// Food items that the snake can eat
#[derive(Component, Debug, Clone, Reflect)]
pub struct Food {
    /// Grid position where food spawned
    pub grid_position: Vec2,
    /// Score value when eaten
    pub score_value: u32,
    /// Type of food (normal, bonus, special)
    pub food_type: FoodType,
    /// Time until food expires (for special foods)
    pub expiration_timer: Option<f32>,
    /// Pulsing animation phase
    pub pulse_phase: f32,
}

/// Different types of food with varying effects
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum FoodType {
    /// Normal food - basic score
    Normal,
    /// Bonus food - extra score
    Bonus,
    /// Speed food - temporarily increases snake speed
    Speed,
    /// Golden food - massive score bonus
    Golden,
}

/// Wall and obstacle components
#[derive(Component, Debug, Clone, Reflect)]
pub struct Wall {
    /// Grid position
    pub grid_position: Vec2,
    /// Type of wall (boundary, obstacle, breakable)
    pub wall_type: WallType,
    /// Health for breakable walls
    pub health: u32,
}

/// Different wall types for level variety
#[derive(Debug, Clone, Reflect)]
pub enum WallType {
    /// Boundary walls (indestructible)
    Boundary,
    /// Static obstacles
    Obstacle,
    /// Breakable walls that can be destroyed
    Breakable,
    /// Moving obstacles
    Moving,
}

// ===============================
// UI COMPONENTS
// ===============================

/// Menu button component
#[derive(Component, Debug, Clone, Reflect)]
pub struct MenuButton {
    /// Button action when clicked/activated
    pub action: ButtonAction,
    /// Current button state
    pub state: ButtonState,
    /// Hover animation timer
    pub hover_timer: f32,
    /// Text to display on button
    pub text: String,
}

/// Actions that buttons can perform
#[derive(Debug, Clone, Reflect)]
pub enum ButtonAction {
    StartGame,
    SelectCharacter(u32),
    PauseGame,
    ResumeGame,
    RestartLevel,
    QuitToMenu,
    OpenSettings,
    CloseSettings,
    NextLevel,
    ShowCredits,
}

/// Button visual states
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

/// Character selection card component
#[derive(Component, Debug, Clone, Reflect)]
pub struct CharacterCard {
    /// Character ID (1-4)
    pub character_id: u32,
    /// Character name
    pub name: String,
    /// Character description
    pub description: String,
    /// Character color theme
    pub color: Color,
    /// Whether this character is selected
    pub is_selected: bool,
    /// Preview animation timer
    pub animation_timer: f32,
    /// Unlock status
    pub is_unlocked: bool,
}

/// UI text element with special properties
#[derive(Component, Debug, Clone, Reflect)]
pub struct UIElement {
    /// Type of UI element
    pub element_type: UIElementType,
    /// Animation properties
    pub animation: Option<UIAnimation>,
    /// Whether element is visible
    pub is_visible: bool,
    /// Layer/depth for rendering order
    pub layer: u32,
}

/// Types of UI elements
#[derive(Debug, Clone, Reflect)]
pub enum UIElementType {
    /// Score display
    Score,
    /// High score display
    HighScore,
    /// Level indicator
    Level,
    /// Timer display
    Timer,
    /// Lives/health display
    Lives,
    /// Pause menu overlay
    PauseMenu,
    /// Game over screen
    GameOverScreen,
    /// Level complete screen
    LevelCompleteScreen,
    /// Title text
    Title,
    /// Subtitle text
    Subtitle,
    /// Instructions text
    Instructions,
}

/// UI animation properties
#[derive(Debug, Clone, Reflect)]
pub struct UIAnimation {
    /// Animation type
    pub animation_type: UIAnimationType,
    /// Animation timer
    pub timer: f32,
    /// Animation duration
    pub duration: f32,
    /// Whether animation loops
    pub loops: bool,
}

/// Types of UI animations
#[derive(Debug, Clone, Reflect)]
pub enum UIAnimationType {
    FadeIn,
    FadeOut,
    Pulse,
    Bounce,
    Shake,
    Slide,
}

// ===============================
// VISUAL EFFECT COMPONENTS
// ===============================

/// Animated sprite component
#[derive(Component, Debug, Clone, Reflect)]
pub struct AnimatedSprite {
    /// Current frame index
    pub current_frame: u32,
    /// Total number of frames
    pub frame_count: u32,
    /// Time per frame (seconds)
    pub frame_duration: f32,
    /// Timer for current frame
    pub frame_timer: f32,
    /// Whether animation loops
    pub loops: bool,
    /// Whether animation is playing
    pub is_playing: bool,
}

/// Explosion effect component (for snake death)
#[derive(Component, Debug, Clone, Reflect)]
pub struct ExplosionEffect {
    /// Explosion intensity (0.0 to 1.0)
    pub intensity: f32,
    /// Explosion duration
    pub duration: f32,
    /// Current timer
    pub timer: f32,
    /// Particle count
    pub particle_count: u32,
    /// Explosion type
    pub explosion_type: ExplosionType,
}

/// Types of explosions
#[derive(Debug, Clone, Reflect)]
pub enum ExplosionType {
    /// Snake death explosion
    Death,
    /// Food pickup burst
    FoodPickup,
    /// Wall destruction
    WallBreak,
    /// Level complete celebration
    Victory,
}

/// Particle component for visual effects
#[derive(Component, Debug, Clone, Reflect)]
pub struct Particle {
    /// Particle velocity
    pub velocity: Vec2,
    /// Particle lifetime
    pub lifetime: f32,
    /// Current age
    pub age: f32,
    /// Start color
    pub start_color: Color,
    /// End color
    pub end_color: Color,
    /// Start scale
    pub start_scale: f32,
    /// End scale
    pub end_scale: f32,
}

// ===============================
// LEVEL COMPONENTS
// ===============================

/// Level background component
#[derive(Component, Debug, Clone, Reflect)]
pub struct LevelBackground {
    /// Level number (1-10)
    pub level: u32,
    /// Background theme
    pub theme: LevelTheme,
    /// Scrolling animation offset
    pub scroll_offset: Vec2,
    /// Scroll speed
    pub scroll_speed: Vec2,
    /// Parallax layers
    pub parallax_layers: Vec<ParallaxLayer>,
}

/// Level themes for visual variety
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum LevelTheme {
    /// Classic green theme
    Classic,
    /// Digital/cyber theme
    Digital,
    /// Forest theme
    Forest,
    /// Desert theme
    Desert,
    /// Ocean theme
    Ocean,
    /// Volcano theme
    Volcano,
    /// Ice theme
    Ice,
    /// Space theme
    Space,
    /// Neon city theme
    NeonCity,
    /// Final boss theme
    FinalBoss,
}

/// Parallax layer for background depth
#[derive(Debug, Clone, Reflect)]
pub struct ParallaxLayer {
    /// Layer depth (higher = further back)
    pub depth: f32,
    /// Scroll multiplier
    pub scroll_multiplier: f32,
    /// Layer opacity
    pub opacity: f32,
}

// ===============================
// CUTSCENE COMPONENTS
// ===============================

/// Cutscene element component
#[derive(Component, Debug, Clone, Reflect)]
pub struct CutsceneElement {
    /// Element type
    pub element_type: CutsceneElementType,
    /// Animation timeline
    pub timeline: CutsceneTimeline,
    /// Whether element is active
    pub is_active: bool,
}

/// Types of cutscene elements
#[derive(Debug, Clone, Reflect)]
pub enum CutsceneElementType {
    /// Background image
    Background,
    /// Character sprite
    Character,
    /// Text dialog
    Dialog,
    /// Sound effect trigger
    SoundEffect,
    /// Music trigger
    Music,
    /// Transition effect
    Transition,
}

/// Cutscene timeline for scripted events
#[derive(Debug, Clone, Reflect)]
pub struct CutsceneTimeline {
    /// Start time (seconds from cutscene start)
    pub start_time: f32,
    /// Duration (seconds)
    pub duration: f32,
    /// Animation curve
    pub easing: EasingType,
}

/// Easing types for smooth animations
#[derive(Debug, Clone, Reflect)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

// ===============================
// SPECIAL COMPONENTS
// ===============================

/// Component for the animated title snake on home screen
#[derive(Component, Debug, Clone, Reflect)]
pub struct TitleSnake {
    /// Animation path points
    pub path_points: Vec<Vec2>,
    /// Current position on path (0.0 to 1.0)
    pub path_position: f32,
    /// Animation speed
    pub animation_speed: f32,
    /// Number of segments to draw
    pub segment_count: u32,
}

/// Grid position component for snap-to-grid entities
#[derive(Component, Debug, Clone, Reflect)]
pub struct GridPosition {
    /// X coordinate on game grid
    pub x: i32,
    /// Y coordinate on game grid
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn to_world_position(&self, grid_size: f32) -> Vec2 {
        Vec2::new(self.x as f32 * grid_size, self.y as f32 * grid_size)
    }
    
    pub fn from_world_position(world_pos: Vec2, grid_size: f32) -> Self {
        Self {
            x: (world_pos.x / grid_size).round() as i32,
            y: (world_pos.y / grid_size).round() as i32,
        }
    }
}

/// Movement component for entities that move smoothly between grid positions
#[derive(Component, Debug, Clone, Reflect)]
pub struct SmoothMovement {
    /// Starting position
    pub start_position: Vec2,
    /// Target position
    pub target_position: Vec2,
    /// Movement progress (0.0 to 1.0)
    pub progress: f32,
    /// Movement duration
    pub duration: f32,
    /// Easing type
    pub easing: EasingType,
}

/// Audio trigger component for positional sound effects
#[derive(Component, Debug, Clone, Reflect)]
pub struct AudioTrigger {
    /// Sound to play
    pub sound_id: String,
    /// Whether sound has been triggered
    pub triggered: bool,
    /// Trigger condition
    pub trigger_condition: AudioTriggerCondition,
}

/// Conditions for triggering audio
#[derive(Debug, Clone, Reflect)]
pub enum AudioTriggerCondition {
    /// Trigger immediately when component is added
    Immediate,
    /// Trigger when snake enters this grid position
    SnakeEnters,
    /// Trigger when food is eaten at this position
    FoodEaten,
    /// Trigger when wall is hit
    WallHit,
    /// Trigger after a delay
    Delayed(f32),
}

// ===============================
// HELPER IMPLEMENTATIONS
// ===============================

impl Default for Food {
    fn default() -> Self {
        Self {
            grid_position: Vec2::ZERO,
            score_value: 10,
            food_type: FoodType::Normal,
            expiration_timer: None,
            pulse_phase: 0.0,
        }
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self {
            action: ButtonAction::StartGame,
            state: ButtonState::Normal,
            hover_timer: 0.0,
            text: "Button".to_string(),
        }
    }
}

impl Default for UIElement {
    fn default() -> Self {
        Self {
            element_type: UIElementType::Score,
            animation: None,
            is_visible: true,
            layer: 0,
        }
    }
}

impl Default for AnimatedSprite {
    fn default() -> Self {
        Self {
            current_frame: 0,
            frame_count: 1,
            frame_duration: 0.1,
            frame_timer: 0.0,
            loops: true,
            is_playing: true,
        }
    }
}