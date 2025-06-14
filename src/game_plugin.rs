use bevy::prelude::*;

// Import all our custom modules
use crate::states::*;
use crate::systems::*;
use crate::components::*;
use crate::resources::*;
use crate::levels::*;
use crate::audio::*;
use crate::utils::*;

/// Main game plugin that orchestrates all Vypertron-Snake systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Initialize game states
        app.add_state::<GameState>()
            .add_state::<PauseState>();

        // Add events for our systems
        app.add_event::<FoodCollisionEvent>()
            .add_event::<WallCollisionEvent>()
            .add_event::<SelfCollisionEvent>()
            .add_event::<SpecialCollisionEvent>()
            .add_event::<SnakeGrowthEvent>()
            .add_event::<SnakeDeathEvent>()
            .add_event::<ExplosionEvent>()
            .add_event::<ParticleEvent>();

        // Initialize game resources
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
                setup_home_screen,
                load_home_screen_assets,
                start_background_music,
            ).chain())
            .add_systems(Update, 
                (
                    animate_title_snake,
                    handle_home_screen_input,
                    update_menu_buttons,
                ).run_if(in_state(GameState::HomeScreen)));

        // Character Selection Systems  
        app.add_systems(OnEnter(GameState::CharacterSelect),
            (
                setup_character_selection,
                load_character_assets,
            ).chain())
            .add_systems(Update,
                (
                    handle_character_selection_input,
                    update_character_cards,
                    animate_character_previews,
                ).run_if(in_state(GameState::CharacterSelect)));

        // Gameplay Systems
        app.add_systems(OnEnter(GameState::Playing),
            (
                setup_game_level,
                spawn_snake,
                spawn_initial_food,
                setup_ui_elements,
                load_level_assets,
            ).chain())
            .add_systems(Update,
                (
                    handle_input,
                    move_snake,
                    check_food_collision,
                    check_wall_collision,
                    check_self_collision,
                    grow_snake,
                    update_score,
                    update_game_timer,
                    animate_snake,
                    update_ui,
                    handle_pause_input,
                ).run_if(in_state(GameState::Playing).and_then(in_state(PauseState::Unpaused))));

        // Pause Systems
        app.add_systems(Update,
                (
                    handle_pause_input,
                    display_pause_menu,
                ).run_if(in_state(GameState::Playing).and_then(in_state(PauseState::Paused))));

        // Game Over Systems
        app.add_systems(OnEnter(GameState::GameOver),
            (
                trigger_death_explosion,
                update_high_score,
                setup_game_over_screen,
            ).chain())
            .add_systems(Update,
                (
                    animate_explosion_effects,
                    handle_game_over_input,
                    update_game_over_ui,
                ).run_if(in_state(GameState::GameOver)));

        // Level Complete Systems
        app.add_systems(OnEnter(GameState::LevelComplete),
            (
                save_level_progress,
                setup_level_complete_screen,
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
                cleanup_food,
                cleanup_effects,
                cleanup_ui_elements,
            ))
            .add_systems(OnExit(GameState::HomeScreen), cleanup_home_screen)
            .add_systems(OnExit(GameState::CharacterSelect), cleanup_character_selection)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(OnExit(GameState::Cutscene), cleanup_cutscene);

        // Global systems that run regardless of state
        app.add_systems(Update,
            (
                update_audio_system,
                handle_window_resize,
                update_animations,
                save_game_data,
                // Additional effect systems
                handle_explosion_events,
                handle_particle_events,
                update_particle_effects,
                update_shockwave_rings,
                update_delayed_effects,
                // Food systems
                spawn_food_system,
                animate_food,
                update_food_expiration,
                update_moving_food,
                // UI systems
                update_popup_notifications,
                update_animated_text,
                cleanup_pause_menu,
                // Effect systems
                update_speed_boost_effects,
                update_invincibility_effects,
                handle_collision_responses,
            ));

        // Startup systems
        app.add_systems(Startup,
            (
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

// We'll implement these placeholder functions later
fn load_home_screen_assets() { /* Implementation needed */ }
fn load_character_assets() { /* Implementation needed */ }
fn setup_game_level() { /* Implementation needed */ }
fn load_level_assets() { /* Implementation needed */ }
fn update_score() { /* Implementation needed */ }
fn update_game_timer() { /* Implementation needed */ }
fn handle_game_over_input() { /* Implementation needed */ }
fn update_game_over_ui() { /* Implementation needed */ }
fn update_high_score() { /* Implementation needed */ }
fn save_level_progress() { /* Implementation needed */ }
fn play_victory_sound() { /* Implementation needed */ }
fn handle_level_complete_input() { /* Implementation needed */ }
fn animate_victory_effects() { /* Implementation needed */ }
fn setup_cutscene() { /* Implementation needed */ }
fn load_cutscene_assets() { /* Implementation needed */ }
fn update_cutscene() { /* Implementation needed */ }
fn handle_cutscene_input() { /* Implementation needed */ }
fn animate_cutscene_elements() { /* Implementation needed */ }
fn cleanup_game_level() { /* Implementation needed */ }
fn cleanup_cutscene() { /* Implementation needed */ }
fn update_audio_system() { /* Implementation needed */ }
fn initialize_audio_system() { /* Implementation needed */ }