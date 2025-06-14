//! Game State Management for Vypertron-Snake
//! 
//! This module defines all the game states and provides utilities for state transitions.
//! The game uses a hierarchical state system where some states can be nested within others.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Main game states for Vypertron-Snake
/// 
/// The game flows through these states in a specific order:
/// HomeScreen → CharacterSelect → Playing → (LevelComplete | GameOver) → Cutscene → Playing (next level)
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, Serialize, Deserialize)]
pub enum GameState {
    /// Initial state - NES-style home screen with animated title snake
    #[default]
    HomeScreen,
    /// Character selection screen - choose from 4 unique snake characters
    CharacterSelect,
    /// Active gameplay state
    Playing,
    /// Game is paused (separate from PauseState for more granular control)
    Paused,
    /// Player died - explosion effects and score display
    GameOver,
    /// Level completed successfully - victory effects and progression
    LevelComplete,
    /// Story cutscenes between levels
    Cutscene,
    /// Settings menu (accessible from various states)
    Settings,
    /// Loading screen for asset loading and level transitions
    Loading,
    /// Credits screen
    Credits,
}

/// Pause state management - independent of main game state
/// 
/// This allows for more granular control over pause functionality
/// and can be used in combination with other states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum PauseState {
    /// Game is running normally
    #[default]
    Unpaused,
    /// Game is paused - show pause menu
    Paused,
}

/// Character selection state for tracking which character is being previewed
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum CharacterSelectState {
    /// Showing all characters
    #[default]
    Overview,
    /// Previewing Character 1 - Classic Green Snake
    Character1,
    /// Previewing Character 2 - Electric Blue Snake  
    Character2,
    /// Previewing Character 3 - Fire Red Snake
    Character3,
    /// Previewing Character 4 - Golden Snake
    Character4,
}

/// Cutscene state for managing different types of story sequences
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum CutsceneState {
    /// Intro cutscene (before first level)
    #[default]
    Intro,
    /// Level transition cutscenes
    LevelTransition,
    /// Final victory cutscene (after level 10)
    Victory,
    /// Game over story sequence
    GameOverStory,
}

/// State transition events for clean state management
#[derive(Event, Debug, Clone)]
pub enum StateTransitionEvent {
    /// Transition to home screen
    ToHomeScreen,
    /// Start character selection
    ToCharacterSelect,
    /// Start playing with selected character and level
    StartGame { character_id: u32, level: u32 },
    /// Pause the current game
    PauseGame,
    /// Resume the current game
    ResumeGame,
    /// Player died - transition to game over
    GameOver { final_score: u32 },
    /// Level completed - transition to next level or victory
    LevelComplete { score: u32, level: u32 },
    /// Start cutscene
    StartCutscene { cutscene_type: CutsceneState },
    /// End cutscene and proceed to next state
    EndCutscene,
    /// Open settings menu
    ToSettings,
    /// Close settings and return to previous state
    FromSettings,
    /// Show credits
    ToCredits,
    /// Restart current level
    RestartLevel,
    /// Quit to main menu
    QuitToMenu,
}

/// Resource for tracking previous state (for returning from settings, etc.)
#[derive(Resource, Debug, Clone, Default)]
pub struct PreviousGameState {
    pub state: Option<GameState>,
}

/// Resource for managing level progression
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameProgression {
    /// Current level (1-10)
    pub current_level: u32,
    /// Highest level unlocked
    pub max_unlocked_level: u32,
    /// Selected character ID (1-4)
    pub selected_character: u32,
    /// Whether we're in a new game or continuing
    pub is_new_game: bool,
    /// Total score across all levels
    pub total_score: u32,
    /// Scores per level
    pub level_scores: [u32; 10],
}

impl Default for GameProgression {
    fn default() -> Self {
        Self {
            current_level: 1,
            max_unlocked_level: 1,
            selected_character: 1,
            is_new_game: true,
            total_score: 0,
            level_scores: [0; 10],
        }
    }
}

