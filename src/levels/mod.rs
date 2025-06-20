//! Level Management System for Vypertron-Snake
//! 
//! This module handles the creation, loading, and management of all 10 unique levels.
//! Each level has its own theme, mechanics, layout, and progression requirements.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use rand::prelude::*;

// ===============================
// LEVEL LOADING SYSTEM
// ===============================

/// System to load and setup a level when entering Playing state
pub fn load_level_system(
    mut commands: Commands,
    level_manager: Res<LevelManager>,
    character_selection: Res<CharacterSelection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_handles: Res<AssetHandles>,
) {
    let current_level = level_manager.current_level;
    let level_def = &level_manager.level_definitions[(current_level - 1) as usize];
    
    info!("Loading Level {}: {}", current_level, level_def.name);
    
    // Clear any existing level entities
    // (This will be handled by cleanup systems)
    
    // Setup level background
    setup_level_background(&mut commands, level_def, &asset_handles);
    
    // Setup level boundaries
    setup_level_boundaries(&mut commands, level_def, &mut meshes, &mut materials);
    
    // Setup level-specific walls and obstacles
    setup_level_walls(&mut commands, level_def, &mut meshes, &mut materials);
    
    // Setup special mechanics for this level
    setup_special_mechanics(&mut commands, level_def);
    
    // Setup level UI
    setup_level_ui(&mut commands, level_def, &asset_handles);
    
    // Apply character-specific modifications
    apply_character_modifications(&mut commands, &character_selection, level_def);
}

