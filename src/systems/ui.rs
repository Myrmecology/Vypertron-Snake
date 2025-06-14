//! UI System for Vypertron-Snake
//! 
//! This module handles all user interface elements including:
//! - HUD (score, level, timer displays)
//! - Pause menu with options
//! - Game over screen with statistics
//! - Level complete screen with progression
//! - UI animations and effects
//! - Dynamic text updates and formatting

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::utils::*;
use crate::audio::*;

// ===============================
// UI COMPONENTS
// ===============================

/// Component for HUD elements that update dynamically
#[derive(Component, Debug)]
pub struct HudElement {
    pub element_type: HudElementType,
    pub update_frequency: f32,
    pub last_update: f32,
}

/// Types of HUD elements
#[derive(Debug, Clone, PartialEq)]
pub enum HudElementType {
    Score,
    HighScore,
    Level,
    Timer,
    Lives,
    Length,
    Speed,
    FoodCount,
}

/// Component for animated UI text
#[derive(Component, Debug)]
pub struct AnimatedText {
    pub animation_type: TextAnimation,
    pub timer: f32,
    pub duration: f32,
    pub original_color: Color,
    pub target_color: Color,
}

/// Types of text animations
#[derive(Debug, Clone)]
pub enum TextAnimation {
    Pulse,
    Flash,
    Rainbow,
    TypeWriter,
    ScoreIncrease,
}

/// Component for popup notifications
#[derive(Component, Debug)]
pub struct PopupNotification {
    pub message: String,
    pub notification_type: NotificationType,
    pub lifetime: f32,
    pub age: f32,
}

/// Types of popup notifications
#[derive(Debug, Clone)]
pub enum NotificationType {
    ScoreGain,
    Achievement,
    Warning,
    LevelUp,
    SpecialFood,
}

// ===============================
// HUD SETUP SYSTEM
// ===============================

