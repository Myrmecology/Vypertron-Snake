//! Food Spawning System for Vypertron-Snake
//! 
//! This module handles all food-related mechanics including:
//! - Dynamic food spawning with safe positioning
//! - 4 food types with unique properties and effects
//! - Level-based food probability and difficulty scaling
//! - Moving food mechanics for advanced levels
//! - Food expiration and cleanup systems
//! - Visual effects and animations

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::utils::*;
use crate::audio::*;
use rand::prelude::*;

// ===============================
// FOOD SPAWNING TIMER
// ===============================

/// Resource to control food spawning timing
#[derive(Resource, Debug)]
pub struct FoodSpawnTimer {
    /// Timer for regular food spawning
    pub spawn_timer: Timer,
    /// Timer for bonus food spawning
    pub bonus_timer: Timer,
    /// Timer for special food spawning
    pub special_timer: Timer,
    /// Minimum number of food items on field
    pub min_food_count: u32,
    /// Maximum number of food items on field
    pub max_food_count: u32,
    /// Base spawn interval (seconds)
    pub base_spawn_interval: f32,
}

impl Default for FoodSpawnTimer {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            bonus_timer: Timer::from_seconds(8.0, TimerMode::Repeating),
            special_timer: Timer::from_seconds(15.0, TimerMode::Repeating),
            min_food_count: 1,
            max_food_count: 3,
            base_spawn_interval: 2.0,
        }
    }
}

// ===============================
// INITIAL FOOD SPAWNING
// ===============================

/// Spawn initial food when entering gameplay
pub fn spawn_initial_food(
    mut commands: Commands,
    level_manager: Res<LevelManager>,
    snake_query: Query<&GridPosition, With<Snake>>,
    segment_query: Query<&GridPosition, With<SnakeSegment>>,
    wall_query: Query<&GridPosition, With<Wall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut food_spawn_timer: ResMut<FoodSpawnTimer>,
) {
    let level_def = &level_manager.level_definitions[(level_manager.current_level - 1) as usize];
    let current_level = level_manager.current_level;
    
    info!("Spawning initial food for level {}", current_level);
    
    // Adjust spawn settings based on level
    adjust_spawn_settings_for_level(&mut food_spawn_timer, current_level);
    
    // Collect occupied positions
    let occupied_positions = collect_occupied_positions(&snake_query, &segment_query, &wall_query);
    
    // Spawn initial normal food
    if let Some(food_pos) = find_safe_food_position(level_def, &occupied_positions) {
        spawn_food_at_position(
            &mut commands,
            food_pos,
            FoodType::Normal,
            current_level,
            &mut meshes,
            &mut materials,
        );
    }
    
    info!("Initial food spawned successfully");
}

/// Adjust spawn timer settings based on current level
fn adjust_spawn_settings_for_level(timer: &mut FoodSpawnTimer, level: u32) {
    match level {
        1..=2 => {
            // Early levels - slow and simple
            timer.min_food_count = 1;
            timer.max_food_count = 2;
            timer.base_spawn_interval = 3.0;
        },
        3..=5 => {
            // Mid levels - moderate difficulty
            timer.min_food_count = 1;
            timer.max_food_count = 3;
            timer.base_spawn_interval = 2.5;
        },
        6..=8 => {
            // Late levels - more challenging
            timer.min_food_count = 2;
            timer.max_food_count = 4;
            timer.base_spawn_interval = 2.0;
        },
        9..=10 => {
            // Final levels - maximum challenge
            timer.min_food_count = 2;
            timer.max_food_count = 5;
            timer.base_spawn_interval = 1.5;
        },
        _ => {
            // Default fallback
            timer.min_food_count = 1;
            timer.max_food_count = 3;
            timer.base_spawn_interval = 2.0;
        },
    }
    
    // Update timers
    timer.spawn_timer = Timer::from_seconds(timer.base_spawn_interval, TimerMode::Repeating);
    timer.bonus_timer = Timer::from_seconds(timer.base_spawn_interval * 4.0, TimerMode::Repeating);
    timer.special_timer = Timer::from_seconds(timer.base_spawn_interval * 8.0, TimerMode::Repeating);
}

// ===============================
// DYNAMIC FOOD SPAWNING
// ===============================

