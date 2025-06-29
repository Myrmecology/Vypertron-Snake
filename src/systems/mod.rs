//! Game Systems for Vypertron-Snake
//! 
//! This module contains all the core game logic systems that make Vypertron-Snake work.
//! Systems are functions that operate on entities with specific components,
//! implementing the game's behavior and mechanics.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::{StateTransitionEvent, GameState, PauseState, CutsceneState}; // FIXED: Explicit imports to avoid ambiguity
use crate::utils::*;
use crate::audio::*;
use rand::prelude::*;

// ===============================
// CORE GAME SYSTEMS
// ===============================

/// Setup the main camera - FIXED: Ensure proper camera setup for UI rendering
pub fn setup_camera(mut commands: Commands) {
    info!("Setting up camera for Vypertron-Snake");
    
    // Spawn a 2D camera that can render both game and UI
    let camera_entity = commands.spawn(Camera2dBundle {
        camera: Camera {
            // Ensure this camera renders to the main window
            target: bevy::render::camera::RenderTarget::default(),
            ..default()
        },
        ..default()
    }).id();
    
    info!("Camera spawned successfully with entity ID: {:?}", camera_entity);
}

/// Load global assets at startup
pub fn load_global_assets(
    _commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    info!("Loading global assets");
    
    // Load fonts
    let main_font = asset_server.load("fonts/retro_pixel.ttf");
    asset_handles.fonts.insert("main_font".to_string(), main_font);
    
    // Load UI textures
    let button_texture = asset_server.load("textures/ui/button.png");
    asset_handles.textures.insert("button".to_string(), button_texture);
    
    // Load character textures
    for i in 1..=4 {
        let character_texture = asset_server.load(&format!("textures/characters/snake_{}.png", i));
        asset_handles.textures.insert(format!("character_{}", i), character_texture);
    }
    
    // Load background textures for all themes
    let themes = [
        "classic", "digital", "forest", "desert", "ocean",
        "volcano", "ice", "space", "neon_city", "final_boss"
    ];
    
    for theme in themes.iter() {
        let bg_texture = asset_server.load(&format!("textures/backgrounds/{}_bg.png", theme));
        asset_handles.textures.insert(format!("{}_bg", theme), bg_texture);
    }
    
    asset_handles.loading_complete = true;
    info!("Global assets loaded successfully");
}

/// Load saved game data
pub fn load_saved_data(
    mut high_scores: ResMut<HighScoreResource>,
    mut game_settings: ResMut<GameSettings>,
    mut character_selection: ResMut<CharacterSelection>,
    mut level_manager: ResMut<LevelManager>,
) {
    info!("Loading saved game data");
    
    // Load high scores
    if let Ok(saved_scores) = SaveUtils::load_game_data::<HighScoreResource>("high_scores") {
        *high_scores = saved_scores;
        info!("Loaded high scores: global={}", high_scores.global_high_score);
    }
    
    // Load game settings
    if let Ok(saved_settings) = SaveUtils::load_game_data::<GameSettings>("game_settings") {
        *game_settings = saved_settings;
        info!("Loaded game settings");
    }
    
    // Load character unlocks
    if let Ok(saved_chars) = SaveUtils::load_game_data::<CharacterSelection>("character_selection") {
        *character_selection = saved_chars;
        info!("Loaded character selection data");
    }
    
    // Load level progress
    if let Ok(saved_levels) = SaveUtils::load_game_data::<LevelManager>("level_progress") {
        *level_manager = saved_levels;
        info!("Loaded level progress");
    }
}

/// Save game data periodically
pub fn save_game_data(
    time: Res<Time>,
    mut save_state: ResMut<SaveLoadState>,
    high_scores: Res<HighScoreResource>,
    game_settings: Res<GameSettings>,
    character_selection: Res<CharacterSelection>,
    level_manager: Res<LevelManager>,
) {
    save_state.last_save_time += time.delta_seconds_f64();
    
    // Auto-save every 30 seconds
    if save_state.last_save_time >= save_state.auto_save_interval {
        save_state.last_save_time = 0.0;
        
        // Save all persistent data
        if let Err(e) = SaveUtils::save_game_data(&*high_scores, "high_scores") {
            warn!("Failed to save high scores: {}", e);
        }
        
        if let Err(e) = SaveUtils::save_game_data(&*game_settings, "game_settings") {
            warn!("Failed to save game settings: {}", e);
        }
        
        if let Err(e) = SaveUtils::save_game_data(&*character_selection, "character_selection") {
            warn!("Failed to save character selection: {}", e);
        }
        
        if let Err(e) = SaveUtils::save_game_data(&*level_manager, "level_progress") {
            warn!("Failed to save level progress: {}", e);
        }
    }
}