/// Setup the main game HUD
pub fn setup_ui_elements(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    score_resource: Res<ScoreResource>,
    level_manager: Res<LevelManager>,
    character_selection: Res<CharacterSelection>,
) {
    info!("Setting up game UI elements");
    
    let font = asset_handles.fonts.get("main_font").cloned().unwrap_or_default();
    
    // Score display (top-left)
    commands.spawn((
        TextBundle::from_section(
            format!("Score: {}", score_resource.current_score),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        HudElement {
            element_type: HudElementType::Score,
            update_frequency: 0.1, // Update 10 times per second
            last_update: 0.0,
        },
        UIElement {
            element_type: UIElementType::Score,
            animation: Some(UIAnimation {
                animation_type: UIAnimationType::Pulse,
                timer: 0.0,
                duration: 2.0,
                loops: true,
            }),
            is_visible: true,
            layer: 100,
        },
    ));
    
    // High score display (top-center)
    commands.spawn((
        TextBundle::from_section(
            format!("High Score: {}", get_current_high_score(&level_manager, &character_selection)),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::rgb(1.0, 0.8, 0.0), // Gold color
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(400.0),
            ..default()
        }),
        HudElement {
            element_type: HudElementType::HighScore,
            update_frequency: 1.0, // Update once per second
            last_update: 0.0,
        },
        UIElement {
            element_type: UIElementType::HighScore,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
    
    // Level display (top-right)
    commands.spawn((
        TextBundle::from_section(
            format!("Level: {}/10", level_manager.current_level),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::rgb(0.5, 1.0, 0.5), // Light green
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        }),
        HudElement {
            element_type: HudElementType::Level,
            update_frequency: 5.0, // Update every 5 seconds (rarely changes)
            last_update: 0.0,
        },
        UIElement {
            element_type: UIElementType::Level,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
    
    // Snake length display (bottom-left)
    commands.spawn((
        TextBundle::from_section(
            "Length: 3",
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::rgb(0.8, 0.8, 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(60.0),
            left: Val::Px(20.0),
            ..default()
        }),
        HudElement {
            element_type: HudElementType::Length,
            update_frequency: 0.5, // Update twice per second
            last_update: 0.0,
        },
        UIElement {
            element_type: UIElementType::Length,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
    
    // Speed display (bottom-left, under length)
    commands.spawn((
        TextBundle::from_section(
            "Speed: 3.0",
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::rgb(0.8, 0.8, 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(35.0),
            left: Val::Px(20.0),
            ..default()
        }),
        HudElement {
            element_type: HudElementType::Speed,
            update_frequency: 0.5,
            last_update: 0.0,
        },
        UIElement {
            element_type: UIElementType::Speed,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
    
    // Character info (bottom-right)
    let character = &character_selection.characters[(character_selection.selected_character - 1) as usize];
    commands.spawn((
        TextBundle::from_section(
            &character.name,
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::rgba(character.color[0], character.color[1], character.color[2], 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Title,
            animation: Some(UIAnimation {
                animation_type: UIAnimationType::Pulse,
                timer: 0.0,
                duration: 3.0,
                loops: true,
            }),
            is_visible: true,
            layer: 100,
        },
    ));
    
    // Instructions (bottom-center)
    commands.spawn((
        TextBundle::from_section(
            "SPACEBAR: Pause • ARROWS: Move",
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::rgb(0.6, 0.6, 0.6),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(400.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Instructions,
            animation: None,
            is_visible: true,
            layer: 90,
        },
    ));
}

/// Get current high score for display
fn get_current_high_score(
    level_manager: &LevelManager,
    character_selection: &CharacterSelection,
) -> u32 {
    let level_index = (level_manager.current_level - 1) as usize;
    let character_index = (character_selection.selected_character - 1) as usize;
    
    // This would access the high score resource, but for now return a placeholder
    1000 // TODO: Access actual high score from resource
}

// ===============================
// HUD UPDATE SYSTEM
// ===============================

/// Update HUD elements with current game data
pub fn update_ui(
    time: Res<Time>,
    score_resource: Res<ScoreResource>,
    level_manager: Res<LevelManager>,
    snake_query: Query<&Snake>,
    food_query: Query<&Food>,
    mut hud_query: Query<(&mut Text, &mut HudElement)>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    for (mut text, mut hud_element) in hud_query.iter_mut() {
        hud_element.last_update += time.delta_seconds();
        
        // Only update if enough time has passed
        if hud_element.last_update >= hud_element.update_frequency {
            hud_element.last_update = 0.0;
            
            // Update text based on element type
            let new_text = match hud_element.element_type {
                HudElementType::Score => {
                    format!("Score: {}", score_resource.current_score)
                },
                HudElementType::HighScore => {
                    format!("High Score: {}", get_display_high_score(&score_resource))
                },
                HudElementType::Level => {
                    format!("Level: {}/10", level_manager.current_level)
                },
                HudElementType::Length => {
                    if let Ok(snake) = snake_query.get_single() {
                        format!("Length: {}", snake.length)
                    } else {
                        "Length: 0".to_string()
                    }
                },
                HudElementType::Speed => {
                    if let Ok(snake) = snake_query.get_single() {
                        format!("Speed: {:.1}", snake.speed)
                    } else {
                        "Speed: 0.0".to_string()
                    }
                },
                HudElementType::FoodCount => {
                    let food_count = food_query.iter().count();
                    format!("Food: {}", food_count)
                },
                _ => text.sections[0].value.clone(), // No change for unhandled types
            };
            
            // Check if text actually changed before updating
            if text.sections[0].value != new_text {
                let old_value = text.sections[0].value.clone();
                text.sections[0].value = new_text;
                
                // Play sound for score changes
                if hud_element.element_type == HudElementType::Score {
                    play_sound_events.send(
                        PlaySoundEvent::new("menu_navigate")
                            .with_volume(0.2)
                            .with_pitch(1.5)
                    );
                }
                
                debug!("Updated {:?}: {} -> {}", hud_element.element_type, old_value, text.sections[0].value);
            }
        }
    }
}

/// Get high score for display purposes
fn get_display_high_score(score_resource: &ScoreResource) -> u32 {
    // For now, just return current score as high score
    // In a full implementation, this would check the actual high score resource
    score_resource.current_score.max(5000) // Ensure at least 5000 for demo
}

// ===============================
// PAUSE MENU SYSTEM
// ===============================

/// Display pause menu when game is paused
pub fn display_pause_menu(
    mut commands: Commands,
    pause_state: Res<State<PauseState>>,
    asset_handles: Res<AssetHandles>,
    existing_pause_menu: Query<Entity, With<PauseMenuMarker>>,
) {
    // Only show pause menu if paused and not already showing
    if pause_state.get() == &PauseState::Paused && existing_pause_menu.is_empty() {
        let font = asset_handles.fonts.get("main_font").cloned().unwrap_or_default();
        
        info!("Displaying pause menu");
        
        // Semi-transparent overlay
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                z_index: ZIndex::Global(1000),
                ..default()
            },
            PauseMenuMarker,
        )).with_children(|parent| {
            // Pause title
            parent.spawn(TextBundle::from_section(
                "⏸️ PAUSED",
                TextStyle {
                    font: font.clone(),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ));
            
            // Resume instruction
            parent.spawn(TextBundle::from_section(
                "Press SPACEBAR to Resume",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            }));
            
            // Additional options
            parent.spawn(TextBundle::from_section(
                "R: Restart Level • Q: Quit to Menu • ESC: Quit to Menu",
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::rgb(0.6, 0.6, 0.6),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            }));
            
            // Current stats
            parent.spawn(TextBundle::from_section(
                "Game Statistics",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(40.0)),
                ..default()
            }));
        });
    }
}

/// Marker component for pause menu
#[derive(Component)]
pub struct PauseMenuMarker;

/// Remove pause menu when game resumes
pub fn cleanup_pause_menu(
    mut commands: Commands,
    pause_state: Res<State<PauseState>>,
    pause_menu_query: Query<Entity, With<PauseMenuMarker>>,
) {
    if pause_state.get() == &PauseState::Unpaused {
        for entity in pause_menu_query.iter() {
            commands.entity(entity).despawn_recursive();
            info!("Removed pause menu");
        }
    }
}

// ===============================
// GAME OVER SCREEN SYSTEM
// ===============================

/// Setup game over screen
pub fn setup_game_over_screen(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    score_resource: Res<ScoreResource>,
    level_manager: Res<LevelManager>,
    character_selection: Res<CharacterSelection>,
    game_statistics: Res<GameStatistics>,
) {
    let font = asset_handles.fonts.get("main_font").cloned().unwrap_or_default();
    let character = &character_selection.characters[(character_selection.selected_character - 1) as usize];
    
    info!("Setting up game over screen");
    
    // Full screen overlay
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.0, 0.0, 0.9).into(),
            z_index: ZIndex::Global(2000),
            ..default()
        },
        UIElement {
            element_type: UIElementType::GameOverScreen,
            animation: Some(UIAnimation {
                animation_type: UIAnimationType::FadeIn,
                timer: 0.0,
                duration: 1.0,
                loops: false,
            }),
            is_visible: true,
            layer: 2000,
        },
    )).with_children(|parent| {
        // Game Over title
        parent.spawn((
            TextBundle::from_section(
                "💥 GAME OVER",
                TextStyle {
                    font: font.clone(),
                    font_size: 64.0,
                    color: Color::rgb(1.0, 0.2, 0.2),
                },
            ),
            AnimatedText {
                animation_type: TextAnimation::Flash,
                timer: 0.0,
                duration: 2.0,
                original_color: Color::rgb(1.0, 0.2, 0.2),
                target_color: Color::rgb(1.0, 0.8, 0.8),
            },
        ));
        
        // Character that died
        parent.spawn(TextBundle::from_section(
            format!("{} has fallen!", character.name),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::rgba(character.color[0], character.color[1], character.color[2], 1.0),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        }));
        
        // Final score
        parent.spawn((
            TextBundle::from_section(
                format!("Final Score: {}", score_resource.current_score),
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::rgb(1.0, 0.8, 0.0),
                },
            ),
            AnimatedText {
                animation_type: TextAnimation::Pulse,
                timer: 0.0,
                duration: 1.5,
                original_color: Color::rgb(1.0, 0.8, 0.0),
                target_color: Color::rgb(1.0, 1.0, 0.5),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(30.0)),
            ..default()
        }));
        
        // Level reached
        parent.spawn(TextBundle::from_section(
            format!("Level Reached: {}/10", level_manager.current_level),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        }));
        
        // Food eaten
        parent.spawn(TextBundle::from_section(
            format!("Food Eaten: {}", score_resource.food_eaten),
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::rgb(0.8, 0.8, 0.8),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        }));
        
        // Score rank
        let rank = ScoreUtils::get_score_rank(score_resource.current_score);
        parent.spawn(TextBundle::from_section(
            format!("Rank: {}", rank.get_title()),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: rank.get_color(),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        }));
        
        // Controls
        parent.spawn(TextBundle::from_section(
            "SPACEBAR: Restart • ESC: Main Menu • S: Statistics",
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::rgb(0.6, 0.6, 0.6),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(40.0)),
            ..default()
        }));
    });
}

