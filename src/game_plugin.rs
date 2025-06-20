use bevy::prelude::*;

// Custom modules - IMPORTANT: Import order matters!
use crate::states::{StateTransitionEvent, GameState, PauseState, CharacterSelectState, CutsceneState, *}; // FIXED: Explicitly import our custom StateTransitionEvent and states first
use crate::systems::*;
use crate::systems::input::*;
use crate::systems::snake::*;
use crate::systems::collision::*;
use crate::components::*;
use crate::resources::*;
use crate::levels::*;
use crate::audio::*;
use crate::utils::*;

/// Main game plugin that orchestrates all Vypertron-Snake systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // === States (FIXED for Bevy 0.14) ===
        app.init_state::<GameState>()
            .add_sub_state::<PauseState>()
            .add_sub_state::<CharacterSelectState>()
            .add_sub_state::<CutsceneState>();

        // === Events ===
        app.add_event::<StateTransitionEvent>() // FIXED: Using our custom StateTransitionEvent
            .add_event::<FoodCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_event::<SelfCollisionEvent>()
            .add_event::<SpecialCollisionEvent>()
            .add_event::<SnakeGrowthEvent>()
            .add_event::<SnakeDeathEvent>()
            .add_event::<ExplosionEvent>()
            .add_event::<ParticleEvent>()
            .add_event::<PlaySoundEvent>(); // ADDED: Audio events

        // === Resources ===
        app.insert_resource(HighScoreResource::default())
            .insert_resource(GameSettings::default())
            .insert_resource(LevelManager::default())
            .insert_resource(CharacterSelection::default())
            .insert_resource(GameTimer(Timer::from_seconds(0.0, TimerMode::Repeating)))
            .insert_resource(SnakeDirection::Right) // FIXED: Now has Resource trait
            .insert_resource(ScoreResource::default())
            .insert_resource(PreviousGameState::default()) // ADDED: State management resource
            .insert_resource(GameProgression::default()) // ADDED: Game progression tracking
            .insert_resource(InputBuffer::default()) // ADDED: Input buffer
            .insert_resource(InputValidation::default()) // ADDED: Input validation
            .insert_resource(AssetHandles::default()); // ADDED: Asset management

        // === Core State Management System ===
        app.add_systems(Update, handle_state_transitions);

        // === Audio Plugin ===
        app.add_plugins(AudioPlugin);

        // === Register Components for Reflect/Editor use ===
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
            .register_type::<CutsceneElement>()
            .register_type::<GridPosition>()
            .register_type::<SmoothMovement>()
            .register_type::<InvincibilityEffect>()
            .register_type::<SpeedBoostEffect>()
            .register_type::<ScoreMultiplierEffect>();

        // === Home Screen ===
        app.add_systems(OnEnter(GameState::HomeScreen), (
                setup_home_screen,
                load_home_screen_assets,
                start_background_music,
            ).chain())
            .add_systems(Update, (
                animate_title_snake,
                handle_home_screen_input,
                update_menu_buttons,
            ).run_if(in_state(GameState::HomeScreen)));

        // === Character Select ===
        app.add_systems(OnEnter(GameState::CharacterSelect), (
                setup_character_selection,
                load_character_assets,
            ).chain())
            .add_systems(Update, (
                handle_character_selection_input,
                update_character_cards,
                animate_character_previews,
            ).run_if(in_state(GameState::CharacterSelect)));

        // === Loading State ===
        app.add_systems(OnEnter(GameState::Loading), (
                setup_loading_screen,
                start_level_loading,
            ).chain())
            .add_systems(Update, (
                update_loading_progress,
                check_loading_complete,
            ).run_if(in_state(GameState::Loading)));

        // === Gameplay ===
        app.add_systems(OnEnter(GameState::Playing), (
                setup_game_level,
                spawn_snake,
                spawn_initial_food,
                setup_ui_elements,
                load_level_assets,
            ).chain())
            .add_systems(Update, (
                // Input and Movement
                handle_input,
                consume_buffered_input,
                move_snake,
                
                // Collision Detection
                check_food_collision,
                check_wall_collision,
                check_self_collision,
                check_special_collisions,
                check_boundary_collision,
                
                // Game Logic
                grow_snake,
                update_score,
                update_game_timer,
                
                // Visual Updates
                animate_snake,
                update_smooth_movement,
                update_ui,
                
                // Effects
                update_speed_boost_effects,
                update_invincibility_effects,
                update_snake_trail,
                handle_collision_responses,
            ).run_if(
                in_state(GameState::Playing)
                .and_then(in_state(PauseState::Unpaused))
            ));

        // === Pause ===
        app.add_systems(Update, (
                handle_pause_input,
                display_pause_menu,
            ).run_if(
                in_state(GameState::Playing)
                .and_then(in_state(PauseState::Paused))
            ));

        // === Game Over ===
        app.add_systems(OnEnter(GameState::GameOver), (
                trigger_death_explosion,
                update_high_score,
                setup_game_over_screen,
            ).chain())
            .add_systems(Update, (
                animate_explosion_effects,
                handle_game_over_input,
                update_game_over_ui,
            ).run_if(in_state(GameState::GameOver)));

        // === Level Complete ===
        app.add_systems(OnEnter(GameState::LevelComplete), (
                save_level_progress,
                setup_level_complete_screen,
                play_victory_sound,
            ).chain())
            .add_systems(Update, (
                handle_level_complete_input,
                animate_victory_effects,
            ).run_if(in_state(GameState::LevelComplete)));

        // === Cutscenes ===
        app.add_systems(OnEnter(GameState::Cutscene), (
                setup_cutscene,
                load_cutscene_assets,
            ).chain())
            .add_systems(Update, (
                update_cutscene,
                handle_cutscene_input,
                animate_cutscene_elements,
            ).run_if(in_state(GameState::Cutscene)));

        // === Settings ===
        app.add_systems(OnEnter(GameState::Settings), (
                setup_settings_screen,
            ))
            .add_systems(Update, (
                handle_settings_input,
                update_settings_ui,
            ).run_if(in_state(GameState::Settings)));

        // === Credits ===
        app.add_systems(OnEnter(GameState::Credits), (
                setup_credits_screen,
            ))
            .add_systems(Update, (
                handle_credits_input,
                update_credits_scroll,
            ).run_if(in_state(GameState::Credits)));

        // === Cleanup ===
        app.add_systems(OnExit(GameState::Playing), (
                cleanup_game_level,
                cleanup_food,
                cleanup_effects,
                cleanup_ui_elements,
            ))
            .add_systems(OnExit(GameState::HomeScreen), cleanup_home_screen)
            .add_systems(OnExit(GameState::CharacterSelect), cleanup_character_selection)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(OnExit(GameState::Cutscene), cleanup_cutscene)
            .add_systems(OnExit(GameState::Settings), cleanup_settings)
            .add_systems(OnExit(GameState::Credits), cleanup_credits)
            .add_systems(OnExit(GameState::Loading), cleanup_loading);

        // === Global Systems (Run in all states) ===
        app.add_systems(Update, (
                update_audio_system,
                handle_window_resize,
                update_animations,
                save_game_data,
                handle_explosion_events,
                handle_particle_events,
                update_particle_effects,
                update_shockwave_rings,
                update_delayed_effects,
                update_popup_notifications,
                update_animated_text,
                input_visual_feedback,
                input_haptic_feedback,
                accessibility_input,
            ));

        // === Food Systems ===
        app.add_systems(Update, (
                spawn_food_system,
                animate_food,
                update_food_expiration,
                update_moving_food,
            ).run_if(in_state(GameState::Playing)));

        // === Debug Systems (Only in debug builds) ===
        #[cfg(debug_assertions)]
        app.add_systems(Update, debug_input);

        // === Startup ===
        app.add_systems(Startup, (
                load_global_assets,
                initialize_audio_system,
                setup_camera,
                load_saved_data,
                initialize_food_system,
                initialize_effects_system,
                initialize_input_system,
            ).chain());
    }
}