// ===============================
// HOME SCREEN SYSTEMS
// ===============================

/// Setup the home screen
pub fn setup_home_screen(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    info!("Setting up home screen");
    
    // Ensure we have a camera before setting up UI
    if camera_query.is_empty() {
        warn!("No camera found! Creating emergency camera for UI rendering");
        commands.spawn(Camera2dBundle::default());
    } else {
        info!("Camera found, proceeding with home screen setup");
    }
    
    // Background - FIXED: Add UIElement for cleanup
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.1, 0.1, 0.2), // FIXED: rgb -> srgb
                custom_size: Some(Vec2::new(crate::DEFAULT_WINDOW_WIDTH, crate::DEFAULT_WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 0,
        },
    ));
    
    // Game title
    commands.spawn((
        TextBundle::from_section(
            "🐍⚡ VYPERTRON-SNAKE",
            TextStyle {
                font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                font_size: 48.0,
                color: Color::srgb(0.0, 1.0, 0.5), // FIXED: rgb -> srgb
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(50.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Title,
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
    
    // Subtitle
    commands.spawn((
        TextBundle::from_section(
            "Premium Snake Experience",
            TextStyle {
                font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                font_size: 24.0,
                color: Color::srgb(0.8, 0.8, 0.8), // FIXED: rgb -> srgb
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(160.0),
            left: Val::Px(50.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Subtitle,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
    
    // Start button
    spawn_menu_button(
        &mut commands,
        &asset_handles,
        Vec2::new(200.0, 300.0),
        "Start Game",
        ButtonAction::StartGame,
        &mut meshes,
        &mut materials,
    );
    
    // Settings button
    spawn_menu_button(
        &mut commands,
        &asset_handles,
        Vec2::new(200.0, 370.0),
        "Settings",
        ButtonAction::OpenSettings,
        &mut meshes,
        &mut materials,
    );
    
    // Credits button
    spawn_menu_button(
        &mut commands,
        &asset_handles,
        Vec2::new(200.0, 440.0),
        "Credits",
        ButtonAction::ShowCredits,
        &mut meshes,
        &mut materials,
    );
    
    // Create animated title snake
    create_title_snake(&mut commands, &mut meshes, &mut materials);
    
    // Instructions text
    commands.spawn((
        TextBundle::from_section(
            "Press SPACEBAR to start your adventure!\nUse arrow keys to move, SPACEBAR to pause",
            TextStyle {
                font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                font_size: 16.0,
                color: Color::srgb(0.6, 0.6, 0.6), // FIXED: rgb -> srgb
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Instructions,
            animation: Some(UIAnimation {
                animation_type: UIAnimationType::FadeIn,
                timer: 0.0,
                duration: 3.0,
                loops: false,
            }),
            is_visible: true,
            layer: 100,
        },
    ));
}

/// Create the animated title snake that wraps around the title - FIXED: Add UIElement for cleanup
fn create_title_snake(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Create path points for snake to follow around title
    let path_points = vec![
        Vec2::new(40.0, 120.0),   // Start left of title
        Vec2::new(500.0, 120.0),  // Move right
        Vec2::new(520.0, 140.0),  // Curve down
        Vec2::new(520.0, 180.0),  // Move down
        Vec2::new(500.0, 200.0),  // Curve left
        Vec2::new(40.0, 200.0),   // Move left
        Vec2::new(20.0, 180.0),   // Curve up
        Vec2::new(20.0, 140.0),   // Move up
        Vec2::new(40.0, 120.0),   // Complete circle
    ];
    
    let snake_material = materials.add(ColorMaterial::from(Color::srgb(0.0, 0.8, 0.0))); // FIXED: rgb -> srgb
    let segment_mesh = meshes.add(Mesh::from(Rectangle::new(16.0, 16.0))); // FIXED: shape::Quad -> Rectangle
    
    // Create title snake entity - FIXED: Add UIElement for cleanup
    commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: segment_mesh.into(),
            material: snake_material,
            transform: Transform::from_xyz(path_points[0].x, path_points[0].y, 1.0),
            ..default()
        },
        TitleSnake {
            path_points,
            path_position: 0.0,
            animation_speed: 0.3,
            segment_count: 8,
        },
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
}

/// Spawn a menu button with consistent styling - FIXED: Add UIElement for cleanup
fn spawn_menu_button(
    commands: &mut Commands,
    asset_handles: &AssetHandles,
    position: Vec2,
    text: &str,
    action: ButtonAction,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let button_material = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.2, 0.3))); // FIXED: rgb -> srgb
    let button_mesh = meshes.add(Mesh::from(Rectangle::new(180.0, 50.0))); // FIXED: shape::Quad -> Rectangle
    
    // Button background - FIXED: Add UIElement for cleanup
    let _button_entity = commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: button_mesh.into(),
            material: button_material,
            transform: Transform::from_xyz(position.x, position.y, 2.0),
            ..default()
        },
        MenuButton {
            action,
            state: ButtonState::Normal,
            hover_timer: 0.0,
            text: text.to_string(),
        },
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    )).id();
    
    // Button text - FIXED: Add UIElement for cleanup
    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(position.y - 10.0),
            left: Val::Px(position.x - 80.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    ));
}

/// Animate the title snake
pub fn animate_title_snake(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TitleSnake)>,
) {
    for (mut transform, mut title_snake) in query.iter_mut() {
        // Update animation position
        title_snake.path_position += title_snake.animation_speed * time.delta_seconds();
        
        // Wrap around when reaching end of path
        if title_snake.path_position >= 1.0 {
            title_snake.path_position = 0.0;
        }
        
        // Calculate current position on path
        let path_len = title_snake.path_points.len() as f32;
        let segment_progress = title_snake.path_position * path_len;
        let segment_index = segment_progress.floor() as usize;
        let segment_t = segment_progress.fract();
        
        if segment_index < title_snake.path_points.len() - 1 {
            let start_point = title_snake.path_points[segment_index];
            let end_point = title_snake.path_points[segment_index + 1];
            
            let current_pos = start_point.lerp(end_point, segment_t);
            transform.translation.x = current_pos.x;
            transform.translation.y = current_pos.y;
        }
    }
}

/// Handle input on home screen
pub fn handle_home_screen_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut button_query: Query<(Entity, &Transform, &mut MenuButton)>,
    mut state_events: EventWriter<StateTransitionEvent>,
    _game_settings: Res<GameSettings>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    // Handle spacebar to start game
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("🎮 SPACEBAR pressed - going to character select");
        play_sound_events.send(PlaySoundEvent::new("menu_select"));
        state_events.send(StateTransitionEvent::ToCharacterSelect);
        return;
    }
    
    // Handle mouse interaction with buttons
    if let Ok(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            info!("🖱️ Mouse cursor at screen position: {:?}", cursor_pos);
            
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos);
                
                if let Some(world_pos) = world_pos {
                    info!("🌍 Mouse world position: {:?}", world_pos);
                    
                    // Check button hover and clicks
                    for (_entity, transform, mut button) in button_query.iter_mut() {
                        let button_bounds = Rect::from_center_size(
                            transform.translation.truncate(),
                            Vec2::new(180.0, 50.0)
                        );
                        
                        info!("🔘 Button '{}' at world pos: {:?}, bounds: {:?}", 
                              button.text, transform.translation.truncate(), button_bounds);
                        
                        if button_bounds.contains(world_pos) {
                            info!("✅ Button '{}' is HOVERED!", button.text);
                            
                            // Button is hovered
                            if button.state != ButtonState::Hovered {
                                button.state = ButtonState::Hovered;
                                play_sound_events.send(PlaySoundEvent::new("menu_navigate"));
                            }
                            
                            // Handle click
                            if mouse_input.just_pressed(MouseButton::Left) {
                                info!("🖱️ Button '{}' was CLICKED!", button.text);
                                button.state = ButtonState::Pressed;
                                play_sound_events.send(PlaySoundEvent::new("menu_select"));
                                
                                // Execute button action
                                match &button.action {
                                    ButtonAction::StartGame => {
                                        info!("🚀 StartGame button clicked");
                                        state_events.send(StateTransitionEvent::ToCharacterSelect);
                                    },
                                    ButtonAction::OpenSettings => {
                                        info!("⚙️ Settings button clicked");
                                        state_events.send(StateTransitionEvent::ToSettings);
                                    },
                                    ButtonAction::ShowCredits => {
                                        info!("📜 Credits button clicked");
                                        state_events.send(StateTransitionEvent::ToCredits);
                                    },
                                    _ => {
                                        info!("❓ Unknown button action: {:?}", button.action);
                                    },
                                }
                            }
                        } else {
                            // Button is not hovered
                            if button.state == ButtonState::Hovered {
                                button.state = ButtonState::Normal;
                            }
                        }
                    }
                } else {
                    warn!("❌ Could not convert cursor position to world coordinates");
                }
            } else {
                warn!("❌ Could not get camera for coordinate conversion");
            }
        }
    }
}