/// Setup the visual background for the current level
fn setup_level_background(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    asset_handles: &AssetHandles,
) {
    let background_texture = match level_def.theme {
        LevelTheme::Classic => "backgrounds/classic_grass.png",
        LevelTheme::Digital => "backgrounds/digital_grid.png",
        LevelTheme::Forest => "backgrounds/forest_canopy.png",
        LevelTheme::Desert => "backgrounds/desert_dunes.png",
        LevelTheme::Ocean => "backgrounds/ocean_depths.png",
        LevelTheme::Volcano => "backgrounds/volcanic_cavern.png",
        LevelTheme::Ice => "backgrounds/ice_crystals.png",
        LevelTheme::Space => "backgrounds/star_field.png",
        LevelTheme::NeonCity => "backgrounds/neon_skyline.png",
        LevelTheme::FinalBoss => "backgrounds/vypertron_lair.png",
        LevelTheme::Cyber => "backgrounds/cyber_matrix.png",
        LevelTheme::Shadow => "backgrounds/shadow_realm.png",
        LevelTheme::Cosmic => "backgrounds/cosmic_void.png",
    };
    
    // Main background
    commands.spawn((
        SpriteBundle {
            texture: asset_handles.textures.get(background_texture)
                .cloned()
                .unwrap_or_default(),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        LevelBackground {
            level: level_def.level_number,
            theme: level_def.theme.clone(),
            scroll_offset: Vec2::ZERO,
            scroll_speed: get_scroll_speed_for_theme(&level_def.theme),
            parallax_layers: create_parallax_layers(&level_def.theme),
        },
    ));
    
    // Add parallax layers for depth
    setup_parallax_layers(commands, level_def, asset_handles);
}

/// Get background scroll speed based on theme
fn get_scroll_speed_for_theme(theme: &LevelTheme) -> Vec2 {
    match theme {
        LevelTheme::Classic => Vec2::new(0.5, 0.0),
        LevelTheme::Digital => Vec2::new(1.0, 0.5),
        LevelTheme::Forest => Vec2::new(0.3, 0.1),
        LevelTheme::Desert => Vec2::new(2.0, 0.0), // Sandstorm effect
        LevelTheme::Ocean => Vec2::new(0.8, 0.4), // Current flow
        LevelTheme::Volcano => Vec2::new(0.0, 1.5), // Rising heat
        LevelTheme::Ice => Vec2::new(0.2, 0.0), // Slow drift
        LevelTheme::Space => Vec2::new(0.0, 0.0), // Stationary stars
        LevelTheme::NeonCity => Vec2::new(1.5, 0.0), // Fast city lights
        LevelTheme::FinalBoss => Vec2::new(0.0, 0.0), // Dramatic stillness
        LevelTheme::Cyber => Vec2::new(2.0, 1.0), // Fast data streams
        LevelTheme::Shadow => Vec2::new(0.1, 0.3), // Eerie slow movement
        LevelTheme::Cosmic => Vec2::new(0.0, 0.0), // Vast stillness
    }
}

/// Create parallax layers for visual depth
fn create_parallax_layers(theme: &LevelTheme) -> Vec<ParallaxLayer> {
    match theme {
        LevelTheme::Forest => vec![
            ParallaxLayer { depth: 1.0, scroll_multiplier: 0.2, opacity: 0.6 }, // Far trees
            ParallaxLayer { depth: 0.5, scroll_multiplier: 0.5, opacity: 0.8 }, // Mid trees
            ParallaxLayer { depth: 0.1, scroll_multiplier: 0.9, opacity: 1.0 }, // Near foliage
        ],
        LevelTheme::Space => vec![
            ParallaxLayer { depth: 5.0, scroll_multiplier: 0.1, opacity: 0.4 }, // Distant stars
            ParallaxLayer { depth: 2.0, scroll_multiplier: 0.3, opacity: 0.6 }, // Nebulae
            ParallaxLayer { depth: 1.0, scroll_multiplier: 0.7, opacity: 0.8 }, // Near planets
        ],
        LevelTheme::Ocean => vec![
            ParallaxLayer { depth: 3.0, scroll_multiplier: 0.2, opacity: 0.5 }, // Deep current
            ParallaxLayer { depth: 1.5, scroll_multiplier: 0.4, opacity: 0.7 }, // Mid current
            ParallaxLayer { depth: 0.5, scroll_multiplier: 0.8, opacity: 0.9 }, // Surface bubbles
        ],
        LevelTheme::NeonCity => vec![
            ParallaxLayer { depth: 4.0, scroll_multiplier: 0.1, opacity: 0.3 }, // Distant buildings
            ParallaxLayer { depth: 2.0, scroll_multiplier: 0.4, opacity: 0.6 }, // Mid buildings
            ParallaxLayer { depth: 1.0, scroll_multiplier: 0.8, opacity: 0.9 }, // Near neon signs
        ],
        _ => vec![
            ParallaxLayer { depth: 2.0, scroll_multiplier: 0.3, opacity: 0.7 },
            ParallaxLayer { depth: 1.0, scroll_multiplier: 0.6, opacity: 0.9 },
        ],
    }
}

/// Setup parallax background layers
fn setup_parallax_layers(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    asset_handles: &AssetHandles,
) {
    let layers = create_parallax_layers(&level_def.theme);
    
    for (index, layer) in layers.iter().enumerate() {
        let layer_texture = format!("backgrounds/{}_layer_{}.png", 
            get_theme_name(&level_def.theme), index);
        
        commands.spawn((
            SpriteBundle {
                texture: asset_handles.textures.get(&layer_texture)
                    .cloned()
                    .unwrap_or_default(),
                transform: Transform::from_xyz(0.0, 0.0, -5.0 - layer.depth),
                sprite: Sprite {
                    // FIXED: Changed Color::rgba to Color::srgba for Bevy 0.14
                    color: Color::srgba(1.0, 1.0, 1.0, layer.opacity),
                    ..default()
                },
                ..default()
            },
            *layer,
        ));
    }
}

/// Get theme name for asset loading
fn get_theme_name(theme: &LevelTheme) -> &'static str {
    match theme {
        LevelTheme::Classic => "classic",
        LevelTheme::Digital => "digital",
        LevelTheme::Forest => "forest",
        LevelTheme::Desert => "desert",
        LevelTheme::Ocean => "ocean",
        LevelTheme::Volcano => "volcano",
        LevelTheme::Ice => "ice",
        LevelTheme::Space => "space",
        LevelTheme::NeonCity => "neon_city",
        LevelTheme::FinalBoss => "final_boss",
        LevelTheme::Cyber => "cyber",
        LevelTheme::Shadow => "shadow",
        LevelTheme::Cosmic => "cosmic",
    }
}

