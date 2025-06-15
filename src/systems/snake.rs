//! Snake Movement System for Vypertron-Snake
//! 
//! This module handles all snake-related mechanics including:
//! - Smooth grid-based movement
//! - Snake growth and segment management
//! - Character-specific abilities
//! - Trail effects and animations
//! - Snake spawning and initialization

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::StateTransitionEvent; // Import our custom StateTransitionEvent
use crate::states::*;
use crate::utils::*;
use crate::audio::*;
use std::collections::VecDeque;

// ===============================
// SNAKE SPAWNING SYSTEM
// ===============================

/// Spawn the snake when entering gameplay
pub fn spawn_snake(
    mut commands: Commands,
    character_selection: Res<CharacterSelection>,
    level_manager: Res<LevelManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_handles: Res<AssetHandles>,
) {
    let current_level = level_manager.current_level;
    let level_def = &level_manager.level_definitions[(current_level - 1) as usize];
    let character = &character_selection.characters[(character_selection.selected_character - 1) as usize];
    
    info!("Spawning snake - Character: {}, Level: {}", character.name, current_level);
    
    // Calculate spawn position (center of grid)
    let (grid_width, grid_height) = level_def.grid_size;
    let spawn_x = grid_width as f32 / 2.0;
    let spawn_y = grid_height as f32 / 2.0;
    let spawn_pos = Vec2::new(spawn_x, spawn_y);
    
    // Create snake material with character color
    let snake_color = ColorUtils::get_character_color(character.id);
    let snake_material = materials.add(ColorMaterial::from(snake_color));
    
    // FIXED: Updated shape creation for Bevy 0.14
    let segment_mesh = meshes.add(Rectangle::new(
        crate::GRID_SIZE * 0.9, // Slightly smaller than grid for visual separation
        crate::GRID_SIZE * 0.9,
    ));
    
    // Spawn snake head
    let snake_entity = commands.spawn((
        MaterialMesh2dBundle {
            mesh: segment_mesh.clone().into(),
            material: snake_material.clone(),
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(spawn_pos, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(spawn_pos, crate::GRID_SIZE).y,
                5.0, // Higher z-level than background
            ),
            ..default()
        },
        Snake {
            direction: Vec2::new(1.0, 0.0), // Start moving right
            speed: level_def.starting_speed,
            move_timer: 0.0,
            length: 3, // Start with 3 segments
            is_alive: true,
            character_id: character.id,
            level: current_level,
        },
        GridPosition::new(spawn_x as i32, spawn_y as i32),
        SnakeSegment {
            segment_index: 0, // Head is index 0
            grid_position: spawn_pos,
            scale: 1.0,
            rotation: 0.0,
        },
        // Add animation component for smooth movement
        SmoothMovement {
            start_position: spawn_pos,
            target_position: spawn_pos,
            progress: 1.0, // Start completed
            duration: 0.2,
            easing: EasingType::EaseOut,
        },
    )).id();
    
    // Spawn initial body segments
    spawn_initial_segments(
        &mut commands,
        snake_entity,
        spawn_pos,
        3, // Initial length
        snake_material,
        segment_mesh,
    );
    
    info!("Snake spawned successfully at position: {:?}", spawn_pos);
}

/// Spawn initial snake body segments
fn spawn_initial_segments(
    commands: &mut Commands,
    snake_head: Entity,
    head_position: Vec2,
    length: u32,
    material: Handle<ColorMaterial>,
    mesh: Handle<Mesh>,
) {
    for i in 1..length {
        let segment_pos = Vec2::new(head_position.x - i as f32, head_position.y);
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    MathUtils::grid_to_world(segment_pos, crate::GRID_SIZE).x,
                    MathUtils::grid_to_world(segment_pos, crate::GRID_SIZE).y,
                    4.0, // Slightly lower than head
                ),
                ..default()
            },
            SnakeSegment {
                segment_index: i,
                grid_position: segment_pos,
                scale: 1.0 - (i as f32 * 0.05), // Gradually smaller segments
                rotation: 0.0,
            },
            GridPosition::new(segment_pos.x as i32, segment_pos.y as i32),
            SmoothMovement {
                start_position: segment_pos,
                target_position: segment_pos,
                progress: 1.0,
                duration: 0.2,
                easing: EasingType::EaseOut,
            },
        ));
    }
}