/// Update menu button visual states
pub fn update_menu_buttons(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MenuButton), With<MenuButton>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut transform, mut button) in query.iter_mut() {
        button.hover_timer += time.delta_seconds();
        
        match button.state {
            ButtonState::Normal => {
                transform.scale = Vec3::splat(1.0);
            },
            ButtonState::Hovered => {
                // Gentle pulse effect
                let pulse = AnimationUtils::pulse(button.hover_timer, 2.0) * 0.05 + 1.0;
                transform.scale = Vec3::splat(pulse);
            },
            ButtonState::Pressed => {
                transform.scale = Vec3::splat(0.95);
                // Reset to normal after brief press
                button.state = ButtonState::Normal;
            },
            ButtonState::Disabled => {
                transform.scale = Vec3::splat(0.9);
            },
        }
    }
}

/// Cleanup home screen entities
pub fn cleanup_home_screen(
    mut commands: Commands,
    query: Query<Entity, Or<(With<UIElement>, With<TitleSnake>, With<MenuButton>)>>,
) {
    info!("Cleaning up home screen");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ===============================
// CHARACTER SELECTION SYSTEMS - FIXED: Proper UI cleanup
// ===============================

/// Setup character selection screen - FIXED: Create proper interactive UI with cleanup tags
pub fn setup_character_selection(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    character_selection: Res<CharacterSelection>,
) {
    info!("Setting up character selection screen with interactive UI");
    
    // Main UI root node - FIXED: Add UIElement for cleanup
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.1, 0.15).into(),
            ..default()
        },
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    )).with_children(|parent| {
        // Title - FIXED: Add UIElement for cleanup
        parent.spawn((
            TextBundle::from_section(
                "Choose Your Snake",
                TextStyle {
                    font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                    font_size: 36.0,
                    color: Color::srgb(0.0, 1.0, 0.8),
                },
            ),
            UIElement {
                element_type: UIElementType::Title,
                animation: Some(UIAnimation {
                    animation_type: UIAnimationType::FadeIn,
                    timer: 0.0,
                    duration: 1.0,
                    loops: false,
                }),
                is_visible: true,
                layer: 100,
            },
        ));

        // Character selection container - FIXED: Add UIElement for cleanup
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(400.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UIElement {
                element_type: UIElementType::Title,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        )).with_children(|parent| {
            // Create character cards
            for (i, character) in character_selection.characters.iter().enumerate() {
                let character_id = (i + 1) as u32;
                let is_unlocked = character_selection.unlocked_characters[i];
                let is_selected = character_id == character_selection.selected_character;
                
                create_character_button(
                    parent,
                    &asset_handles,
                    character,
                    character_id,
                    is_unlocked,
                    is_selected,
                );
            }
        });

        // Selected character info - FIXED: Add UIElement for cleanup
        let selected_character = &character_selection.characters[(character_selection.selected_character - 1) as usize];
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.2, 0.2, 0.3, 0.8).into(),
                ..default()
            },
            UIElement {
                element_type: UIElementType::Title,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    &selected_character.name,
                    TextStyle {
                        font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                UIElement {
                    element_type: UIElementType::Title,
                    animation: None,
                    is_visible: true,
                    layer: 100,
                },
            ));
            
            parent.spawn((
                TextBundle::from_section(
                    &selected_character.description,
                    TextStyle {
                        font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                        font_size: 16.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                    },
                ),
                UIElement {
                    element_type: UIElementType::Subtitle,
                    animation: None,
                    is_visible: true,
                    layer: 100,
                },
            ));
        });

        // Bottom buttons container - FIXED: Add UIElement for cleanup
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UIElement {
                element_type: UIElementType::Title,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        )).with_children(|parent| {
            // Back button
            create_ui_menu_button(
                parent,
                &asset_handles,
                "Back",
                ButtonAction::QuitToMenu,
                Color::srgb(0.6, 0.2, 0.2),
            );

            // Start Adventure button
            create_ui_menu_button(
                parent,
                &asset_handles,
                "Start Adventure",
                ButtonAction::StartGame,
                Color::srgb(0.2, 0.6, 0.2),
            );
        });
    });
}