/// Setup level boundary walls
fn setup_level_boundaries(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let (grid_width, grid_height) = level_def.grid_size;
    let wall_color = get_wall_color_for_theme(&level_def.theme);
    
    let wall_material = materials.add(ColorMaterial::from(wall_color));
    let wall_mesh = meshes.add(Mesh::from(Rectangle::new(
        crate::GRID_SIZE, crate::GRID_SIZE
    )));
    
    // Top and bottom boundaries
    for x in 0..grid_width {
        // Top wall
        commands.spawn((
            Mesh2dBundle {
                mesh: wall_mesh.clone().into(),
                material: wall_material.clone(),
                transform: Transform::from_xyz(
                    (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.0,
                ),
                ..default()
            },
            Wall {
                grid_position: Vec2::new(x as f32, grid_height as f32 - 1.0),
                wall_type: WallType::Boundary,
                health: u32::MAX,
            },
            GridPosition::new(x as i32, grid_height as i32 - 1),
        ));
        
        // Bottom wall
        commands.spawn((
            Mesh2dBundle {
                mesh: wall_mesh.clone().into(),
                material: wall_material.clone(),
                transform: Transform::from_xyz(
                    (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    -(grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.0,
                ),
                ..default()
            },
            Wall {
                grid_position: Vec2::new(x as f32, 0.0),
                wall_type: WallType::Boundary,
                health: u32::MAX,
            },
            GridPosition::new(x as i32, 0),
        ));
    }
    
    // Left and right boundaries
    for y in 1..(grid_height - 1) {
        // Left wall
        commands.spawn((
            Mesh2dBundle {
                mesh: wall_mesh.clone().into(),
                material: wall_material.clone(),
                transform: Transform::from_xyz(
                    -(grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.0,
                ),
                ..default()
            },
            Wall {
                grid_position: Vec2::new(0.0, y as f32),
                wall_type: WallType::Boundary,
                health: u32::MAX,
            },
            GridPosition::new(0, y as i32),
        ));
        
        // Right wall
        commands.spawn((
            Mesh2dBundle {
                mesh: wall_mesh.clone().into(),
                material: wall_material.clone(),
                transform: Transform::from_xyz(
                    (grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.0,
                ),
                ..default()
            },
            Wall {
                grid_position: Vec2::new(grid_width as f32 - 1.0, y as f32),
                wall_type: WallType::Boundary,
                health: u32::MAX,
            },
            GridPosition::new(grid_width as i32 - 1, y as i32),
        ));
    }
}

/// Get wall color based on level theme
/// FIXED: Changed all Color::rgb to Color::srgb for Bevy 0.14
fn get_wall_color_for_theme(theme: &LevelTheme) -> Color {
    match theme {
        LevelTheme::Classic => Color::srgb(0.4, 0.2, 0.1), // Brown wood
        LevelTheme::Digital => Color::srgb(0.0, 0.8, 1.0), // Cyan circuits
        LevelTheme::Forest => Color::srgb(0.3, 0.15, 0.05), // Dark bark
        LevelTheme::Desert => Color::srgb(0.8, 0.6, 0.3), // Sandstone
        LevelTheme::Ocean => Color::srgb(0.1, 0.3, 0.6), // Deep blue coral
        LevelTheme::Volcano => Color::srgb(0.6, 0.1, 0.0), // Dark lava rock
        LevelTheme::Ice => Color::srgb(0.7, 0.9, 1.0), // Ice blue
        LevelTheme::Space => Color::srgb(0.3, 0.3, 0.4), // Metallic gray
        LevelTheme::NeonCity => Color::srgb(0.8, 0.0, 0.8), // Neon purple
        LevelTheme::FinalBoss => Color::srgb(0.5, 0.0, 0.0), // Ominous red
        LevelTheme::Cyber => Color::srgb(0.0, 1.0, 0.0), // Matrix green
        LevelTheme::Shadow => Color::srgb(0.2, 0.1, 0.3), // Dark purple
        LevelTheme::Cosmic => Color::srgb(0.1, 0.1, 0.2), // Deep space blue
    }
}

/// Setup level-specific walls and obstacles
fn setup_level_walls(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    match level_def.wall_pattern {
        WallPattern::Empty => {
            // No additional walls beyond boundaries
        },
        WallPattern::BasicObstacles => {
            setup_basic_obstacles(commands, level_def, meshes, materials);
        },
        WallPattern::Maze => {
            setup_maze_pattern(commands, level_def, meshes, materials);
        },
        WallPattern::MovingWalls => {
            setup_moving_walls(commands, level_def, meshes, materials);
        },
        WallPattern::BreakableWalls => {
            setup_breakable_walls(commands, level_def, meshes, materials);
        },
        WallPattern::MultiRoom => {
            setup_multi_room_layout(commands, level_def, meshes, materials);
        },
    }
}

/// Setup basic obstacle pattern
fn setup_basic_obstacles(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let (grid_width, grid_height) = level_def.grid_size;
    let mut rng = thread_rng();
    let wall_color = get_wall_color_for_theme(&level_def.theme);
    let wall_material = materials.add(ColorMaterial::from(wall_color));
    let wall_mesh = meshes.add(Mesh::from(Rectangle::new(
        crate::GRID_SIZE, crate::GRID_SIZE
    )));
    
    // Place random obstacles (about 5-10% of grid)
    let obstacle_count = (grid_width * grid_height / 15) as usize;
    
    for _ in 0..obstacle_count {
        let x = rng.gen_range(3..(grid_width - 3));
        let y = rng.gen_range(3..(grid_height - 3));
        
        commands.spawn((
            Mesh2dBundle {
                mesh: wall_mesh.clone().into(),
                material: wall_material.clone(),
                transform: Transform::from_xyz(
                    (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.0,
                ),
                ..default()
            },
            Wall {
                grid_position: Vec2::new(x as f32, y as f32),
                wall_type: WallType::Obstacle,
                health: u32::MAX,
            },
            GridPosition::new(x as i32, y as i32),
        ));
    }
}

/// Setup maze pattern
fn setup_maze_pattern(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Implementation for maze generation
    // This would create a navigable maze pattern
    let (grid_width, grid_height) = level_def.grid_size;
    let wall_color = get_wall_color_for_theme(&level_def.theme);
    let wall_material = materials.add(ColorMaterial::from(wall_color));
    let wall_mesh = meshes.add(Mesh::from(Rectangle::new(
        crate::GRID_SIZE, crate::GRID_SIZE
    )));
    
    // Simple maze pattern - create corridors
    for x in (4..grid_width - 4).step_by(4) {
        for y in 2..(grid_height - 2) {
            if y % 4 != 0 { // Leave gaps for corridors
                commands.spawn((
                    Mesh2dBundle {
                        mesh: wall_mesh.clone().into(),
                        material: wall_material.clone(),
                        transform: Transform::from_xyz(
                            (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                            (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                            0.0,
                        ),
                        ..default()
                    },
                    Wall {
                        grid_position: Vec2::new(x as f32, y as f32),
                        wall_type: WallType::Obstacle,
                        health: u32::MAX,
                    },
                    GridPosition::new(x as i32, y as i32),
                ));
            }
        }
    }
}

/// Setup moving walls
fn setup_moving_walls(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let (grid_width, grid_height) = level_def.grid_size;
    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let wall_color = Color::srgb(0.8, 0.4, 0.0); // Orange for moving walls
    let wall_material = materials.add(ColorMaterial::from(wall_color));
    let wall_mesh = meshes.add(Mesh::from(Rectangle::new(
        crate::GRID_SIZE, crate::GRID_SIZE
    )));
    
    // Create a few moving walls
    for i in 0..3 {
        let start_x = 5 + i * 10;
        let y = grid_height / 2;
        
        commands.spawn((
            Mesh2dBundle {
                mesh: wall_mesh.clone().into(),
                material: wall_material.clone(),
                transform: Transform::from_xyz(
                    (start_x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.0,
                ),
                ..default()
            },
            Wall {
                grid_position: Vec2::new(start_x as f32, y as f32),
                wall_type: WallType::Moving,
                health: u32::MAX,
            },
            GridPosition::new(start_x as i32, y as i32),
            SmoothMovement {
                start_position: Vec2::new(start_x as f32, y as f32),
                target_position: Vec2::new((start_x + 5) as f32, y as f32),
                progress: 0.0,
                duration: 3.0, // 3 seconds to move
                easing: EasingType::Linear,
            },
        ));
    }
}

/// Setup breakable walls
fn setup_breakable_walls(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let (grid_width, grid_height) = level_def.grid_size;
    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let wall_color = Color::srgb(0.6, 0.4, 0.2); // Brown for breakable walls
    let wall_material = materials.add(ColorMaterial::from(wall_color));
    let wall_mesh = meshes.add(Mesh::from(Rectangle::new(
        crate::GRID_SIZE, crate::GRID_SIZE
    )));
    
    // Create clusters of breakable walls
    let mut rng = thread_rng();
    for _ in 0..5 {
        let center_x = rng.gen_range(5..(grid_width - 5));
        let center_y = rng.gen_range(5..(grid_height - 5));
        
        // Create a 3x3 cluster
        for dx in -1..=1 {
            for dy in -1..=1 {
                if rng.gen_bool(0.7) { // 70% chance for each wall in cluster
                    let x = (center_x as i32 + dx) as u32;
                    let y = (center_y as i32 + dy) as u32;
                    
                    commands.spawn((
                        Mesh2dBundle {
                            mesh: wall_mesh.clone().into(),
                            material: wall_material.clone(),
                            transform: Transform::from_xyz(
                                (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                                (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                                0.0,
                            ),
                            ..default()
                        },
                        Wall {
                            grid_position: Vec2::new(x as f32, y as f32),
                            wall_type: WallType::Breakable,
                            health: 1, // Can be broken by snake
                        },
                        GridPosition::new(x as i32, y as i32),
                    ));
                }
            }
        }
    }
}

/// Setup multi-room layout
fn setup_multi_room_layout(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let (grid_width, grid_height) = level_def.grid_size;
    let wall_color = get_wall_color_for_theme(&level_def.theme);
    let wall_material = materials.add(ColorMaterial::from(wall_color));
    let wall_mesh = meshes.add(Mesh::from(Rectangle::new(
        crate::GRID_SIZE, crate::GRID_SIZE
    )));
    
    // Divide the level into 4 rooms with connecting passages
    let mid_x = grid_width / 2;
    let mid_y = grid_height / 2;
    
    // Vertical divider (with gaps for passages)
    for y in 2..(grid_height - 2) {
        if y != mid_y - 1 && y != mid_y && y != mid_y + 1 { // Leave passage
            commands.spawn((
                Mesh2dBundle {
                    mesh: wall_mesh.clone().into(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        (mid_x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                        (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                Wall {
                    grid_position: Vec2::new(mid_x as f32, y as f32),
                    wall_type: WallType::Obstacle,
                    health: u32::MAX,
                },
                GridPosition::new(mid_x as i32, y as i32),
            ));
        }
    }
    
    // Horizontal divider (with gaps for passages)
    for x in 2..(grid_width - 2) {
        if x != mid_x - 1 && x != mid_x && x != mid_x + 1 { // Leave passage
            commands.spawn((
                Mesh2dBundle {
                    mesh: wall_mesh.clone().into(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                        (mid_y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                Wall {
                    grid_position: Vec2::new(x as f32, mid_y as f32),
                    wall_type: WallType::Obstacle,
                    health: u32::MAX,
                },
                GridPosition::new(x as i32, mid_y as i32),
            ));
        }
    }
}

/// Setup special mechanics for the level
fn setup_special_mechanics(
    commands: &mut Commands,
    level_def: &LevelDefinition,
) {
    for mechanic in &level_def.special_mechanics {
        match mechanic {
            SpecialMechanic::Teleporters => setup_teleporters(commands, level_def),
            SpecialMechanic::SpeedZones => setup_speed_zones(commands, level_def),
            SpecialMechanic::MovingFood => {
                // Moving food will be handled in food system
            },
            SpecialMechanic::Invincibility => setup_invincibility_pickups(commands, level_def),
            SpecialMechanic::WallBreaking => {
                // Wall breaking is handled by character abilities
            },
            SpecialMechanic::MultipleFoods => {
                // Multiple foods handled in food spawning system
            },
            SpecialMechanic::Gravity => setup_gravity_zones(commands, level_def),
            SpecialMechanic::Trail => {
                // Trail effect handled in snake movement system
            },
        }
    }
}

/// Setup teleporter portals
fn setup_teleporters(commands: &mut Commands, level_def: &LevelDefinition) {
    let (grid_width, grid_height) = level_def.grid_size;
    
    // Create entrance and exit teleporters
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
                color: Color::srgb(0.5, 0.0, 1.0), // Purple
                custom_size: Some(Vec2::new(crate::GRID_SIZE, crate::GRID_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                (5.0 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                (5.0 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                1.0,
            ),
            ..default()
        },
        GridPosition::new(5, 5),
        AudioTrigger {
            sound_id: "teleport".to_string(),
            triggered: false,
            trigger_condition: AudioTriggerCondition::SnakeEnters,
        },
    ));
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
                color: Color::srgb(0.5, 0.0, 1.0), // Purple
                custom_size: Some(Vec2::new(crate::GRID_SIZE, crate::GRID_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                ((grid_width - 5) as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                ((grid_height - 5) as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                1.0,
            ),
            ..default()
        },
        GridPosition::new(grid_width as i32 - 5, grid_height as i32 - 5),
        AudioTrigger {
            sound_id: "teleport".to_string(),
            triggered: false,
            trigger_condition: AudioTriggerCondition::SnakeEnters,
        },
    ));
}

/// Setup speed zones
fn setup_speed_zones(commands: &mut Commands, level_def: &LevelDefinition) {
    let (grid_width, grid_height) = level_def.grid_size;
    let mut rng = thread_rng();
    
    // Create a few speed zones
    for _ in 0..3 {
        let x = rng.gen_range(5..(grid_width - 5));
        let y = rng.gen_range(5..(grid_height - 5));
        let zone_size = rng.gen_range(3..6);
        
        // Create speed boost zone
        for dx in 0..zone_size {
            for dy in 0..zone_size {
                let zone_x = x + dx;
                let zone_y = y + dy;
                
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            // FIXED: Changed Color::rgba to Color::srgba for Bevy 0.14
                            color: Color::srgba(1.0, 1.0, 0.0, 0.3), // Yellow with transparency
                            custom_size: Some(Vec2::new(crate::GRID_SIZE, crate::GRID_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (zone_x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                            (zone_y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                            0.5,
                        ),
                        ..default()
                    },
                    GridPosition::new(zone_x as i32, zone_y as i32),
                ));
            }
        }
    }
}

/// Setup invincibility power-up pickups
fn setup_invincibility_pickups(commands: &mut Commands, level_def: &LevelDefinition) {
    let (grid_width, grid_height) = level_def.grid_size;
    let mut rng = thread_rng();
    
    // Place a few invincibility pickups
    for _ in 0..2 {
        let x = rng.gen_range(3..(grid_width - 3));
        let y = rng.gen_range(3..(grid_height - 3));
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
                    color: Color::srgb(0.0, 1.0, 1.0), // Cyan
                    custom_size: Some(Vec2::new(crate::GRID_SIZE * 0.8, crate::GRID_SIZE * 0.8)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (x as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (y as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    1.0,
                ),
                ..default()
            },
            GridPosition::new(x as i32, y as i32),
            Food {
                grid_position: Vec2::new(x as f32, y as f32),
                score_value: 50,
                food_type: FoodType::Speed, // Reuse speed type for special effects
                expiration_timer: Some(30.0), // Expires after 30 seconds
                pulse_phase: 0.0,
            },
            AnimatedSprite {
                current_frame: 0,
                frame_count: 8,
                frame_duration: 0.1,
                frame_timer: 0.0,
                loops: true,
                is_playing: true,
            },
        ));
    }
}

/// Setup gravity zones
fn setup_gravity_zones(commands: &mut Commands, level_def: &LevelDefinition) {
    let (grid_width, grid_height) = level_def.grid_size;
    
    // Create gravity wells in corners
    let gravity_positions = vec![
        (grid_width / 4, grid_height / 4),
        (3 * grid_width / 4, grid_height / 4),
        (grid_width / 4, 3 * grid_height / 4),
        (3 * grid_width / 4, 3 * grid_height / 4),
    ];
    
    for (gx, gy) in gravity_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    // FIXED: Changed Color::rgba to Color::srgba for Bevy 0.14
                    color: Color::srgba(0.5, 0.0, 0.5, 0.4), // Purple with transparency
                    custom_size: Some(Vec2::new(crate::GRID_SIZE * 3.0, crate::GRID_SIZE * 3.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (gx as f32 - grid_width as f32 / 2.0) * crate::GRID_SIZE,
                    (gy as f32 - grid_height as f32 / 2.0) * crate::GRID_SIZE,
                    0.2,
                ),
                ..default()
            },
            GridPosition::new(gx as i32, gy as i32),
        ));
    }
}

/// Setup level UI elements
fn setup_level_ui(
    _commands: &mut Commands,
    _level_def: &LevelDefinition,
    _asset_handles: &AssetHandles,
) {
    // Level name display
    /*
    commands.spawn((
        TextBundle::from_section(
            &level_def.name,
            TextStyle {
                font: asset_handles.fonts.get("main_font")
                    .cloned()
                    .unwrap_or_default(),
                font_size: 24.0,
                // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
                color: Color::srgb(0.8, 0.8, 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
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
    
    // Level description (appears briefly at start)
    commands.spawn((
        TextBundle::from_section(
            &level_def.description,
            TextStyle {
                font: asset_handles.fonts.get("main_font")
                    .cloned()
                    .unwrap_or_default(),
                font_size: 16.0,
                color: Color::srgb(0.8, 0.8, 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(20.0),
            ..default()
        }),
        UIElement {
            element_type: UIElementType::Subtitle,
            animation: Some(UIAnimation {
                animation_type: UIAnimationType::FadeOut,
                timer: 0.0,
                duration: 3.0,
                loops: false,
            }),
            is_visible: true,
            layer: 100,
        },
    ));
    */
}

/// Apply character-specific modifications to the level
fn apply_character_modifications(
    _commands: &mut Commands,
    character_selection: &CharacterSelection,
    _level_def: &LevelDefinition,
) {
    let character = &character_selection.characters[(character_selection.selected_character - 1) as usize];
    
    match character.special_ability {
        CharacterAbility::SpeedBoost => {
            // Character will move faster - handled in movement system
        },
        CharacterAbility::WallBreaker => {
            // Character can break walls - handled in collision system
        },
        CharacterAbility::ScoreBooster => {
            // Character gets bonus points - handled in scoring system
        },
        CharacterAbility::None => {
            // No special modifications
        },
    }
}

/// System to update scrolling backgrounds
/// FIXED: Resolved borrowing issue for Bevy 0.14
pub fn update_scrolling_backgrounds(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LevelBackground)>,
) {
    for (mut transform, mut background) in query.iter_mut() {
        // Store scroll_speed locally to avoid borrowing conflicts
        let scroll_speed = background.scroll_speed;
        background.scroll_offset += scroll_speed * time.delta_seconds();
        
        // Apply scrolling to transform
        transform.translation.x = background.scroll_offset.x % (crate::DEFAULT_WINDOW_WIDTH);
        transform.translation.y = background.scroll_offset.y % (crate::DEFAULT_WINDOW_HEIGHT);
    }
}

/// System to update parallax layers
pub fn update_parallax_layers(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ParallaxLayer)>,
    background_query: Query<&LevelBackground>,
) {
    if let Ok(background) = background_query.get_single() {
        for (mut transform, layer) in query.iter_mut() {
            let parallax_offset = background.scroll_offset * layer.scroll_multiplier;
            
            transform.translation.x = parallax_offset.x % (crate::DEFAULT_WINDOW_WIDTH);
            transform.translation.y = parallax_offset.y % (crate::DEFAULT_WINDOW_HEIGHT);
        }
    }
}