// ===============================
// SNAKE MOVEMENT SYSTEM
// ===============================

/// Main snake movement system
pub fn move_snake(
    time: Res<Time>,
    mut snake_query: Query<(Entity, &mut Snake, &mut Transform, &mut GridPosition), With<Snake>>,
    mut segment_query: Query<(
        &mut SnakeSegment, 
        &mut Transform, 
        &mut GridPosition,
        &mut SmoothMovement
    ), (With<SnakeSegment>, Without<Snake>)>,
    level_manager: Res<LevelManager>,
    character_selection: Res<CharacterSelection>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    mut state_events: EventWriter<StateTransitionEvent>,
) {
    for (snake_entity, mut snake, mut snake_transform, mut snake_grid_pos) in snake_query.iter_mut() {
        if !snake.is_alive {
            continue;
        }
        
        // Update movement timer
        snake.move_timer += time.delta_seconds();
        
        // Calculate movement interval based on snake speed
        let move_interval = 1.0 / snake.speed;
        
        if snake.move_timer >= move_interval {
            // Time to move!
            snake.move_timer = 0.0;
            
            // Store previous positions for segment following
            let previous_positions = collect_segment_positions(&segment_query);
            
            // Calculate new head position
            let current_pos = snake_grid_pos.to_world_position(crate::GRID_SIZE);
            let new_grid_pos = Vec2::new(
                snake_grid_pos.x as f32 + snake.direction.x,
                snake_grid_pos.y as f32 + snake.direction.y,
            );
            
            // Update head position
            snake_grid_pos.x = new_grid_pos.x as i32;
            snake_grid_pos.y = new_grid_pos.y as i32;
            
            // Apply character-specific movement modifiers
            apply_character_movement_modifiers(&mut snake, &character_selection);
            
            // Start smooth movement animation for head
            let world_pos = MathUtils::grid_to_world(new_grid_pos, crate::GRID_SIZE);
            snake_transform.translation.x = world_pos.x;
            snake_transform.translation.y = world_pos.y;
            
            // Update all body segments to follow
            update_snake_segments(
                &mut segment_query,
                &previous_positions,
                new_grid_pos,
                snake.length,
            );
            
            // Play movement sound with character-specific pitch
            let pitch_modifier = match snake.character_id {
                1 => 1.0,    // Verdant Viper - Normal
                2 => 1.2,    // Electric Eel - Higher pitch (faster)
                3 => 0.8,    // Crimson Crusher - Lower pitch (powerful)
                4 => 1.1,    // Golden Guardian - Slightly higher (elegant)
                _ => 1.0,
            };
            
            play_sound_events.send(
                PlaySoundEvent::new("snake_move")
                    .with_volume(0.25)
                    .with_pitch(pitch_modifier)
                    .at_position(new_grid_pos)
            );
            
            // Check level boundaries and handle wrapping (if teleporters are active)
            let level_def = &level_manager.level_definitions[(snake.level - 1) as usize];
            if level_def.special_mechanics.contains(&SpecialMechanic::Teleporters) {
                handle_teleporter_wrapping(&mut snake_grid_pos, level_def);
            }
            
            debug!("Snake moved to: {:?}", new_grid_pos);
        }
        
        // Update smooth movement progress for visual interpolation
        update_smooth_movement_progress(
            &time,
            &mut snake_transform,
            snake.move_timer,
            1.0 / snake.speed,
        );
    }
}

/// Collect current positions of all snake segments
fn collect_segment_positions(
    segment_query: &Query<(
        &mut SnakeSegment, 
        &mut Transform, 
        &mut GridPosition,
        &mut SmoothMovement
    ), (With<SnakeSegment>, Without<Snake>)>
) -> Vec<Vec2> {
    let mut positions_with_indices = Vec::new();
    
    for (segment, _, grid_pos, _) in segment_query.iter() {
        positions_with_indices.push((
            segment.segment_index,
            Vec2::new(grid_pos.x as f32, grid_pos.y as f32)
        ));
    }
    
    // FIXED: Sort by segment index instead of trying to compare Vec2 positions
    positions_with_indices.sort_by_key(|(index, _)| *index);
    
    // Extract just the positions
    positions_with_indices.into_iter().map(|(_, pos)| pos).collect()
}