// ===============================
// LEVEL COMPLETE SCREEN SYSTEM
// ===============================

/// Setup level complete screen
pub fn setup_level_complete_screen(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    score_resource: Res<ScoreResource>,
    level_manager: Res<LevelManager>,
    character_selection: Res<CharacterSelection>,
) {
    let font = asset_handles.fonts.get("main_font").cloned().unwrap_or_default();
    let completed_level = level_manager.current_level;
    let character = &character_selection.characters[(character_selection.selected_character - 1) as usize];
    
    info!("Setting up level complete screen for level {}", completed_level);
    
    // Full screen overlay with celebration background
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.2, 0.0, 0.9).into(),
            z_index: ZIndex::Global(2000),
            ..default()
        },
        UIElement {
            element_type: UIElementType::LevelCompleteScreen,
            animation: Some(UIAnimation {
                animation_type: UIAnimationType::FadeIn,
                timer: 0.0,
                duration: 1.0,
                loops: false,
            }),
            is_visible: true,
            layer: 2000,
        },
    )).with_children(|parent| {
        // Level complete title
        parent.spawn((
            TextBundle::from_section(
                "🎉 LEVEL COMPLETE!",
                TextStyle {
                    font: font.clone(),
                    font_size: 56.0,
                    color: Color::rgb(0.2, 1.0, 0.2),
                },
            ),
            AnimatedText {
                animation_type: TextAnimation::Rainbow,
                timer: 0.0,
                duration: 3.0,
                original_color: Color::rgb(0.2, 1.0, 0.2),
                target_color: Color::rgb(0.8, 1.0, 0.8),
            },
        ));
        
        // Level info
        let level_name = if completed_level <= 10 {
            &level_manager.level_definitions[(completed_level - 1) as usize].name
        } else {
            "Final Level"
        };
        
        parent.spawn(TextBundle::from_section(
            format!("Level {}: {}", completed_level, level_name),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        }));
        
        // Character success
        parent.spawn(TextBundle::from_section(
            format!("{} conquers another realm!", character.name),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::rgba(character.color[0], character.color[1], character.color[2], 1.0),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(15.0)),
            ..default()
        }));
        
        // Level score
        parent.spawn((
            TextBundle::from_section(
                format!("Level Score: {}", score_resource.level_score),
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::rgb(1.0, 0.8, 0.0),
                },
            ),
            AnimatedText {
                animation_type: TextAnimation::ScoreIncrease,
                timer: 0.0,
                duration: 2.0,
                original_color: Color::rgb(1.0, 0.8, 0.0),
                target_color: Color::rgb(1.0, 1.0, 0.5),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(30.0)),
            ..default()
        }));
        
        // Bonuses (if any)
        if score_resource.time_bonus > 0 {
            parent.spawn(TextBundle::from_section(
                format!("⏱️ Time Bonus: +{}", score_resource.time_bonus),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::rgb(0.5, 1.0, 0.8),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            }));
        }
        
        // Next level preview (if not final level)
        if completed_level < 10 {
            let next_level_name = &level_manager.level_definitions[completed_level as usize].name;
            parent.spawn(TextBundle::from_section(
                format!("Next: {}", next_level_name),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::rgb(0.8, 0.8, 1.0),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            }));
        } else {
            // Final level completed
            parent.spawn(TextBundle::from_section(
                "🏆 ALL LEVELS COMPLETED! YOU ARE THE VYPERTRON CHAMPION!",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::rgb(1.0, 0.8, 0.0),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            }));
        }
        
        // Controls
        let controls_text = if completed_level < 10 {
            "SPACEBAR: Next Level • R: Replay Level • ESC: Main Menu"
        } else {
            "SPACEBAR: Play Again • ESC: Main Menu • You're Amazing!"
        };
        
        parent.spawn(TextBundle::from_section(
            controls_text,
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::rgb(0.6, 0.6, 0.6),
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(40.0)),
            ..default()
        }));
    });
}