// === Placeholder Stubs ===
// These should match Bevy signatures so they compile cleanly

fn load_home_screen_assets() {
    info!("Loading home screen assets...");
}

fn load_character_assets() {
    info!("Loading character assets...");
}

fn setup_game_level() {
    info!("Setting up game level...");
}

fn load_level_assets() {
    info!("Loading level assets...");
}

fn setup_loading_screen() {
    info!("Setting up loading screen...");
}

fn start_level_loading() {
    info!("Starting level loading...");
}

fn update_loading_progress() {
    // Progress tracking logic would go here
}

fn check_loading_complete() {
    // Check if loading is complete and transition to playing
}

fn update_score() {
    // Score update logic
}

fn update_game_timer() {
    // Game timer logic
}

fn handle_game_over_input() {
    info!("Handling game over input...");
}

fn update_game_over_ui() {
    // Game over UI updates
}

fn update_high_score() {
    info!("Updating high score...");
}

fn save_level_progress() {
    info!("Saving level progress...");
}

fn play_victory_sound() {
    info!("Playing victory sound...");
}

fn handle_level_complete_input() {
    info!("Handling level complete input...");
}

fn animate_victory_effects() {
    // Victory animation logic
}

fn setup_cutscene() {
    info!("Setting up cutscene...");
}