/// Main food spawning system
pub fn spawn_food_system(
    mut commands: Commands,
    time: Res<Time>,
    mut food_spawn_timer: ResMut<FoodSpawnTimer>,
    level_manager: Res<LevelManager>,
    food_query: Query<&GridPosition, With<Food>>,
    snake_query: Query<&GridPosition, With<Snake>>,
    segment_query: Query<&GridPosition, With<SnakeSegment>>,
    wall_query: Query<&GridPosition, With<Wall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    let level_def = &level_manager.level_definitions[(level_manager.current_level - 1) as usize];
    let current_level = level_manager.current_level;
    
    // Update timers
    food_spawn_timer.spawn_timer.tick(time.delta());
    food_spawn_timer.bonus_timer.tick(time.delta());
    food_spawn_timer.special_timer.tick(time.delta());
    
    // Count current food items
    let current_food_count = food_query.iter().count() as u32;
    let occupied_positions = collect_occupied_positions(&snake_query, &segment_query, &wall_query);
    
    // Spawn regular food if needed
    if current_food_count < food_spawn_timer.min_food_count || 
       (food_spawn_timer.spawn_timer.just_finished() && current_food_count < food_spawn_timer.max_food_count) {
        
        if let Some(food_pos) = find_safe_food_position(level_def, &occupied_positions) {
            let food_type = RandomUtils::random_food_type(current_level, &mut thread_rng());
            
            spawn_food_at_position(
                &mut commands,
                food_pos,
                food_type.clone(),
                current_level,
                &mut meshes,
                &mut materials,
            );
            
            // Play spawn sound
            play_food_spawn_sound(&food_type, &mut play_sound_events);
            
            info!("Spawned {:?} food at {:?}", food_type, food_pos);
        }
    }
    
    // Spawn bonus food periodically
    if food_spawn_timer.bonus_timer.just_finished() {
        if let Some(food_pos) = find_safe_food_position(level_def, &occupied_positions) {
            spawn_food_at_position(
                &mut commands,
                food_pos,
                FoodType::Bonus,
                current_level,
                &mut meshes,
                &mut materials,
            );
            
            play_sound_events.send(PlaySoundEvent::new("menu_select").with_volume(0.4));
            info!("Spawned bonus food at {:?}", food_pos);
        }
    }
    
    // Spawn special food rarely
    if food_spawn_timer.special_timer.just_finished() {
        if let Some(food_pos) = find_safe_food_position(level_def, &occupied_positions) {
            let special_type = if thread_rng().gen_bool(0.7) {
                FoodType::Speed
            } else {
                FoodType::Golden
            };
            
            spawn_food_at_position(
                &mut commands,
                food_pos,
                special_type.clone(),
                current_level,
                &mut meshes,
                &mut materials,
            );
            
            play_food_spawn_sound(&special_type, &mut play_sound_events);
            info!("Spawned special {:?} food at {:?}", special_type, food_pos);
        }
    }
    
    // Handle multiple food spawning for advanced levels
    if level_def.special_mechanics.contains(&SpecialMechanic::MultipleFoods) {
        handle_multiple_food_spawning(
            &mut commands,
            &level_def,
            &occupied_positions,
            current_level,
            &mut meshes,
            &mut materials,
            &time,
        );
    }
}

/// Play appropriate sound for food spawning
fn play_food_spawn_sound(
    food_type: &FoodType,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    let (sound, volume, pitch) = match food_type {
        FoodType::Normal => ("menu_navigate", 0.2, 1.5),
        FoodType::Bonus => ("menu_select", 0.3, 1.2),
        FoodType::Speed => ("teleport", 0.4, 2.0),
        FoodType::Golden => ("level_complete", 0.5, 0.5),
    };
    
    play_sound_events.send(
        PlaySoundEvent::new(sound)
            .with_volume(volume)
            .with_pitch(pitch)
    );
}