/// Create a character selection button with proper interaction - FIXED: Add UIElement for cleanup
fn create_character_button(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    character: &CharacterDefinition,
    character_id: u32,
    is_unlocked: bool,
    is_selected: bool,
) {
    let button_color = if is_unlocked {
        if is_selected {
            Color::srgba(character.color[0], character.color[1], character.color[2], 0.8)
        } else {
            Color::srgba(0.2, 0.2, 0.3, 0.8)
        }
    } else {
        Color::srgba(0.1, 0.1, 0.1, 0.8)
    };

    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(if is_selected { 3.0 } else { 1.0 })),
                ..default()
            },
            background_color: button_color.into(),
            border_color: if is_selected {
                Color::srgb(1.0, 1.0, 0.0).into()
            } else {
                Color::srgb(0.5, 0.5, 0.5).into()
            },
            ..default()
        },
        MenuButton {
            action: ButtonAction::SelectCharacter(character_id),
            state: ButtonState::Normal,
            hover_timer: 0.0,
            text: character.name.clone(),
        },
        CharacterCard {
            character_id,
            name: character.name.clone(),
            description: character.description.clone(),
            color: Color::srgba(character.color[0], character.color[1], character.color[2], 1.0),
            is_selected,
            animation_timer: 0.0,
            is_unlocked,
        },
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    )).with_children(|parent| {
        // Character preview (placeholder colored square)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                background_color: if is_unlocked {
                    Color::srgba(character.color[0], character.color[1], character.color[2], 1.0).into()
                } else {
                    Color::srgb(0.2, 0.2, 0.2).into()
                },
                ..default()
            },
            UIElement {
                element_type: UIElementType::Title,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        ));

        // Character name
        parent.spawn((
            TextBundle::from_section(
                &character.name,
                TextStyle {
                    font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                    font_size: 18.0,
                    color: if is_unlocked { Color::WHITE } else { Color::srgb(0.5, 0.5, 0.5) },
                },
            ),
            UIElement {
                element_type: UIElementType::Title,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        ));

        // Status text
        parent.spawn((
            TextBundle::from_section(
                if is_unlocked {
                    if is_selected { "SELECTED" } else { "Available" }
                } else {
                    "LOCKED"
                },
                TextStyle {
                    font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                    font_size: 12.0,
                    color: if is_unlocked {
                        if is_selected { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.8, 0.8, 0.8) }
                    } else {
                        Color::srgb(0.4, 0.4, 0.4)
                    },
                },
            ),
            UIElement {
                element_type: UIElementType::Subtitle,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        ));
    });
}

