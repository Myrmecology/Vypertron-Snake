//! Input System for Vypertron-Snake
//! 
//! This module handles all input processing including:
//! - Snake movement (arrow keys)
//! - Pause/Resume (spacebar)
//! - Menu navigation
//! - Character selection
//! - Smooth input buffering and validation

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::StateTransitionEvent; // Import our custom StateTransitionEvent specifically
use crate::states::*;
use crate::utils::*;
use crate::audio::*;

// ===============================
// INPUT COMPONENTS
// ===============================

/// Component for entities that can receive input
#[derive(Component, Debug)]
pub struct InputReceiver {
    /// Whether this entity accepts input in current state
    pub accepts_input: bool,
    /// Input priority (higher = processed first)
    pub priority: u32,
}

/// Input buffer for smooth movement
#[derive(Resource, Debug)]
pub struct InputBuffer {
    /// Buffered direction inputs
    pub direction_buffer: Vec<Vec2>,
    /// Maximum buffer size
    pub max_buffer_size: usize,
    /// Time each input stays in buffer
    pub buffer_duration: f32,
    /// Timer for buffer cleanup
    pub buffer_timer: f32,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            direction_buffer: Vec::new(),
            max_buffer_size: 3,
            buffer_duration: 0.2, // 200ms buffer
            buffer_timer: 0.0,
        }
    }
}

/// Input validation state
#[derive(Resource, Debug, Default)]
pub struct InputValidation {
    /// Last valid direction input
    pub last_direction: Option<Vec2>,
    /// Time of last direction input
    pub last_input_time: f32,
    /// Minimum time between direction changes
    pub min_input_interval: f32,
    /// Whether rapid input is allowed
    pub allow_rapid_input: bool,
}

// ===============================
// CORE INPUT SYSTEMS
// ===============================

/// Main input handling system - delegates to specific input handlers
pub fn handle_input(
    game_state: Res<State<GameState>>,
    pause_state: Res<State<PauseState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED: Input<KeyCode> -> ButtonInput<KeyCode>
    _mouse_input: Res<ButtonInput<MouseButton>>, // FIXED: Input<MouseButton> -> ButtonInput<MouseButton>
    time: Res<Time>,
    mut input_buffer: ResMut<InputBuffer>,
    mut input_validation: ResMut<InputValidation>,
    game_settings: Res<GameSettings>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    mut state_events: EventWriter<StateTransitionEvent>,
) {
    // Update input timing
    input_validation.last_input_time += time.delta_seconds();
    
    // Handle global inputs first (pause, etc.)
    handle_global_input(
        &keyboard_input,
        &game_state,
        &pause_state,
        &game_settings,
        &mut state_events,
        &mut play_sound_events,
    );
    
    // Handle state-specific input
    match game_state.get() {
        GameState::HomeScreen => {
            // Home screen input handled in systems/mod.rs
        },
        GameState::CharacterSelect => {
            // Character selection input handled in systems/mod.rs
        },
        GameState::Playing => {
            if pause_state.get() == &PauseState::Unpaused {
                handle_gameplay_input(
                    &keyboard_input,
                    &time,
                    &mut input_buffer,
                    &mut input_validation,
                    &game_settings,
                    &mut play_sound_events,
                );
            }
        },
        GameState::Paused => {
            handle_pause_menu_input(
                &keyboard_input,
                &game_settings,
                &mut state_events,
                &mut play_sound_events,
            );
        },
        GameState::GameOver => {
            handle_game_over_input(
                &keyboard_input,
                &game_settings,
                &mut state_events,
                &mut play_sound_events,
            );
        },
        GameState::LevelComplete => {
            handle_level_complete_input(
                &keyboard_input,
                &game_settings,
                &mut state_events,
                &mut play_sound_events,
            );
        },
        GameState::Cutscene => {
            handle_cutscene_input(
                &keyboard_input,
                &game_settings,
                &mut state_events,
                &mut play_sound_events,
            );
        },
        GameState::Settings => {
            handle_settings_input(
                &keyboard_input,
                &game_settings,
                &mut state_events,
                &mut play_sound_events,
            );
        },
        GameState::Loading => {
            // No input during loading
        },
        GameState::Credits => {
            handle_credits_input(
                &keyboard_input,
                &game_settings,
                &mut state_events,
                &mut play_sound_events,
            );
        },
    }
    
    // Update input buffer
    update_input_buffer(&time, &mut input_buffer);
}