fn load_cutscene_assets() {
    info!("Loading cutscene assets...");
}

fn update_cutscene() {
    // Cutscene update logic
}

fn handle_cutscene_input() {
    info!("Handling cutscene input...");
}

fn animate_cutscene_elements() {
    // Cutscene animation logic
}

fn setup_settings_screen() {
    info!("Setting up settings screen...");
}

fn update_settings_ui() {
    // Settings UI updates
}

fn setup_credits_screen() {
    info!("Setting up credits screen...");
}

fn update_credits_scroll() {
    // Credits scrolling logic
}

fn cleanup_game_level() {
    info!("Cleaning up game level...");
}

fn cleanup_cutscene() {
    info!("Cleaning up cutscene...");
}

fn cleanup_settings() {
    info!("Cleaning up settings...");
}

fn cleanup_credits() {
    info!("Cleaning up credits...");
}

fn cleanup_loading() {
    info!("Cleaning up loading screen...");
}

fn update_audio_system() {
    // Audio system updates
}

fn initialize_audio_system() {
    info!("Initializing audio system...");
}

fn trigger_death_explosion() {
    info!("Triggering death explosion...");
}

fn animate_explosion_effects() {
    // Explosion animation logic
}

fn display_pause_menu() {
    // Pause menu display logic
}

fn handle_explosion_events() {
    // Handle explosion events
}

fn handle_particle_events() {
    // Handle particle events
}

fn update_shockwave_rings() {
    // Shockwave ring animations
}

fn update_delayed_effects() {
    // Delayed effect processing
}

fn cleanup_pause_menu() {
    // Cleanup pause menu when not needed
}

fn handle_window_resize() {
    // Handle window resize events
}

fn update_animations() {
    // Global animation updates
}

fn save_game_data() {
    // Periodic game data saving
}

fn update_popup_notifications() {
    // Update popup notifications
}

fn update_animated_text() {
    // Update animated text elements
}

fn load_global_assets() {
    info!("Loading global assets...");
}

fn setup_camera() {
    info!("Setting up camera...");
}

fn load_saved_data() {
    info!("Loading saved data...");
}

fn initialize_food_system() {
    info!("Initializing food system...");
}

fn initialize_effects_system() {
    info!("Initializing effects system...");
}
