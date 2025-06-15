//! Game State Management for Vypertron-Snake
//!
//! Defines all hierarchical game states, transition events, and logic for handling progression.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// === Primary Game States ===
///
/// Full game flow:
/// HomeScreen → CharacterSelect → Playing → (LevelComplete | GameOver)
/// → Cutscene → Playing (next level)
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, Serialize, Deserialize)]
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
    Loading,
    Credits,
}

/// === Pause Sub-State ===
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[source(GameState = GameState::Playing)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

/// === Character Selection State ===
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[source(GameState = GameState::CharacterSelect)]
pub enum CharacterSelectState {
    #[default]
    Overview,
    Character1,
    Character2,
    Character3,
    Character4,
}

/// === Cutscene Sub-State ===
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[source(GameState = GameState::Cutscene)]
pub enum CutsceneState {
    #[default]
    Intro,
    LevelTransition,
    Victory,
    GameOverStory,
}

/// === Game Transition Events ===
/// Note: This is a CUSTOM event, different from Bevy's StateTransitionEvent<S>
#[derive(Event, Debug, Clone)]
pub enum StateTransitionEvent {
    ToHomeScreen,
    ToCharacterSelect,
    StartGame { character_id: u32, level: u32 },
    PauseGame,
    ResumeGame,
    GameOver { final_score: u32 },
    LevelComplete { score: u32, level: u32 },
    StartCutscene { cutscene_type: CutsceneState },
    EndCutscene,
    ToSettings,
    FromSettings,
    ToCredits,
    RestartLevel,
    QuitToMenu,
}

/// === Supporting Resources ===

/// Tracks previous game state (for returning from settings, etc.)
#[derive(Resource, Debug, Clone, Default)]
pub struct PreviousGameState {
    pub state: Option<GameState>,
}

/// Tracks level, score, and character selection
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameProgression {
    pub current_level: u32,
    pub max_unlocked_level: u32,
    pub selected_character: u32,
    pub is_new_game: bool,
    pub total_score: u32,
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

/// === State Transition System ===
pub fn handle_state_transitions(
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
            StateTransitionEvent::PauseGame => pause_state.set(PauseState::Paused),
            StateTransitionEvent::ResumeGame => pause_state.set(PauseState::Unpaused),
            StateTransitionEvent::GameOver { final_score } => {
                let i = (progression.current_level - 1) as usize;
                if i < 10 {
                    progression.level_scores[i] = progression.level_scores[i].max(*final_score);
                }
                progression.total_score = progression.level_scores.iter().sum();
                game_state.set(GameState::GameOver);
                pause_state.set(PauseState::Unpaused);
            }
            StateTransitionEvent::LevelComplete { score, level } => {
                let i = (*level - 1) as usize;
                if i < 10 {
                    progression.level_scores[i] = progression.level_scores[i].max(*score);
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
                game_state.set(if progression.current_level > 10 {
                    GameState::Credits
                } else {
                    GameState::Playing
                });
            }
            StateTransitionEvent::ToSettings => {
                previous_state.state = Some(current_state.get().clone());
                game_state.set(GameState::Settings);
            }
            StateTransitionEvent::FromSettings => {
                game_state.set(
                    previous_state.state.take().unwrap_or(GameState::HomeScreen),
                );
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

/// === Utility Functions ===

pub fn is_playing_state(state: &GameState) -> bool {
    matches!(state, GameState::Playing)
}

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

pub fn should_show_game_world(state: &GameState) -> bool {
    matches!(
        state,
        GameState::Playing
            | GameState::Paused
            | GameState::GameOver
            | GameState::LevelComplete
    )
}

/// === Plugin for Game State Management ===
pub struct StateManagementPlugin;

impl Plugin for StateManagementPlugin {
    fn build(&self, app: &mut App) {
        // Bevy 0.14 syntax for states
        app.init_state::<GameState>()
            .add_sub_state::<PauseState>()
            .add_sub_state::<CharacterSelectState>()
            .add_sub_state::<CutsceneState>()
            .insert_resource(PreviousGameState::default())
            .insert_resource(GameProgression::default())
            .add_event::<StateTransitionEvent>()
            .add_systems(Update, handle_state_transitions);
    }
}