/// Handle multiple food spawning for advanced levels
fn handle_multiple_food_spawning(
    commands: &mut Commands,
    level_def: &LevelDefinition,
    occupied_positions: &[Vec2],
    current_level: u32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    time: &Time,
) {
    // Spawn multiple foods simultaneously on certain intervals
    static mut MULTI_SPAWN_TIMER: f32 = 0.0;
    
    unsafe {
        MULTI_SPAWN_TIMER += time.delta_seconds();
        
        if MULTI_SPAWN_TIMER >= 20.0 { // Every 20 seconds
            MULTI_SPAWN_TIMER = 0.0;
            
            // Spawn 2-3 foods at once
            let spawn_count = thread_rng().gen_range(2..=3);
            
            for _ in 0..spawn_count {
                if let Some(food_pos) = find_safe_food_position(level_def, occupied_positions) {
                    let food_type = RandomUtils::random_food_type(current_level, &mut thread_rng());
                    spawn_food_at_position(
                        commands,
                        food_pos,
                        food_type,
                        current_level,
                        meshes,
                        materials,
                    );
                }
            }
            
            info!("Multi-spawned {} foods", spawn_count);
        }
    }
}

// ===============================
// FOOD POSITIONING
// ===============================

/// Collect all occupied grid positions - FIXED: Generic version to handle different query filters
fn collect_occupied_positions<SF, SEF, WF>(
    snake_query: &Query<&GridPosition, SF>,
    segment_query: &Query<&GridPosition, SEF>,
    wall_query: &Query<&GridPosition, WF>,
) -> Vec<Vec2> 
where
    SF: bevy::ecs::query::QueryFilter,
    SEF: bevy::ecs::query::QueryFilter,
    WF: bevy::ecs::query::QueryFilter,
{
    let mut occupied = Vec::new();
    
    // Add snake positions
    for pos in snake_query.iter() {
        occupied.push(Vec2::new(pos.x as f32, pos.y as f32));
    }
    
    // Add segment positions
    for pos in segment_query.iter() {
        occupied.push(Vec2::new(pos.x as f32, pos.y as f32));
    }
    
    // Add wall positions
    for pos in wall_query.iter() {
        occupied.push(Vec2::new(pos.x as f32, pos.y as f32));
    }
    
    occupied
}

/// Find a safe position to spawn food
fn find_safe_food_position(
    level_def: &LevelDefinition,
    occupied_positions: &[Vec2],
) -> Option<Vec2> {
    let (grid_width, grid_height) = level_def.grid_size;
    let mut rng = thread_rng();
    
    // Try to find a safe position (max 50 attempts)
    for _ in 0..50 {
        let x = rng.gen_range(1..(grid_width - 1));
        let y = rng.gen_range(1..(grid_height - 1));
        let candidate_pos = Vec2::new(x as f32, y as f32);
        
        // Check if position is safe (not occupied and not too close to snake)
        let mut is_safe = true;
        
        for &occupied_pos in occupied_positions {
            let distance = MathUtils::manhattan_distance(candidate_pos, occupied_pos);
            
            // Must be at least 2 grid units away from any occupied position
            if distance < 2.0 {
                is_safe = false;
                break;
            }
        }
        
        if is_safe {
            return Some(candidate_pos);
        }
    }
    
    warn!("Could not find safe food position after 50 attempts");
    None
}

// ===============================
// FOOD ENTITY CREATION
// ===============================

/// Spawn food at specific position
fn spawn_food_at_position(
    commands: &mut Commands,
    position: Vec2,
    food_type: FoodType,
    level: u32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let (color, size_multiplier, score_value) = get_food_visual_properties(&food_type, level);
    
    let food_material = materials.add(ColorMaterial::from(color));
    let food_size = crate::GRID_SIZE * size_multiplier;
    let food_mesh = meshes.add(Circle::new(food_size * 0.4)); // FIXED: shape::Circle -> Circle
    
    // Calculate expiration timer for special foods
    let expiration_timer = match food_type {
        FoodType::Speed => Some(10.0), // Speed food expires after 10 seconds
        FoodType::Golden => Some(15.0), // Golden food expires after 15 seconds
        _ => None, // Normal and bonus foods don't expire
    };
    
    commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: food_mesh.into(),
            material: food_material,
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
                3.0, // Above background, below snake
            ),
            ..default()
        },
        Food {
            grid_position: position,
            score_value,
            food_type: food_type.clone(),
            expiration_timer,
            pulse_phase: 0.0,
        },
        GridPosition::new(position.x as i32, position.y as i32),
        // Add pulsing animation
        AnimatedSprite {
            current_frame: 0,
            frame_count: 8,
            frame_duration: 0.1,
            frame_timer: 0.0,
            loops: true,
            is_playing: true,
        },
        // Add audio trigger for when snake approaches
        AudioTrigger {
            sound_id: get_food_approach_sound(&food_type),
            triggered: false,
            trigger_condition: AudioTriggerCondition::SnakeEnters,
        },
    ));
}