/// Update snake body segments to follow the head
fn update_snake_segments(
    segment_query: &mut Query<(
        &mut SnakeSegment, 
        &mut Transform, 
        &mut GridPosition,
        &mut SmoothMovement
    ), (With<SnakeSegment>, Without<Snake>)>,
    previous_positions: &[Vec2],
    head_position: Vec2,
    snake_length: u32,
) {
    // Create the complete position chain: head + previous positions
    let mut position_chain = vec![head_position];
    position_chain.extend_from_slice(previous_positions);
    
    // Update each segment to follow the position chain
    for (mut segment, mut transform, mut grid_pos, mut smooth_movement) in segment_query.iter_mut() {
        let segment_index = segment.segment_index as usize;
        
        if segment_index < position_chain.len() && segment_index < snake_length as usize {
            let new_pos = position_chain[segment_index];
            
            // Update grid position
            grid_pos.x = new_pos.x as i32;
            grid_pos.y = new_pos.y as i32;
            segment.grid_position = new_pos;
            
            // Start smooth movement animation
            smooth_movement.start_position = Vec2::new(transform.translation.x, transform.translation.y);
            smooth_movement.target_position = MathUtils::grid_to_world(new_pos, crate::GRID_SIZE);
            smooth_movement.progress = 0.0;
            
            // Apply scale variation for visual interest
            let scale_variation = 1.0 - (segment_index as f32 * 0.02);
            segment.scale = scale_variation.max(0.7);
        }
    }
}

/// Apply character-specific movement modifiers
fn apply_character_movement_modifiers(
    snake: &mut Snake,
    character_selection: &CharacterSelection,
) {
    let character = &character_selection.characters[(snake.character_id - 1) as usize];
    
    match character.special_ability {
        CharacterAbility::SpeedBoost => {
            // Electric Eel - 20% speed boost
            snake.speed *= 1.2;
        },
        CharacterAbility::WallBreaker => {
            // Crimson Crusher - No movement modifier, but can break walls
        },
        CharacterAbility::ScoreBooster => {
            // Golden Guardian - No movement modifier, but bonus scoring
        },
        CharacterAbility::None => {
            // Verdant Viper - Balanced, no modifiers
        },
    }
}

/// Handle teleporter wrapping mechanics
fn handle_teleporter_wrapping(
    grid_pos: &mut GridPosition,
    level_def: &LevelDefinition,
) {
    let (grid_width, grid_height) = level_def.grid_size;
    
    // Wrap around boundaries
    if grid_pos.x < 0 {
        grid_pos.x = grid_width as i32 - 1;
    } else if grid_pos.x >= grid_width as i32 {
        grid_pos.x = 0;
    }
    
    if grid_pos.y < 0 {
        grid_pos.y = grid_height as i32 - 1;
    } else if grid_pos.y >= grid_height as i32 {
        grid_pos.y = 0;
    }
}

/// Update smooth movement visual interpolation
fn update_smooth_movement_progress(
    _time: &Time, // FIXED: Added underscore prefix for unused parameter
    _transform: &mut Transform, // FIXED: Added underscore prefix for unused parameter
    move_timer: f32,
    move_interval: f32,
) {
    // Calculate smooth interpolation progress
    let progress = (move_timer / move_interval).min(1.0);
    let _eased_progress = AnimationUtils::apply_easing(progress, &EasingType::EaseOut); // FIXED: Added underscore prefix
    
    // Apply subtle anticipation and follow-through
    let _anticipation = if progress < 0.3 { // FIXED: Added underscore prefix
        (progress / 0.3) * 0.1 // Slight pull-back
    } else {
        0.1 + ((progress - 0.3) / 0.7) * 0.9 // Forward movement
    };
    
    // This would be used for sub-grid smooth movement if desired
    // For now, we keep discrete grid movement for classic Snake feel
}

// ===============================
// SNAKE GROWTH SYSTEM
// ===============================