// ===============================
// POPUP NOTIFICATION SYSTEM
// ===============================

/// Spawn popup notifications for various events
pub fn spawn_popup_notification(
    commands: &mut Commands,
    asset_handles: &AssetHandles,
    message: String,
    notification_type: NotificationType,
) {
    let font = asset_handles.fonts.get("main_font").cloned().unwrap_or_default();
    
    let (color, font_size, lifetime) = match notification_type {
        NotificationType::ScoreGain => (Color::rgb(1.0, 1.0, 0.2), 20.0, 1.5),
        NotificationType::Achievement => (Color::rgb(1.0, 0.5, 0.0), 24.0, 3.0),
        NotificationType::Warning => (Color::rgb(1.0, 0.2, 0.2), 22.0, 2.0),
        NotificationType::LevelUp => (Color::rgb(0.2, 1.0, 0.2), 28.0, 2.5),
        NotificationType::SpecialFood => (Color::rgb(0.8, 0.2, 1.0), 18.0, 2.0),
    };
    
    commands.spawn((
        TextBundle::from_section(
            message.clone(),
            TextStyle {
                font,
                font_size,
                color,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(600.0), // Center-ish
            ..default()
        }),
        PopupNotification {
            message,
            notification_type,
            lifetime,
            age: 0.0,
        },
        AnimatedText {
            animation_type: TextAnimation::Flash,
            timer: 0.0,
            duration: lifetime,
            original_color: color,
            target_color: Color::NONE,
        },
    ));
}

