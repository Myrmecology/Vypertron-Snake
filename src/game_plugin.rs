use bevy::prelude::*;

// Import all our custom modules
pub mod states;
pub mod systems;
pub mod components;
pub mod resources;
pub mod levels;
pub mod audio;
pub mod utils;

// Re-export key types for easier access
pub use states::*;
pub use systems::*;
pub use components::*;
pub use resources::*;
pub use levels::*;
pub use audio::*;
pub use utils::*;

/// Main game plugin that orchestrates all Vypertron-Snake systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Initialize game states
        app.add_state::<GameState>()
            .add_state::<PauseState>();

        // Add events for our systems
        app.add_event::<crate::systems::FoodCollisionEvent>()
            .add_event::<crate::systems::WallCollisionEvent>()
            .add_event::<crate::systems::SelfCollisionEvent>()
            .add_event::<crate::systems::SpecialCollisionEvent>()
            .add_event::<crate::systems::SnakeGrowthEvent>()
            .add_event::<crate::systems::SnakeDeathEvent>()
            .add_event::<crate::systems::ExplosionEvent>()
            .add_event::<crate::systems::ParticleEvent>();
        app.insert_resource(HighScoreResource::default())
            .insert_resource(GameSettings::default())
            .insert_resource(LevelManager::default())
            .insert_resource(CharacterSelection::default())
            .insert_resource(GameTimer(Timer::from_seconds(0.0, TimerMode::Repeating)))
            .insert_resource(SnakeDirection::Right)
            .insert_resource(ScoreResource::default());

        // Audio system initialization
        app.add_plugins(AudioPlugin);

        // Register all our custom components
        app.register_type::<Snake>()
            .register_type::<SnakeSegment>()
            .register_type::<Food>()
            .register_type::<Wall>()
            .register_type::<MenuButton>()
            .register_type::<CharacterCard>()
            .register_type::<LevelBackground>()
            .register_type::<UIElement>()
            .register_type::<AnimatedSprite>()
            .register_type::<ExplosionEffect>()
            .register_type::<CutsceneElement>();

        // Home Screen Systems
        app.add_systems(OnEnter(GameState::HomeScreen), 
            (
                crate::systems::setup_home_screen,
                load_home_screen_assets,
                crate::audio::start_background_music,
            ).chain())
            .add_systems(Update, 
                (
                    crate::systems::animate_title_snake,
                    crate::systems::handle_home_screen_input,
                    crate::systems::update_menu_buttons,
                ).run_if(in_state(GameState::HomeScreen)));

        // Character Selection Systems  
        app.add_systems(OnEnter(GameState::CharacterSelect),
            (
                crate::systems::setup_character_selection,
                load_character_assets,
            ).chain())
            .add_systems(Update,
                (
                    crate::systems::handle_character_selection_input,
                    crate::systems::update_character_cards,
                    crate::systems::animate_character_previews,
                ).run_if(in_state(GameState::CharacterSelect)));

        // Gameplay Systems
        app.add_systems(OnEnter(GameState::Playing),
            (
                setup_game_level,
                crate::systems::spawn_snake,
                crate::systems::spawn_initial_food,
                crate::systems::setup_ui_elements,
                load_level_assets,
            ).chain())
            .add_systems(Update,
                (
                    crate::systems::handle_input,
                    crate::systems::move_snake,
                    crate::systems::check_food_collision,
                    crate::systems::check_wall_collision,
                    crate::systems::check_self_collision,
                    crate::systems::grow_snake,
                    update_score,
                    update_game_timer,
                    crate::systems::animate_snake,
                    crate::systems::update_ui,
                    crate::systems::handle_pause_input,
                ).run_if(in_state(GameState::Playing).and_then(in_state(PauseState::Unpaused))));

        // Pause Systems
        app.add_systems(Update,
                (
                    crate::systems::handle_pause_input,
                    crate::systems::display_pause_menu,
                ).run_if(in_state(GameState::Playing).and_then(in_state(PauseState::Paused))));

        // Game Over Systems
        app.add_systems(OnEnter(GameState::GameOver),
            (
                crate::systems::trigger_death_explosion,
                update_high_score,
                crate::systems::setup_game_over_screen,
            ).chain())
            .add_systems(Update,
                (
                    crate::systems::animate_explosion_effects,
                    handle_game_over_input,
                    update_game_over_ui,
                ).run_if(in_state(GameState::GameOver)));

        // Level Complete Systems
        app.add_systems(OnEnter(GameState::LevelComplete),
            (
                save_level_progress,
                crate::systems::setup_level_complete_screen,
                play_victory_sound,
            ).chain())
            .add_systems(Update,
                (
                    handle_level_complete_input,
                    animate_victory_effects,
                ).run_if(in_state(GameState::LevelComplete)));

        // Cutscene Systems
        app.add_systems(OnEnter(GameState::Cutscene),
            (
                setup_cutscene,
                load_cutscene_assets,
            ).chain())
            .add_systems(Update,
                (
                    update_cutscene,
                    handle_cutscene_input,
                    animate_cutscene_elements,
                ).run_if(in_state(GameState::Cutscene)));

        // Level transition and cleanup systems
        app.add_systems(OnExit(GameState::Playing), (
                cleanup_game_level,
                crate::systems::cleanup_food,
                crate::systems::cleanup_effects,
                crate::systems::cleanup_ui_elements,
            ))
            .add_systems(OnExit(GameState::HomeScreen), crate::systems::cleanup_home_screen)
            .add_systems(OnExit(GameState::CharacterSelect), crate::systems::cleanup_character_selection)
            .add_systems(OnExit(GameState::GameOver), crate::systems::cleanup_game_over)
            .add_systems(OnExit(GameState::Cutscene), cleanup_cutscene);

        // Global systems that run regardless of state
        app.add_systems(Update,
            (
                update_audio_system,
                crate::systems::handle_window_resize,
                crate::systems::update_animations,
                crate::systems::save_game_data,
                // Additional effect systems
                crate::systems::handle_explosion_events,
                crate::systems::handle_particle_events,
                crate::systems::update_particle_effects,
                crate::systems::update_shockwave_rings,
                crate::systems::update_delayed_effects,
                // Food systems
                crate::systems::spawn_food_system,
                crate::systems::animate_food,
                crate::systems::update_food_expiration,
                crate::systems::update_moving_food,
                // UI systems
                crate::systems::update_popup_notifications,
                crate::systems::update_animated_text,
                crate::systems::cleanup_pause_menu,
                // Effect systems
                crate::systems::update_speed_boost_effects,
                crate::systems::update_invincibility_effects,
                crate::systems::handle_collision_responses,
            ));

        // Startup systems
        app.add_systems(Startup,
            (
                crate::systems::load_global_assets,
                initialize_audio_system,
                crate::systems::setup_camera,
                crate::systems::load_saved_data,
                crate::systems::initialize_food_system,
                crate::systems::initialize_effects_system,
                crate::systems::initialize_input_system,
            ).chain());
    }
}