/// Handle snake growth when food is eaten
pub fn grow_snake(
    mut commands: Commands,
    mut snake_query: Query<&mut Snake>,
    segment_query: Query<&SnakeSegment, Without<Snake>>,
    mut growth_events: EventReader<SnakeGrowthEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    character_selection: Res<CharacterSelection>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    for growth_event in growth_events.read() {
        for mut snake in snake_query.iter_mut() {
            if snake.character_id == growth_event.character_id {
                // Increase snake length
                let old_length = snake.length;
                snake.length += growth_event.growth_amount;
                
                // Find the last segment position
                let last_segment_pos = find_last_segment_position(&segment_query, old_length);
                
                // Spawn new segments
                for i in old_length..snake.length {
                    spawn_new_segment(
                        &mut commands,
                        last_segment_pos,
                        i,
                        &mut meshes,
                        &mut materials,
                        &character_selection,
                        snake.character_id,
                    );
                }
                
                // Play growth sound with pitch based on new length
                let pitch = 1.0 + (snake.length as f32 * 0.02).min(0.5);
                play_sound_events.send(
                    PlaySoundEvent::new("food_pickup")
                        .with_volume(0.6)
                        .with_pitch(pitch)
                );
                
                // Increase speed slightly with growth
                snake.speed += 0.1;
                
                info!("Snake grew to length: {} (speed: {:.2})", snake.length, snake.speed);
            }
        }
    }
}

/// Find the position of the last snake segment
fn find_last_segment_position(
    segment_query: &Query<&SnakeSegment, Without<Snake>>,
    snake_length: u32,
) -> Vec2 {
    let mut last_pos = Vec2::ZERO;
    let mut max_index = 0;
    
    for segment in segment_query.iter() {
        if segment.segment_index >= max_index && segment.segment_index < snake_length {
            max_index = segment.segment_index;
            last_pos = segment.grid_position;
        }
    }
    
    last_pos
}

/// Spawn a new snake segment
fn spawn_new_segment(
    commands: &mut Commands,
    position: Vec2,
    segment_index: u32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    character_selection: &CharacterSelection,
    character_id: u32,
) {
    let character_color = ColorUtils::get_character_color(character_id);
    let segment_material = materials.add(ColorMaterial::from(character_color));
    
    // FIXED: Updated shape creation for Bevy 0.14
    let segment_mesh = meshes.add(Rectangle::new(
        crate::GRID_SIZE * 0.9,
        crate::GRID_SIZE * 0.9,
    ));
    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: segment_mesh.into(),
            material: segment_material,
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
                4.0 - (segment_index as f32 * 0.01), // Slightly lower z for each segment
            ),
            ..default()
        },
        SnakeSegment {
            segment_index,
            grid_position: position,
            scale: 1.0 - (segment_index as f32 * 0.02),
            rotation: 0.0,
        },
        GridPosition::new(position.x as i32, position.y as i32),
        SmoothMovement {
            start_position: position,
            target_position: position,
            progress: 1.0,
            duration: 0.2,
            easing: EasingType::EaseOut,
        },
        // Add growing animation
        AnimatedSprite {
            current_frame: 0,
            frame_count: 8,
            frame_duration: 0.05,
            frame_timer: 0.0,
            loops: false,
            is_playing: true,
        },
    ));
}

// ===============================
// SNAKE ANIMATION SYSTEM
// ===============================