/// Get visual properties for different food types
fn get_food_visual_properties(food_type: &FoodType, level: u32) -> (Color, f32, u32) {
    match food_type {
        FoodType::Normal => (
            Color::srgb(1.0, 0.2, 0.2), // FIXED: Color::rgb -> Color::srgb
            0.7,
            ScoreUtils::calculate_food_score(food_type, level),
        ),
        FoodType::Bonus => (
            Color::srgb(0.2, 1.0, 0.2), // FIXED: Color::rgb -> Color::srgb
            0.8,
            ScoreUtils::calculate_food_score(food_type, level),
        ),
        FoodType::Speed => (
            Color::srgb(0.2, 0.2, 1.0), // FIXED: Color::rgb -> Color::srgb
            0.9,
            ScoreUtils::calculate_food_score(food_type, level),
        ),
        FoodType::Golden => (
            Color::srgb(1.0, 0.8, 0.0), // FIXED: Color::rgb -> Color::srgb
            1.0,
            ScoreUtils::calculate_food_score(food_type, level),
        ),
    }
}

/// Get approach sound for different food types
fn get_food_approach_sound(food_type: &FoodType) -> String {
    match food_type {
        FoodType::Normal => "snake_move".to_string(),
        FoodType::Bonus => "menu_navigate".to_string(),
        FoodType::Speed => "teleport".to_string(),
        FoodType::Golden => "level_complete".to_string(),
    }
}

// ===============================
// FOOD ANIMATION SYSTEM
// ===============================

/// Update food animations (pulsing, glowing, etc.)
pub fn animate_food(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Food, &AnimatedSprite), With<Food>>,
) {
    for (mut transform, mut food, _animated_sprite) in query.iter_mut() {
        // Update pulse phase
        food.pulse_phase += time.delta_seconds() * 2.0;
        
        // Different animation based on food type
        match food.food_type {
            FoodType::Normal => {
                // Gentle pulsing
                let pulse = (food.pulse_phase.sin() * 0.1 + 1.0).max(0.9);
                transform.scale = Vec3::splat(pulse);
            },
            FoodType::Bonus => {
                // Faster pulsing with color shift
                let pulse = (food.pulse_phase * 1.5).sin() * 0.15 + 1.0;
                transform.scale = Vec3::splat(pulse);
            },
            FoodType::Speed => {
                // Rapid pulsing with slight movement
                let pulse = (food.pulse_phase * 3.0).sin() * 0.2 + 1.0;
                transform.scale = Vec3::splat(pulse);
                
                // Slight bobbing movement
                let bob = (food.pulse_phase * 2.0).sin() * 2.0;
                transform.translation.y += bob - transform.translation.y.fract();
            },
            FoodType::Golden => {
                // Majestic golden glow
                let glow = (food.pulse_phase * 0.8).sin() * 0.25 + 1.0;
                transform.scale = Vec3::splat(glow);
                
                // Gentle rotation
                transform.rotation = Quat::from_rotation_z(food.pulse_phase * 0.5);
            },
        }
    }
}

// ===============================
// FOOD EXPIRATION SYSTEM
// ===============================

