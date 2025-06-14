use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ===============================
// CORE GAME COMPONENTS
// ===============================

#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct Snake {
    pub direction: Vec2,
    pub speed: f32,
    pub move_timer: f32,
    pub length: u32,
    pub is_alive: bool,
    pub character_id: u32,
    pub level: u32,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0),
            speed: 5.0,
            move_timer: 0.0,
            length: 3,
            is_alive: true,
            character_id: 1,
            level: 1,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct SnakeSegment {
    pub segment_index: u32,
    pub grid_position: Vec2,
    pub scale: f32,
    pub rotation: f32,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct Food {
    pub grid_position: Vec2,
    pub score_value: u32,
    pub food_type: FoodType,
    pub expiration_timer: Option<f32>,
    pub pulse_phase: f32,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum FoodType {
    Normal,
    Bonus,
    Speed,
    Golden,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct Wall {
    pub grid_position: Vec2,
    pub wall_type: WallType,
    pub health: u32,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
pub enum WallType {
    Boundary,
    Obstacle,
    Breakable,
    Moving,
}

// ===============================
// UI COMPONENTS
// ===============================

#[derive(Component, Debug, Clone, Reflect)]
pub struct MenuButton {
    pub action: ButtonAction,
    pub state: ButtonState,
    pub hover_timer: f32,
    pub text: String,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
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

#[derive(Debug, Clone, Reflect, PartialEq)]
#[reflect_value]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct CharacterCard {
    pub character_id: u32,
    pub name: String,
    pub description: String,
    pub color: Color,
    pub is_selected: bool,
    pub animation_timer: f32,
    pub is_unlocked: bool,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct UIElement {
    pub element_type: UIElementType,
    pub animation: Option<UIAnimation>,
    pub is_visible: bool,
    pub layer: u32,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
pub enum UIElementType {
    Score,
    HighScore,
    Level,
    Timer,
    Lives,
    PauseMenu,
    GameOverScreen,
    LevelCompleteScreen,
    Title,
    Subtitle,
    Instructions,
}

#[derive(Debug, Clone, Reflect)]
pub struct UIAnimation {
    pub animation_type: UIAnimationType,
    pub timer: f32,
    pub duration: f32,
    pub loops: bool,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
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

#[derive(Component, Debug, Clone, Reflect)]
pub struct AnimatedSprite {
    pub current_frame: u32,
    pub frame_count: u32,
    pub frame_duration: f32,
    pub frame_timer: f32,
    pub loops: bool,
    pub is_playing: bool,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct ExplosionEffect {
    pub intensity: f32,
    pub duration: f32,
    pub timer: f32,
    pub particle_count: u32,
    pub explosion_type: ExplosionType,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
pub enum ExplosionType {
    Death,
    FoodPickup,
    WallBreak,
    Victory,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub age: f32,
    pub start_color: Color,
    pub end_color: Color,
    pub start_scale: f32,
    pub end_scale: f32,
}

// ===============================
// LEVEL COMPONENTS
// ===============================

#[derive(Component, Debug, Clone, Reflect)]
pub struct LevelBackground {
    pub level: u32,
    pub theme: LevelTheme,
    pub scroll_offset: Vec2,
    pub scroll_speed: Vec2,
    pub parallax_layers: Vec<ParallaxLayer>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum LevelTheme {
    Classic,
    Digital,
    Forest,
    Desert,
    Ocean,
    Volcano,
    Ice,
    Space,
    NeonCity,
    FinalBoss,
}

#[derive(Debug, Clone, Reflect)]
pub struct ParallaxLayer {
    pub depth: f32,
    pub scroll_multiplier: f32,
    pub opacity: f32,
}

// ===============================
// CUTSCENE COMPONENTS
// ===============================

#[derive(Component, Debug, Clone, Reflect)]
pub struct CutsceneElement {
    pub element_type: CutsceneElementType,
    pub timeline: CutsceneTimeline,
    pub is_active: bool,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
pub enum CutsceneElementType {
    Background,
    Character,
    Dialog,
    SoundEffect,
    Music,
    Transition,
}

#[derive(Debug, Clone, Reflect)]
pub struct CutsceneTimeline {
    pub start_time: f32,
    pub duration: f32,
    pub easing: EasingType,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
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

#[derive(Component, Debug, Clone, Reflect)]
pub struct TitleSnake {
    pub path_points: Vec<Vec2>,
    pub path_position: f32,
    pub animation_speed: f32,
    pub segment_count: u32,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct GridPosition {
    pub x: i32,
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

#[derive(Component, Debug, Clone, Reflect)]
pub struct SmoothMovement {
    pub start_position: Vec2,
    pub target_position: Vec2,
    pub progress: f32,
    pub duration: f32,
    pub easing: EasingType,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct AudioTrigger {
    pub sound_id: String,
    pub triggered: bool,
    pub trigger_condition: AudioTriggerCondition,
}

#[derive(Debug, Clone, Reflect)]
#[reflect_value]
pub enum AudioTriggerCondition {
    Immediate,
    SnakeEnters,
    FoodEaten,
    WallHit,
    Delayed(f32),
}

// ===============================
// DEFAULT IMPLEMENTATIONS
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