/// Handle global inputs that work in any state
fn handle_global_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED: Input<KeyCode> -> ButtonInput<KeyCode>
    game_state: &Res<State<GameState>>,
    pause_state: &Res<State<PauseState>>,
    game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Global pause toggle (spacebar)
    if keyboard_input.just_pressed(game_settings.controls.pause) {
        match game_state.get() {
            GameState::Playing => {
                match pause_state.get() {
                    PauseState::Unpaused => {
                        play_sound_events.send(PlaySoundEvent::new("menu_select").with_volume(0.6));
                        state_events.send(StateTransitionEvent::PauseGame);
                    },
                    PauseState::Paused => {
                        play_sound_events.send(PlaySoundEvent::new("menu_select").with_volume(0.6));
                        state_events.send(StateTransitionEvent::ResumeGame);
                    },
                }
            },
            GameState::HomeScreen => {
                // Spacebar starts game from home screen
                play_sound_events.send(PlaySoundEvent::new("menu_select"));
                state_events.send(StateTransitionEvent::ToCharacterSelect);
            },
            _ => {
                // In other states, spacebar acts as select
                if matches!(game_state.get(), GameState::CharacterSelect | GameState::Settings | GameState::Credits) {
                    play_sound_events.send(PlaySoundEvent::new("menu_select"));
                }
            },
        }
    }
    
    // Global back/escape
    if keyboard_input.just_pressed(game_settings.controls.back) {
        match game_state.get() {
            GameState::CharacterSelect | GameState::Settings | GameState::Credits => {
                play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
                state_events.send(StateTransitionEvent::ToHomeScreen);
            },
            GameState::Playing if pause_state.get() == &PauseState::Paused => {
                play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
                state_events.send(StateTransitionEvent::QuitToMenu);
            },
            _ => {},
        }
    }
}

/// Handle gameplay input (snake movement)
fn handle_gameplay_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    _time: &Res<Time>,
    input_buffer: &mut ResMut<InputBuffer>,
    input_validation: &mut ResMut<InputValidation>,
    game_settings: &Res<GameSettings>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Get direction input
    if let Some(direction) = InputUtils::arrow_keys_to_direction(keyboard_input, &game_settings.controls) {
        // Validate input timing
        if input_validation.allow_rapid_input || 
           input_validation.last_input_time >= input_validation.min_input_interval {
            
            // Validate direction change (no 180-degree turns)
            let is_valid = if let Some(last_dir) = input_validation.last_direction {
                InputUtils::is_valid_direction_change(last_dir, direction)
            } else {
                true
            };
            
            if is_valid {
                // Add to input buffer
                add_direction_to_buffer(direction, input_buffer);
                
                // Update validation state
                input_validation.last_direction = Some(direction);
                input_validation.last_input_time = 0.0;
                
                // Play subtle movement sound
                play_sound_events.send(
                    PlaySoundEvent::new("snake_move")
                        .with_volume(0.3)
                        .with_pitch(1.0 + (direction.length() * 0.1))
                );
                
                info!("Direction input: {:?}", direction);
            } else {
                // Invalid direction - play error sound
                play_sound_events.send(
                    PlaySoundEvent::new("wall_hit")
                        .with_volume(0.2)
                        .with_pitch(2.0)
                );
                warn!("Invalid direction change blocked");
            }
        } else {
            // Input too rapid - ignored but not error sound
            debug!("Input ignored - too rapid");
        }
    }
    
    // Handle special ability activation (Enter key)
    if keyboard_input.just_pressed(game_settings.controls.select) {
        // Trigger character special ability
        play_sound_events.send(PlaySoundEvent::new("menu_select").with_volume(0.5));
        info!("Special ability activated");
    }
}

/// Add direction to input buffer with validation
fn add_direction_to_buffer(direction: Vec2, input_buffer: &mut InputBuffer) {
    // Don't add duplicate directions
    if let Some(last_direction) = input_buffer.direction_buffer.last() {
        if (*last_direction - direction).length() < 0.1 {
            return;
        }
    }
    
    // Add to buffer
    input_buffer.direction_buffer.push(direction);
    
    // Maintain buffer size limit
    if input_buffer.direction_buffer.len() > input_buffer.max_buffer_size {
        input_buffer.direction_buffer.remove(0);
    }
}

/// Update and clean input buffer
fn update_input_buffer(_time: &Res<Time>, input_buffer: &mut ResMut<InputBuffer>) {
    input_buffer.buffer_timer += _time.delta_seconds();
    
    // Clean old buffered inputs
    if input_buffer.buffer_timer >= input_buffer.buffer_duration {
        input_buffer.buffer_timer = 0.0;
        
        // Remove oldest input if buffer has multiple entries
        if input_buffer.direction_buffer.len() > 1 {
            input_buffer.direction_buffer.remove(0);
        }
    }
}

/// Handle pause menu input
pub fn handle_pause_menu_input( // FIXED: Added pub
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Resume with spacebar or enter
    if keyboard_input.just_pressed(game_settings.controls.pause) || 
       keyboard_input.just_pressed(game_settings.controls.select) {
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::ResumeGame);
    }
    
    // Restart level with R
    if keyboard_input.just_pressed(KeyCode::KeyR) { // FIXED: R -> KeyR
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::RestartLevel);
    }
    
    // Quit to menu with Q
    if keyboard_input.just_pressed(KeyCode::KeyQ) { // FIXED: Q -> KeyQ
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        state_events.send(StateTransitionEvent::QuitToMenu);
    }
}