/// Create a menu button with proper interaction (renamed to avoid conflict) - FIXED: Add UIElement for cleanup
fn create_ui_menu_button(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    text: &str,
    action: ButtonAction,
    color: Color,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: color.into(),
            border_color: Color::srgb(0.5, 0.5, 0.5).into(),
            ..default()
        },
        MenuButton {
            action,
            state: ButtonState::Normal,
            hover_timer: 0.0,
            text: text.to_string(),
        },
        UIElement {
            element_type: UIElementType::Title,
            animation: None,
            is_visible: true,
            layer: 100,
        },
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_handles.fonts.get("main_font").cloned().unwrap_or_default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            UIElement {
                element_type: UIElementType::Title,
                animation: None,
                is_visible: true,
                layer: 100,
            },
        ));
    });
}

/// Handle character selection input - REMOVED: This is now handled by the input module
pub fn handle_character_selection_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, // FIXED: Input -> ButtonInput
    mouse_input: Res<ButtonInput<MouseButton>>, // FIXED: Input -> ButtonInput
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut character_selection: ResMut<CharacterSelection>,
    mut character_card_query: Query<(Entity, &Transform, &mut CharacterCard)>, // FIXED: Combined queries to avoid conflict
    mut button_query: Query<(Entity, &Transform, &mut MenuButton)>, // FIXED: Combined query
    mut state_events: EventWriter<StateTransitionEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    // This function is now replaced by the one in input.rs
    // Keeping it here as a placeholder to avoid compilation errors
    // The actual input handling is done by the input module
    info!("Character selection input handled by input module");
}