/// Update popup notifications
pub fn update_popup_notifications(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PopupNotification, &mut Transform), With<PopupNotification>>,
) {
    for (entity, mut popup, mut transform) in query.iter_mut() {
        popup.age += time.delta_seconds();
        
        // Float upward
        transform.translation.y += 50.0 * time.delta_seconds();
        
        // Fade out over time
        let alpha = (1.0 - popup.age / popup.lifetime).max(0.0);
        // Note: In a real implementation, we'd update the text color alpha
        
        // Remove when expired
        if popup.age >= popup.lifetime {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ===============================
// TEXT ANIMATION SYSTEM
// ===============================

/// Update animated text effects
pub fn update_animated_text(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut AnimatedText)>,
) {
    for (mut text, mut animated_text) in query.iter_mut() {
        animated_text.timer += time.delta_seconds();
        
        let progress = (animated_text.timer / animated_text.duration).min(1.0);
        
        match animated_text.animation_type {
            TextAnimation::Pulse => {
                let pulse = (progress * 2.0 * std::f32::consts::PI).sin() * 0.5 + 0.5;
                let color = ColorUtils::lerp_color(
                    animated_text.original_color,
                    animated_text.target_color,
                    pulse,
                );
                text.sections[0].style.color = color;
            },
            TextAnimation::Flash => {
                let flash = if (progress * 10.0).floor() as i32 % 2 == 0 {
                    animated_text.original_color
                } else {
                    animated_text.target_color
                };
                text.sections[0].style.color = flash;
            },
            TextAnimation::Rainbow => {
                let hue = (progress * 360.0) % 360.0;
                let rainbow_color = ColorUtils::hsv_to_rgb(hue, 1.0, 1.0);
                text.sections[0].style.color = rainbow_color;
            },
            TextAnimation::ScoreIncrease => {
                let bounce = AnimationUtils::apply_easing(progress, &EasingType::Bounce);
                let scale = 1.0 + bounce * 0.5;
                // Note: Scale would be applied to transform in a real implementation
            },
            TextAnimation::TypeWriter => {
                // Placeholder for typewriter effect
            },
        }
        
        // Reset animation if it's meant to loop
        if progress >= 1.0 {
            animated_text.timer = 0.0;
        }
    }
}

// ===============================
// CLEANUP SYSTEMS
// ===============================

/// Cleanup game over screen entities
pub fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, With<UIElement>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Game over screen cleaned up");
}

/// Cleanup UI elements when leaving gameplay
pub fn cleanup_ui_elements(
    mut commands: Commands,
    query: Query<Entity, Or<(With<HudElement>, With<UIElement>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("UI elements cleaned up");
}