/// State transition system - handles all state change events
pub fn handle_state_transitions(
    mut commands: Commands,
    mut state_events: EventReader<StateTransitionEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<NextState<PauseState>>,
    mut character_state: ResMut<NextState<CharacterSelectState>>,
    mut cutscene_state: ResMut<NextState<CutsceneState>>,
    mut previous_state: ResMut<PreviousGameState>,
    mut progression: ResMut<GameProgression>,
    current_state: Res<State<GameState>>,
) {
    for event in state_events.read() {
        match event {
            StateTransitionEvent::ToHomeScreen => {
                game_state.set(GameState::HomeScreen);
                pause_state.set(PauseState::Unpaused);
            }
            
            StateTransitionEvent::ToCharacterSelect => {
                previous_state.state = Some(current_state.get().clone());
                game_state.set(GameState::CharacterSelect);
                character_state.set(CharacterSelectState::Overview);
            }
            
            StateTransitionEvent::StartGame { character_id, level } => {
                progression.selected_character = *character_id;
                progression.current_level = *level;
                progression.is_new_game = *level == 1;
                game_state.set(GameState::Loading);
                pause_state.set(PauseState::Unpaused);
            }
            
            StateTransitionEvent::PauseGame => {
                pause_state.set(PauseState::Paused);
            }
            
            StateTransitionEvent::ResumeGame => {
                pause_state.set(PauseState::Unpaused);
            }
            
            StateTransitionEvent::GameOver { final_score } => {
                // Update scores and progression
                let level_index = (progression.current_level - 1) as usize;
                if level_index < 10 {
                    progression.level_scores[level_index] = progression.level_scores[level_index].max(*final_score);
                }
                progression.total_score = progression.level_scores.iter().sum();
                
                game_state.set(GameState::GameOver);
                pause_state.set(PauseState::Unpaused);
            }
            
            StateTransitionEvent::LevelComplete { score, level } => {
                // Update progression
                let level_index = (*level - 1) as usize;
                if level_index < 10 {
                    progression.level_scores[level_index] = progression.level_scores[level_index].max(*score);
                }
                progression.max_unlocked_level = progression.max_unlocked_level.max(*level + 1);
                progression.total_score = progression.level_scores.iter().sum();
                
                game_state.set(GameState::LevelComplete);
            }
            
            StateTransitionEvent::StartCutscene { cutscene_type } => {
                previous_state.state = Some(current_state.get().clone());
                cutscene_state.set(cutscene_type.clone());
                game_state.set(GameState::Cutscene);
            }
            
            StateTransitionEvent::EndCutscene => {
                // Determine next state based on progression
                if progression.current_level > 10 {
                    game_state.set(GameState::Credits);
                } else {
                    game_state.set(GameState::Playing);
                }
            }
            
            StateTransitionEvent::ToSettings => {
                previous_state.state = Some(current_state.get().clone());
                game_state.set(GameState::Settings);
            }
            
            StateTransitionEvent::FromSettings => {
                if let Some(prev_state) = &previous_state.state {
                    game_state.set(prev_state.clone());
                } else {
                    game_state.set(GameState::HomeScreen);
                }
                previous_state.state = None;
            }
            
            StateTransitionEvent::ToCredits => {
                previous_state.state = Some(current_state.get().clone());
                game_state.set(GameState::Credits);
            }
            
            StateTransitionEvent::RestartLevel => {
                game_state.set(GameState::Loading);
                pause_state.set(PauseState::Unpaused);
            }
            
            StateTransitionEvent::QuitToMenu => {
                game_state.set(GameState::HomeScreen);
                pause_state.set(PauseState::Unpaused);
                progression.is_new_game = true;
            }
        }
    }
}

/// Utility function to check if we're in a playable state
pub fn is_playing_state(state: &GameState) -> bool {
    matches!(state, GameState::Playing)
}

/// Utility function to check if we're in a menu state
pub fn is_menu_state(state: &GameState) -> bool {
    matches!(
        state,
        GameState::HomeScreen 
        | GameState::CharacterSelect 
        | GameState::Settings 
        | GameState::GameOver 
        | GameState::LevelComplete
        | GameState::Credits
    )
}

/// Utility function to check if we should show the game world
pub fn should_show_game_world(state: &GameState) -> bool {
    matches!(
        state,
        GameState::Playing 
        | GameState::Paused 
        | GameState::GameOver 
        | GameState::LevelComplete
    )
}

/// Plugin for state management
pub struct StateManagementPlugin;

impl Plugin for StateManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add all state types
            .add_state::<GameState>()
            .add_state::<PauseState>()
            .add_state::<CharacterSelectState>()
            .add_state::<CutsceneState>()
            
            // Add resources
            .insert_resource(PreviousGameState::default())
            .insert_resource(GameProgression::default())
            
            // Add events
            .add_event::<StateTransitionEvent>()
            
            // Add systems
            .add_systems(Update, handle_state_transitions);
    }
}