/// Handle game over screen input
pub fn handle_game_over_input( // FIXED: Added pub
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Restart with spacebar or enter
    if keyboard_input.just_pressed(game_settings.controls.pause) || 
       keyboard_input.just_pressed(game_settings.controls.select) {
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::RestartLevel);
    }
    
    // Return to menu with escape
    if keyboard_input.just_pressed(game_settings.controls.back) {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        state_events.send(StateTransitionEvent::QuitToMenu);
    }
    
    // Show statistics with S
    if keyboard_input.just_pressed(KeyCode::KeyS) { // FIXED: S -> KeyS
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        // Could trigger statistics display
        info!("Show statistics requested");
    }
}

/// Handle level complete screen input
pub fn handle_level_complete_input( // FIXED: Added pub
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Continue to next level
    if keyboard_input.just_pressed(game_settings.controls.pause) || 
       keyboard_input.just_pressed(game_settings.controls.select) {
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::StartCutscene { 
            cutscene_type: CutsceneState::LevelTransition 
        });
    }
    
    // Replay level with R
    if keyboard_input.just_pressed(KeyCode::KeyR) { // FIXED: R -> KeyR
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::RestartLevel);
    }
    
    // Return to menu
    if keyboard_input.just_pressed(game_settings.controls.back) {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        state_events.send(StateTransitionEvent::QuitToMenu);
    }
}

/// Handle cutscene input
pub fn handle_cutscene_input( // FIXED: Added pub
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Skip cutscene with spacebar or enter
    if keyboard_input.just_pressed(game_settings.controls.pause) || 
       keyboard_input.just_pressed(game_settings.controls.select) {
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::EndCutscene);
    }
    
    // Skip with escape
    if keyboard_input.just_pressed(game_settings.controls.back) {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        state_events.send(StateTransitionEvent::EndCutscene);
    }
}

/// Handle settings menu input
pub fn handle_settings_input( // FIXED: Added pub
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Return from settings
    if keyboard_input.just_pressed(game_settings.controls.back) || 
       keyboard_input.just_pressed(game_settings.controls.select) {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        state_events.send(StateTransitionEvent::FromSettings);
    }
    
    // Navigation within settings (would be handled by specific settings UI)
    if keyboard_input.just_pressed(game_settings.controls.move_up) {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate").with_volume(0.5));
    }
    
    if keyboard_input.just_pressed(game_settings.controls.move_down) {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate").with_volume(0.5));
    }
}

/// Handle credits screen input
pub fn handle_credits_input( // FIXED: Added pub
    keyboard_input: &Res<ButtonInput<KeyCode>>, // FIXED
    _game_settings: &Res<GameSettings>,
    state_events: &mut EventWriter<StateTransitionEvent>,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Return from credits with any key
    if keyboard_input.get_just_pressed().next().is_some() {
        play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
        state_events.send(StateTransitionEvent::ToHomeScreen);
    }
}

// ===============================
// INPUT BUFFER SYSTEMS
// ===============================

/// System to consume buffered input for snake movement
pub fn consume_buffered_input(
    mut input_buffer: ResMut<InputBuffer>,
    mut snake_query: Query<&mut Snake>,
    mut snake_direction: ResMut<SnakeDirection>,
) {
    if let Some(direction) = input_buffer.direction_buffer.first().copied() {
        // Apply direction to snake
        for mut snake in snake_query.iter_mut() {
            snake.direction = direction;
            
            // FIXED: Update global direction resource with single dereference
            *snake_direction = if direction.x > 0.0 {
                SnakeDirection::Right
            } else if direction.x < 0.0 {
                SnakeDirection::Left
            } else if direction.y > 0.0 {
                SnakeDirection::Up
            } else {
                SnakeDirection::Down
            };
            
            // FIXED: Use single dereference for logging
            info!("Snake direction updated: {:?}", *snake_direction);
        }
        
        // Remove consumed input
        input_buffer.direction_buffer.remove(0);
    }
}

/// System to handle pause input specifically
pub fn handle_pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED
    game_state: Res<State<GameState>>,
    pause_state: Res<State<PauseState>>,
    game_settings: Res<GameSettings>,
    mut state_events: EventWriter<StateTransitionEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    // Only handle pause in playing state
    if game_state.get() != &GameState::Playing {
        return;
    }
    
    if keyboard_input.just_pressed(game_settings.controls.pause) {
        match pause_state.get() {
            PauseState::Unpaused => {
                play_sound_events.send(PlaySoundEvent::new("menu_select").with_volume(0.6));
                state_events.send(StateTransitionEvent::PauseGame);
                info!("Game paused");
            },
            PauseState::Paused => {
                play_sound_events.send(PlaySoundEvent::new("menu_select").with_volume(0.6));
                state_events.send(StateTransitionEvent::ResumeGame);
                info!("Game resumed");
            },
        }
    }
}