/// Main game states for Vypertron-Snake
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    HomeScreen,
    CharacterSelect,
    Playing,
    Paused,
    GameOver,
    LevelComplete,
    Cutscene,
    Settings,
}

/// Pause state management
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

/// Snake movement direction
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
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

/// Game timer resource for timing-based mechanics
#[derive(Resource)]
pub struct GameTimer(pub Timer);

/// Current score tracking
#[derive(Resource, Default)]
pub struct ScoreResource {
    pub current_score: u32,
    pub level_score: u32,
    pub food_eaten: u32,
    pub time_bonus: u32,
}

// System function stubs - we'll implement these in subsequent files
fn setup_home_screen() { /* Implementation in systems/home_screen.rs */ }
fn load_home_screen_assets() { /* Implementation in systems/asset_loading.rs */ }
fn start_background_music() { /* Implementation in audio/music_system.rs */ }
fn animate_title_snake() { /* Implementation in systems/animations.rs */ }
fn handle_home_screen_input() { /* Implementation in systems/input.rs */ }
fn update_menu_buttons() { /* Implementation in systems/ui.rs */ }
fn setup_character_selection() { /* Implementation in systems/character_select.rs */ }
fn load_character_assets() { /* Implementation in systems/asset_loading.rs */ }
fn handle_character_selection_input() { /* Implementation in systems/input.rs */ }
fn update_character_cards() { /* Implementation in systems/ui.rs */ }
fn animate_character_previews() { /* Implementation in systems/animations.rs */ }
fn setup_game_level() { /* Implementation in systems/level_setup.rs */ }
fn spawn_snake() { /* Implementation in systems/snake.rs */ }
fn spawn_initial_food() { /* Implementation in systems/food.rs */ }
fn setup_ui_elements() { /* Implementation in systems/ui.rs */ }
fn load_level_assets() { /* Implementation in systems/asset_loading.rs */ }
fn handle_input() { /* Implementation in systems/input.rs */ }
fn move_snake() { /* Implementation in systems/snake.rs */ }
fn check_food_collision() { /* Implementation in systems/collision.rs */ }
fn check_wall_collision() { /* Implementation in systems/collision.rs */ }
fn check_self_collision() { /* Implementation in systems/collision.rs */ }
fn grow_snake() { /* Implementation in systems/snake.rs */ }
fn update_score() { /* Implementation in systems/scoring.rs */ }
fn update_game_timer() { /* Implementation in systems/timer.rs */ }
fn animate_sprites() { /* Implementation in systems/animations.rs */ }
fn update_ui() { /* Implementation in systems/ui.rs */ }
fn handle_pause_input() { /* Implementation in systems/input.rs */ }
fn display_pause_menu() { /* Implementation in systems/ui.rs */ }
fn trigger_death_explosion() { /* Implementation in systems/effects.rs */ }
fn update_high_score() { /* Implementation in systems/scoring.rs */ }
fn setup_game_over_screen() { /* Implementation in systems/game_over.rs */ }
fn animate_explosion_effects() { /* Implementation in systems/effects.rs */ }
fn handle_game_over_input() { /* Implementation in systems/input.rs */ }
fn update_game_over_ui() { /* Implementation in systems/ui.rs */ }
fn save_level_progress() { /* Implementation in systems/save_system.rs */ }
fn setup_level_complete_screen() { /* Implementation in systems/level_complete.rs */ }
fn play_victory_sound() { /* Implementation in audio/sfx_system.rs */ }
fn handle_level_complete_input() { /* Implementation in systems/input.rs */ }
fn animate_victory_effects() { /* Implementation in systems/effects.rs */ }
fn setup_cutscene() { /* Implementation in systems/cutscene.rs */ }
fn load_cutscene_assets() { /* Implementation in systems/asset_loading.rs */ }
fn update_cutscene() { /* Implementation in systems/cutscene.rs */ }
fn handle_cutscene_input() { /* Implementation in systems/input.rs */ }
fn animate_cutscene_elements() { /* Implementation in systems/animations.rs */ }
fn cleanup_game_level() { /* Implementation in systems/cleanup.rs */ }
fn cleanup_home_screen() { /* Implementation in systems/cleanup.rs */ }
fn cleanup_character_selection() { /* Implementation in systems/cleanup.rs */ }
fn cleanup_game_over() { /* Implementation in systems/cleanup.rs */ }
fn cleanup_cutscene() { /* Implementation in systems/cleanup.rs */ }
fn update_audio_system() { /* Implementation in audio/audio_manager.rs */ }
fn handle_window_resize() { /* Implementation in systems/window.rs */ }
fn update_animations() { /* Implementation in systems/animations.rs */ }
fn save_game_data() { /* Implementation in systems/save_system.rs */ }
fn load_global_assets() { /* Implementation in systems/asset_loading.rs */ }
fn initialize_audio_system() { /* Implementation in audio/audio_manager.rs */ }
fn setup_camera() { /* Implementation in systems/camera.rs */ }
fn load_saved_data() { /* Implementation in systems/save_system.rs */ }