/// Update snake visual animations
pub fn animate_snake(
    time: Res<Time>,
    mut snake_query: Query<&mut Transform, With<Snake>>,
    mut segment_query: Query<(&mut Transform, &mut SnakeSegment), (With<SnakeSegment>, Without<Snake>)>,
    _level_manager: Res<LevelManager>, // FIXED: Added underscore prefix for unused parameter
) {
    // Animate snake head
    for mut transform in snake_query.iter_mut() {
        // Gentle pulsing for living snake
        let pulse = AnimationUtils::pulse(time.elapsed_seconds(), 2.0) * 0.02 + 1.0;
        transform.scale = Vec3::splat(pulse);
    }
    
    // Animate body segments
    for (mut transform, mut segment) in segment_query.iter_mut() {
        // Wave animation through body
        let wave_offset = AnimationUtils::wave_offset(
            time.elapsed_seconds(),
            segment.segment_index as f32,
            1.5, // Frequency
            0.3, // Phase offset between segments
        );
        
        // Apply subtle wave motion
        segment.rotation = wave_offset * 0.05; // Slight rotation
        transform.rotation = Quat::from_rotation_z(segment.rotation);
        
        // Scale variation
        let scale_base = 1.0 - (segment.segment_index as f32 * 0.02);
        let scale_variation = AnimationUtils::pulse(
            time.elapsed_seconds() + segment.segment_index as f32 * 0.1,
            3.0
        ) * 0.01;
        
        segment.scale = scale_base + scale_variation;
        transform.scale = Vec3::splat(segment.scale);
    }
}

/// Update smooth movement animations
pub fn update_smooth_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut SmoothMovement)>,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        if movement.progress < 1.0 {
            movement.progress += time.delta_seconds() / movement.duration;
            movement.progress = movement.progress.min(1.0);
            
            let eased_progress = AnimationUtils::apply_easing(movement.progress, &movement.easing);
            
            // Interpolate position
            let current_pos = movement.start_position.lerp(movement.target_position, eased_progress);
            transform.translation.x = current_pos.x;
            transform.translation.y = current_pos.y;
        }
    }
}

// ===============================
// SNAKE TRAIL SYSTEM
// ===============================

/// Handle trail effects for certain levels
pub fn update_snake_trail(
    mut commands: Commands,
    snake_query: Query<(&Transform, &Snake)>,
    level_manager: Res<LevelManager>,
    time: Res<Time>,
    mut trail_timer: Local<f32>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let level_def = &level_manager.level_definitions[(level_manager.current_level - 1) as usize];
    
    // Only create trails if the level has trail mechanics
    if !level_def.special_mechanics.contains(&SpecialMechanic::Trail) {
        return;
    }
    
    *trail_timer += time.delta_seconds();
    
    // Create trail particles every 0.1 seconds
    if *trail_timer >= 0.1 {
        *trail_timer = 0.0;
        
        for (transform, snake) in snake_query.iter() {
            if snake.is_alive {
                // Create fading trail particle
                let trail_color = ColorUtils::get_character_color(snake.character_id);
                
                // FIXED: Use Color components correctly for Bevy 0.14
                // Convert color to linear RGB components 
                let linear_color = trail_color.to_linear();
                let trail_material = materials.add(ColorMaterial::from(
                    Color::srgba(linear_color.red, linear_color.green, linear_color.blue, 0.3)
                ));
                
                // FIXED: Updated shape creation for Bevy 0.14
                let trail_mesh = meshes.add(Rectangle::new(
                    crate::GRID_SIZE * 0.5,
                    crate::GRID_SIZE * 0.5,
                ));
                
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: trail_mesh.into(),
                        material: trail_material,
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y,
                            1.0, // Behind snake
                        ),
                        ..default()
                    },
                    Particle {
                        velocity: Vec2::ZERO,
                        lifetime: 2.0,
                        age: 0.0,
                        start_color: Color::srgba(linear_color.red, linear_color.green, linear_color.blue, 0.3), // FIXED: Color::rgba -> Color::srgba
                        end_color: Color::srgba(linear_color.red, linear_color.green, linear_color.blue, 0.0), // FIXED: Color::rgba -> Color::srgba
                        start_scale: 0.5,
                        end_scale: 0.1,
                    },
                ));
            }
        }
    }
}

// ===============================
// EVENTS
// ===============================

/// Event for snake growth
#[derive(Event)]
pub struct SnakeGrowthEvent {
    pub character_id: u32,
    pub growth_amount: u32,
    pub food_type: FoodType,
}

/// Event for snake death
#[derive(Event)]
pub struct SnakeDeathEvent {
    pub character_id: u32,
    pub death_position: Vec2,
    pub death_cause: DeathCause,
}

/// Causes of snake death
#[derive(Debug, Clone)]
pub enum DeathCause {
    WallCollision,
    SelfCollision,
    TimeLimit,
    Other(String),
}