// ===============================
// INPUT FEEDBACK SYSTEMS
// ===============================

/// System to provide visual feedback for input
pub fn input_visual_feedback(
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: Res<GameSettings>,
    mut snake_query: Query<&mut Transform, With<Snake>>,
    time: Res<Time>,
) {
    // Subtle visual feedback for direction changes
    for mut transform in snake_query.iter_mut() {
        // Slight rotation when changing direction
        if keyboard_input.just_pressed(game_settings.controls.move_up) ||
           keyboard_input.just_pressed(game_settings.controls.move_down) ||
           keyboard_input.just_pressed(game_settings.controls.move_left) ||
           keyboard_input.just_pressed(game_settings.controls.move_right) {
            
            // Add tiny shake effect
            let shake_offset = (time.elapsed_seconds() * 50.0).sin() * 0.5;
            transform.translation.x += shake_offset;
        }
    }
}

/// System to provide haptic feedback (if supported)
pub fn input_haptic_feedback(
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED
    _game_settings: Res<GameSettings>,
    // Note: Bevy doesn't have built-in haptic support yet
    // This is a placeholder for future haptic feedback
) {
    // Placeholder for haptic feedback on direction changes
    if keyboard_input.get_just_pressed().next().is_some() {
        // Future: trigger controller rumble or haptic feedback
        debug!("Haptic feedback triggered");
    }
}

// ===============================
// ACCESSIBILITY INPUT SYSTEMS
// ===============================

/// System to handle accessibility input features
pub fn accessibility_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED
    game_settings: Res<GameSettings>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    // High contrast toggle
    if keyboard_input.just_pressed(KeyCode::KeyH) && // FIXED: H -> KeyH
       keyboard_input.pressed(KeyCode::ControlLeft) {
        if game_settings.accessibility.high_contrast {
            info!("High contrast mode disabled");
        } else {
            info!("High contrast mode enabled");
        }
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
    }
    
    // Audio cues toggle
    if keyboard_input.just_pressed(KeyCode::KeyA) && // FIXED: A -> KeyA
       keyboard_input.pressed(KeyCode::ControlLeft) {
        if game_settings.accessibility.audio_cues {
            info!("Audio cues disabled");
        } else {
            info!("Audio cues enabled");
        }
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
    }
    
    // Reduced motion toggle
    if keyboard_input.just_pressed(KeyCode::KeyM) && // FIXED: M -> KeyM
       keyboard_input.pressed(KeyCode::ControlLeft) {
        if game_settings.accessibility.reduced_motion {
            info!("Reduced motion disabled");
        } else {
            info!("Reduced motion enabled");
        }
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
    }
}

// ===============================
// DEBUG INPUT SYSTEMS
// ===============================

/// Debug input system for development
#[cfg(debug_assertions)]
pub fn debug_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED
    mut snake_query: Query<&mut Snake>,
    mut level_manager: ResMut<LevelManager>,
    mut state_events: EventWriter<StateTransitionEvent>,
) {
    // Debug controls (only in debug builds)
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        // Skip level with Ctrl+N
        if keyboard_input.just_pressed(KeyCode::KeyN) { // FIXED: N -> KeyN
            level_manager.current_level = (level_manager.current_level % 10) + 1;
            // FIXED: Use only character_id parameter
            state_events.send(StateTransitionEvent::StartGame { 
                character_id: 1
            });
            info!("Debug: Skipped to level {}", level_manager.current_level);
        }
        
        // Add snake length with Ctrl+L
        if keyboard_input.just_pressed(KeyCode::KeyL) { // FIXED: L -> KeyL
            for mut snake in snake_query.iter_mut() {
                snake.length += 5;
                info!("Debug: Snake length increased to {}", snake.length);
            }
        }
        
        // Toggle invincibility with Ctrl+I
        if keyboard_input.just_pressed(KeyCode::KeyI) { // FIXED: I -> KeyI
            for mut snake in snake_query.iter_mut() {
                snake.is_alive = true;
                info!("Debug: Snake invincibility toggled");
            }
        }
    }
}

// ===============================
// INPUT INITIALIZATION
// ===============================

/// Initialize input systems and resources
pub fn initialize_input_system(mut commands: Commands) {
    commands.insert_resource(InputBuffer::default());
    commands.insert_resource(InputValidation {
        last_direction: None,
        last_input_time: 0.0,
        min_input_interval: 0.1, // 100ms minimum between inputs
        allow_rapid_input: false,
    });
    
    info!("Input system initialized");
}