/// Handle food expiration for special foods
pub fn update_food_expiration(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Food, &mut Transform), With<Food>>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    for (entity, mut food, mut transform) in query.iter_mut() {
        if let Some(ref mut timer) = food.expiration_timer {
            *timer -= time.delta_seconds();
            
            // Warning phase when food is about to expire
            if *timer <= 3.0 && *timer > 0.0 {
                // Rapid flashing to indicate expiration
                let flash_rate = 8.0;
                let flash = (time.elapsed_seconds() * flash_rate).sin();
                let _alpha = if flash > 0.0 { 1.0 } else { 0.3 }; // FIXED: Added underscore prefix for unused variable
                
                // This would ideally affect the material color, but we'll use scale for now
                let warning_scale = if flash > 0.0 { 1.2 } else { 0.8 };
                transform.scale = Vec3::splat(warning_scale);
            }
            
            // Expire the food
            if *timer <= 0.0 {
                // Play expiration sound
                play_sound_events.send(
                    PlaySoundEvent::new("menu_navigate")
                        .with_volume(0.3)
                        .with_pitch(0.7)
                );
                
                // Remove the food entity
                commands.entity(entity).despawn_recursive();
                
                info!("Food {:?} expired and was removed", food.food_type);
            }
        }
    }
}

// ===============================
// MOVING FOOD SYSTEM
// ===============================

/// FIXED: Handle moving food mechanics for certain levels - with disjoint queries
pub fn update_moving_food(
    time: Res<Time>,
    mut food_query: Query<(&mut GridPosition, &mut Transform, &Food), With<Food>>,
    level_manager: Res<LevelManager>,
    snake_query: Query<&GridPosition, (With<Snake>, Without<Food>)>,
    segment_query: Query<&GridPosition, (With<SnakeSegment>, Without<Food>)>,
    wall_query: Query<&GridPosition, (With<Wall>, Without<Food>)>,
) {
    let level_def = &level_manager.level_definitions[(level_manager.current_level - 1) as usize];
    
    // Only apply moving food mechanics if the level has this feature
    if !level_def.special_mechanics.contains(&SpecialMechanic::MovingFood) {
        return;
    }
    
    static mut MOVE_TIMER: f32 = 0.0;
    
    unsafe {
        MOVE_TIMER += time.delta_seconds();
        
        // Move food every 3 seconds
        if MOVE_TIMER >= 3.0 {
            MOVE_TIMER = 0.0;
            
            let occupied_positions = collect_occupied_positions(&snake_query, &segment_query, &wall_query);
            
            for (mut grid_pos, mut transform, food) in food_query.iter_mut() {
                // Only move certain types of food
                if matches!(food.food_type, FoodType::Normal | FoodType::Bonus) {
                    // Try to find a new safe position
                    if let Some(new_pos) = find_safe_food_position(level_def, &occupied_positions) {
                        // Update positions
                        grid_pos.x = new_pos.x as i32;
                        grid_pos.y = new_pos.y as i32;
                        
                        let world_pos = MathUtils::grid_to_world(new_pos, crate::GRID_SIZE);
                        transform.translation.x = world_pos.x;
                        transform.translation.y = world_pos.y;
                        
                        info!("Moved {:?} food to new position: {:?}", food.food_type, new_pos);
                    }
                }
            }
        }
    }
}

// ===============================
// FOOD CLEANUP SYSTEM
// ===============================

/// Clean up all food entities when leaving gameplay
pub fn cleanup_food(
    mut commands: Commands,
    query: Query<Entity, With<Food>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    info!("All food entities cleaned up");
}

// ===============================
// FOOD STATISTICS SYSTEM
// ===============================

/// Track food-related statistics
pub fn update_food_statistics(
    food_query: Query<&Food>,
    mut _game_statistics: ResMut<GameStatistics>, // FIXED: Added underscore prefix for unused parameter
    time: Res<Time>,
) {
    // Count food types currently on field
    let mut food_counts = [0u32; 4]; // Normal, Bonus, Speed, Golden
    
    for food in food_query.iter() {
        let index = match food.food_type {
            FoodType::Normal => 0,
            FoodType::Bonus => 1,
            FoodType::Speed => 2,
            FoodType::Golden => 3,
        };
        food_counts[index] += 1;
    }
    
    // Update statistics (this could be used for analytics or achievements)
    // For now, we'll just track basic info
    
    // Calculate average food lifetime
    static mut TOTAL_FOOD_TIME: f32 = 0.0;
    unsafe {
        TOTAL_FOOD_TIME += time.delta_seconds();
    }
}

/// Initialize food spawn timer resource
pub fn initialize_food_system(mut commands: Commands) {
    commands.insert_resource(FoodSpawnTimer::default());
    info!("Food spawning system initialized");
}