/// Update character card animations
pub fn update_character_cards(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CharacterCard)>,
) {
    for (mut transform, mut card) in query.iter_mut() {
        card.animation_timer += time.delta_seconds();
        
        if card.is_selected {
            // Gentle pulsing for selected character
            let pulse = AnimationUtils::pulse(card.animation_timer, 1.5) * 0.1 + 1.0;
            transform.scale = Vec3::splat(pulse);
        } else {
            transform.scale = Vec3::splat(1.0);
        }
    }
}

/// Animate character previews
pub fn animate_character_previews(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<CharacterCard>>,
) {
    for mut transform in query.iter_mut() {
        // Gentle floating animation
        let float_offset = AnimationUtils::floating_offset(time.elapsed_seconds(), 5.0, 1.0);
        transform.translation.y += float_offset * 0.5;
    }
}

/// Cleanup character selection screen - FIXED: Now properly cleans up UIElement components
pub fn cleanup_character_selection(
    mut commands: Commands,
    query: Query<Entity, Or<(With<UIElement>, With<CharacterCard>, With<MenuButton>)>>,
) {
    info!("Cleaning up character selection screen");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Character selection screen cleanup complete");
}

// ===============================
// WINDOW MANAGEMENT
// ===============================

/// Handle window resize events
pub fn handle_window_resize(
    mut resize_events: EventReader<bevy::window::WindowResized>,
    mut game_settings: ResMut<GameSettings>,
) {
    for event in resize_events.read() {
        game_settings.resolution = (event.width, event.height);
        info!("Window resized to: {}x{}", event.width, event.height);
    }
}

// ===============================
// GLOBAL ANIMATION SYSTEM
// ===============================

/// FIXED: Update all UI animations without borrowing conflicts
pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(&mut Style, &mut UIElement), With<UIElement>>,
    _text_query: Query<&mut Text>,
) {
    for (_style, mut ui_element) in query.iter_mut() {
        // First, collect all the information we need from animation
        let (should_update_timer, should_reset_timer, should_hide) = if let Some(ref animation) = ui_element.animation {
            let progress = (animation.timer / animation.duration).min(1.0);
            let should_hide = matches!(animation.animation_type, UIAnimationType::FadeOut) && progress >= 1.0;
            let should_reset = animation.loops && progress >= 1.0;
            (true, should_reset, should_hide)
        } else {
            (false, false, false)
        };
        
        // Now apply the changes without conflicting borrows
        if should_update_timer {
            if let Some(ref mut animation) = ui_element.animation {
                animation.timer += time.delta_seconds();
                
                let progress = (animation.timer / animation.duration).min(1.0);
                
                // Convert UIAnimationType to EasingType or handle appropriately
                let easing_type = match animation.animation_type {
                    UIAnimationType::FadeIn => EasingType::EaseInOut,
                    UIAnimationType::FadeOut => EasingType::EaseInOut,
                    UIAnimationType::Pulse => EasingType::EaseInOut,
                    UIAnimationType::Bounce => EasingType::Bounce,
                    UIAnimationType::Shake => EasingType::EaseInOut,
                    UIAnimationType::Slide => EasingType::EaseInOut,
                };
                let _eased_progress = AnimationUtils::apply_easing(progress, &easing_type);
                
                match animation.animation_type {
                    UIAnimationType::FadeIn => {
                        // Apply fade in effect - would need access to text color
                    },
                    UIAnimationType::FadeOut => {
                        // Apply fade out effect - visibility handled below
                    },
                    UIAnimationType::Pulse => {
                        // Pulsing effect - handled by individual systems
                    },
                    UIAnimationType::Bounce => {
                        // Bouncing effect
                    },
                    UIAnimationType::Shake => {
                        // Screen shake effect
                    },
                    UIAnimationType::Slide => {
                        // Sliding animation
                    },
                }
                
                // Reset looping animations
                if should_reset_timer {
                    animation.timer = 0.0;
                }
            }
        }
        
        // Set visibility without animation borrow
        if should_hide {
            ui_element.is_visible = false;
        }
    }
}

// ===============================
// SYSTEM MODULE IMPORTS
// ===============================

// Import all our system modules
pub mod input;
pub mod snake;
pub mod collision;
pub mod food;
pub mod ui;
pub mod effects;

// Re-export system functions for easy access
pub use input::*;
pub use snake::*;
pub use collision::*;
pub use food::*;
pub use ui::*;
pub use effects::*;