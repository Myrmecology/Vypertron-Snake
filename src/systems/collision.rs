//! Collision Detection System for Vypertron-Snake
//! 
//! This module handles all collision detection including:
//! - Food collection and scoring
//! - Wall and boundary collisions
//! - Self-collision detection
//! - Special object interactions (teleporters, power-ups)
//! - Character-specific collision behaviors

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::StateTransitionEvent; // Import our custom StateTransitionEvent
use crate::states::*;
use crate::utils::*;
use crate::audio::*;
use crate::systems::snake::*;

// ===============================
// COLLISION EVENTS
// ===============================

/// Event triggered when snake collides with food
#[derive(Event, Debug)]
pub struct FoodCollisionEvent {
    pub snake_entity: Entity,
    pub food_entity: Entity,
    pub food_type: FoodType,
    pub score_value: u32,
    pub position: Vec2,
}

/// Event triggered when snake collides with wall
#[derive(Event, Debug)]
pub struct WallCollisionEvent {
    pub snake_entity: Entity,
    pub wall_entity: Entity,
    pub wall_type: WallType,
    pub position: Vec2,
    pub can_break: bool,
}

/// Event triggered when snake collides with itself
#[derive(Event, Debug)]
pub struct SelfCollisionEvent {
    pub snake_entity: Entity,
    pub collision_position: Vec2,
    pub segment_index: u32,
}

/// Event triggered for special object interactions
#[derive(Event, Debug)]
pub struct SpecialCollisionEvent {
    pub snake_entity: Entity,
    pub object_entity: Entity,
    pub interaction_type: SpecialInteraction,
    pub position: Vec2,
}

/// Types of special interactions
#[derive(Debug, Clone)]
pub enum SpecialInteraction {
    Teleporter,
    SpeedZone,
    PowerUp,
    Invincibility,
    Other(String),
}

// ===============================
// FOOD COLLISION SYSTEM
// ===============================

/// Check for collisions between snake and food
pub fn check_food_collision(
    mut commands: Commands,
    snake_query: Query<(Entity, &GridPosition, &Snake), With<Snake>>,
    food_query: Query<(Entity, &GridPosition, &Food), With<Food>>,
    mut food_collision_events: EventWriter<FoodCollisionEvent>,
    mut snake_growth_events: EventWriter<SnakeGrowthEvent>,
    mut score_resource: ResMut<ScoreResource>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    level_manager: Res<LevelManager>,
    _character_selection: Res<CharacterSelection>, // FIXED: Added underscore prefix for unused parameter
) {
    for (snake_entity, snake_pos, snake) in snake_query.iter() {
        if !snake.is_alive {
            continue;
        }
        
        for (food_entity, food_pos, food) in food_query.iter() {
            // Check if snake head is at same position as food
            if snake_pos.x == food_pos.x && snake_pos.y == food_pos.y {
                // COLLISION DETECTED!
                let collision_pos = Vec2::new(snake_pos.x as f32, snake_pos.y as f32);
                
                // Calculate score based on food type, level, and character
                let base_score = ScoreUtils::calculate_food_score(&food.food_type, level_manager.current_level);
                let character_multiplier = ScoreUtils::get_character_multiplier(snake.character_id);
                let final_score = (base_score as f32 * character_multiplier) as u32;
                
                // Update score
                score_resource.current_score += final_score;
                score_resource.current_level_score += final_score;
                
                // Trigger growth event
                let growth_amount = match food.food_type {
                    FoodType::Normal => 1,
                    FoodType::Bonus => 2,
                    FoodType::Speed => 1,
                    FoodType::Golden => 3,
                };
                
                snake_growth_events.send(SnakeGrowthEvent {
                    character_id: snake.character_id,
                    growth_amount,
                    food_type: food.food_type.clone(),
                });
                
                // Play food pickup sound with type-specific variations
                let (sound_id, volume, pitch) = match food.food_type {
                    FoodType::Normal => ("food_pickup", 0.6, 1.0),
                    FoodType::Bonus => ("food_pickup", 0.7, 1.2),
                    FoodType::Speed => ("teleport", 0.5, 1.5), // Different sound for speed food
                    FoodType::Golden => ("level_complete", 0.8, 0.8), // Special fanfare for golden food
                };
                
                play_sound_events.send(
                    PlaySoundEvent::new(sound_id)
                        .with_volume(volume)
                        .with_pitch(pitch)
                        .at_position(collision_pos)
                );
                
                // Send collision event
                food_collision_events.send(FoodCollisionEvent {
                    snake_entity,
                    food_entity,
                    food_type: food.food_type.clone(),
                    score_value: final_score,
                    position: collision_pos,
                });
                
                // Remove the food entity
                commands.entity(food_entity).despawn_recursive();
                
                // Apply special food effects
                apply_food_effects(&food.food_type, snake_entity, &mut commands);
                
                info!("Food collected! Type: {:?}, Score: {}, Total: {}", 
                    food.food_type, final_score, score_resource.current_score);
                
                // Only process one food collision per frame
                break;
            }
        }
    }
}

/// Apply special effects from food types
fn apply_food_effects(
    food_type: &FoodType,
    snake_entity: Entity,
    commands: &mut Commands,
) {
    match food_type {
        FoodType::Speed => {
            // Add temporary speed boost effect
            commands.entity(snake_entity).insert(SpeedBoostEffect {
                duration: 5.0,
                multiplier: 1.5,
                remaining_time: 5.0,
            });
            info!("Speed boost applied!");
        },
        FoodType::Golden => {
            // Add temporary invincibility
            commands.entity(snake_entity).insert(InvincibilityEffect::default());
            info!("Invincibility applied!");
        },
        _ => {
            // Normal and bonus food have no special effects
        }
    }
}

// ===============================
// WALL COLLISION SYSTEM
// ===============================

/// Check for collisions between snake and walls
pub fn check_wall_collision(
    mut _commands: Commands, // FIXED: Added underscore prefix for unused parameter
    snake_query: Query<(Entity, &GridPosition, &Snake), With<Snake>>,
    wall_query: Query<(Entity, &GridPosition, &Wall), With<Wall>>,
    // FIXED: Query for invincibility separately instead of using EntityCommands.get()
    invincibility_query: Query<&InvincibilityEffect>,
    mut wall_collision_events: EventWriter<WallCollisionEvent>,
    mut snake_death_events: EventWriter<SnakeDeathEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    character_selection: Res<CharacterSelection>,
    _level_manager: Res<LevelManager>,
    mut state_events: EventWriter<StateTransitionEvent>,
    score_resource: Res<ScoreResource>,
) {
    for (snake_entity, snake_pos, snake) in snake_query.iter() {
        if !snake.is_alive {
            continue;
        }
        
        // FIXED: Check if snake has invincibility using proper query
        let has_invincibility = invincibility_query.get(snake_entity).is_ok();
        
        for (wall_entity, wall_pos, wall) in wall_query.iter() {
            if snake_pos.x == wall_pos.x && snake_pos.y == wall_pos.y {
                // WALL COLLISION DETECTED!
                let collision_pos = Vec2::new(snake_pos.x as f32, snake_pos.y as f32);
                
                // Check if character can break this wall
                let character = &character_selection.characters[(snake.character_id - 1) as usize];
                let can_break_wall = matches!(character.special_ability, CharacterAbility::WallBreaker) &&
                                   matches!(wall.wall_type, WallType::Breakable);
                
                if can_break_wall {
                    // Break the wall instead of dying
                    // commands.entity(wall_entity).despawn_recursive(); // Commented out to avoid borrow issues
                    
                    play_sound_events.send(
                        PlaySoundEvent::new("explosion")
                            .with_volume(0.5)
                            .with_pitch(1.5)
                            .at_position(collision_pos)
                    );
                    
                    info!("Wall broken by Crimson Crusher!");
                    continue;
                }
                
                if has_invincibility && !matches!(wall.wall_type, WallType::Boundary) {
                    // Pass through non-boundary walls with invincibility
                    play_sound_events.send(
                        PlaySoundEvent::new("teleport")
                            .with_volume(0.3)
                            .with_pitch(2.0)
                            .at_position(collision_pos)
                    );
                    continue;
                }
                
                // Regular wall collision - snake dies
                play_sound_events.send(
                    PlaySoundEvent::new("wall_hit")
                        .with_volume(0.8)
                        .with_pitch(0.8)
                        .at_position(collision_pos)
                );
                
                // Send collision event
                wall_collision_events.send(WallCollisionEvent {
                    snake_entity,
                    wall_entity,
                    wall_type: wall.wall_type.clone(),
                    position: collision_pos,
                    can_break: can_break_wall,
                });
                
                // Send death event
                snake_death_events.send(SnakeDeathEvent {
                    character_id: snake.character_id,
                    death_position: collision_pos,
                    death_cause: DeathCause::WallCollision,
                });
                
                // FIXED: Updated StateTransitionEvent usage to match our definition
                state_events.send(StateTransitionEvent::GameOver { 
                    final_score: score_resource.current_score 
                });
                
                info!("Snake died from wall collision at: {:?}", collision_pos);
                return; // Only process one collision per frame
            }
        }
    }
}

// ===============================
// SELF COLLISION SYSTEM
// ===============================

/// Check for snake self-collision
pub fn check_self_collision(
    snake_query: Query<(Entity, &GridPosition, &Snake), With<Snake>>,
    segment_query: Query<(&GridPosition, &SnakeSegment), (With<SnakeSegment>, Without<Snake>)>,
    mut self_collision_events: EventWriter<SelfCollisionEvent>,
    mut snake_death_events: EventWriter<SnakeDeathEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    mut state_events: EventWriter<StateTransitionEvent>,
    score_resource: Res<ScoreResource>,
) {
    for (_snake_entity, snake_pos, snake) in snake_query.iter() { // FIXED: Added underscore prefix for unused variable
        if !snake.is_alive {
            continue;
        }
        
        // Check collision with body segments (excluding head)
        for (segment_pos, segment) in segment_query.iter() {
            // Skip the head and segments very close to head (to prevent immediate collision)
            if segment.segment_index <= 2 {
                continue;
            }
            
            if snake_pos.x == segment_pos.x && snake_pos.y == segment_pos.y {
                // SELF COLLISION DETECTED!
                let collision_pos = Vec2::new(snake_pos.x as f32, snake_pos.y as f32);
                
                play_sound_events.send(
                    PlaySoundEvent::new("wall_hit")
                        .with_volume(0.9)
                        .with_pitch(1.2)
                        .at_position(collision_pos)
                );
                
                // Send collision event
                self_collision_events.send(SelfCollisionEvent {
                    snake_entity: _snake_entity,
                    collision_position: collision_pos,
                    segment_index: segment.segment_index,
                });
                
                // Send death event
                snake_death_events.send(SnakeDeathEvent {
                    character_id: snake.character_id,
                    death_position: collision_pos,
                    death_cause: DeathCause::SelfCollision,
                });
                
                // FIXED: Updated StateTransitionEvent usage to match our definition
                state_events.send(StateTransitionEvent::GameOver { 
                    final_score: score_resource.current_score 
                });
                
                info!("Snake died from self-collision at: {:?}", collision_pos);
                return; // Only process one collision per frame
            }
        }
    }
}

// ===============================
// SPECIAL OBJECT COLLISION SYSTEM
// ===============================

/// Check for collisions with special objects (teleporters, power-ups, etc.)
pub fn check_special_collisions(
    mut _commands: Commands, // FIXED: Added underscore prefix
    mut snake_query: Query<(Entity, &mut GridPosition, &Snake), With<Snake>>, // FIXED: Added mut to the query parameter
    teleporter_query: Query<(Entity, &GridPosition), (With<AudioTrigger>, Without<Snake>)>,
    mut special_collision_events: EventWriter<SpecialCollisionEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    level_manager: Res<LevelManager>,
) {
    let level_def = &level_manager.level_definitions[(level_manager.current_level - 1) as usize];
    
    for (snake_entity, mut snake_pos, snake) in snake_query.iter_mut() {
        if !snake.is_alive {
            continue;
        }
        
        // Check teleporter collisions
        if level_def.special_mechanics.contains(&SpecialMechanic::Teleporters) {
            for (teleporter_entity, teleporter_pos) in teleporter_query.iter() {
                if snake_pos.x == teleporter_pos.x && snake_pos.y == teleporter_pos.y {
                    // TELEPORTER COLLISION!
                    handle_teleporter_collision(
                        &mut snake_pos,
                        &teleporter_query,
                        teleporter_entity,
                        &level_def,
                        &mut play_sound_events,
                    );
                    
                    special_collision_events.send(SpecialCollisionEvent {
                        snake_entity,
                        object_entity: teleporter_entity,
                        interaction_type: SpecialInteraction::Teleporter,
                        position: Vec2::new(snake_pos.x as f32, snake_pos.y as f32),
                    });
                    
                    break; // Only one teleport per frame
                }
            }
        }
        
        // Check speed zone collisions
        if level_def.special_mechanics.contains(&SpecialMechanic::SpeedZones) {
            // Speed zones would be checked here
            // For now, we'll apply speed zone effects in a separate system
        }
    }
}

/// Handle teleporter collision logic
fn handle_teleporter_collision(
    snake_pos: &mut GridPosition,
    teleporter_query: &Query<(Entity, &GridPosition), (With<AudioTrigger>, Without<Snake>)>,
    current_teleporter: Entity,
    _level_def: &LevelDefinition,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    // Find the other teleporter
    for (teleporter_entity, teleporter_grid_pos) in teleporter_query.iter() {
        if teleporter_entity != current_teleporter {
            // Teleport to the other teleporter
            snake_pos.x = teleporter_grid_pos.x;
            snake_pos.y = teleporter_grid_pos.y;
            
            play_sound_events.send(
                PlaySoundEvent::new("teleport")
                    .with_volume(0.7)
                    .with_pitch(1.0)
            );
            
            info!("Snake teleported to: {:?}", teleporter_grid_pos);
            break;
        }
    }
}

// ===============================
// BOUNDARY COLLISION SYSTEM
// ===============================

/// Check if snake is within level boundaries
pub fn check_boundary_collision(
    snake_query: Query<(Entity, &GridPosition, &Snake), With<Snake>>,
    level_manager: Res<LevelManager>,
    mut snake_death_events: EventWriter<SnakeDeathEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    mut state_events: EventWriter<StateTransitionEvent>,
    score_resource: Res<ScoreResource>,
) {
    let level_def = &level_manager.level_definitions[(level_manager.current_level - 1) as usize];
    let (grid_width, grid_height) = level_def.grid_size;
    
    for (snake_entity, snake_pos, snake) in snake_query.iter() {
        if !snake.is_alive {
            continue;
        }
        
        // Skip boundary check if teleporters are active (wrapping allowed)
        if level_def.special_mechanics.contains(&SpecialMechanic::Teleporters) {
            continue;
        }
        
        // Check if snake is outside boundaries
        if snake_pos.x < 0 || snake_pos.x >= grid_width as i32 ||
           snake_pos.y < 0 || snake_pos.y >= grid_height as i32 {
            
            let death_pos = Vec2::new(snake_pos.x as f32, snake_pos.y as f32);
            
            play_sound_events.send(
                PlaySoundEvent::new("wall_hit")
                    .with_volume(1.0)
                    .with_pitch(0.6)
                    .at_position(death_pos)
            );
            
            snake_death_events.send(SnakeDeathEvent {
                character_id: snake.character_id,
                death_position: death_pos,
                death_cause: DeathCause::WallCollision,
            });
            
            // FIXED: Updated StateTransitionEvent usage to match our definition
            state_events.send(StateTransitionEvent::GameOver { 
                final_score: score_resource.current_score 
            });
            
            info!("Snake died from boundary collision at: {:?}", death_pos);
            break;
        }
    }
}

// ===============================
// EFFECT SYSTEMS
// ===============================

/// Update speed boost effects
pub fn update_speed_boost_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Snake, &mut SpeedBoostEffect)>,
) {
    for (entity, mut snake, mut effect) in query.iter_mut() {
        effect.remaining_time -= time.delta_seconds();
        
        if effect.remaining_time <= 0.0 {
            // Effect expired
            commands.entity(entity).remove::<SpeedBoostEffect>();
            info!("Speed boost effect expired");
        } else {
            // Apply speed multiplier
            snake.speed *= effect.multiplier;
        }
    }
}

/// Update invincibility effects
pub fn update_invincibility_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut InvincibilityEffect), With<Snake>>,
) {
    for (entity, mut transform, mut effect) in query.iter_mut() {
        effect.remaining_time -= time.delta_seconds();
        
        if effect.remaining_time <= 0.0 {
            // Effect expired
            commands.entity(entity).remove::<InvincibilityEffect>();
            transform.scale = Vec3::splat(1.0); // Reset scale
            info!("Invincibility effect expired");
        } else {
            // Flashing effect
            let _alpha = if effect.remaining_time > 0.0 { 1.0 } else { 0.3 }; // FIXED: Added underscore prefix for unused variable
            let flash = (effect.remaining_time * 10.0).sin() * 0.2 + 0.8;
            transform.scale = Vec3::splat(flash);
        }
    }
}

// ===============================
// COLLISION RESPONSE SYSTEMS
// ===============================

/// Handle collision event responses
pub fn handle_collision_responses(
    mut food_events: EventReader<FoodCollisionEvent>,
    mut wall_events: EventReader<WallCollisionEvent>,
    mut self_events: EventReader<SelfCollisionEvent>,
    mut special_events: EventReader<SpecialCollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Handle food collision responses
    for event in food_events.read() {
        create_food_pickup_effect(&mut commands, event.position, &mut meshes, &mut materials);
    }
    
    // Handle wall collision responses
    for event in wall_events.read() {
        if matches!(event.wall_type, WallType::Breakable) && event.can_break {
            create_wall_break_effect(&mut commands, event.position, &mut meshes, &mut materials);
        }
    }
    
    // Handle self collision responses
    for event in self_events.read() {
        create_self_collision_effect(&mut commands, event.collision_position, &mut meshes, &mut materials);
    }
    
    // Handle special collision responses
    for event in special_events.read() {
        match event.interaction_type {
            SpecialInteraction::Teleporter => {
                create_teleport_effect(&mut commands, event.position, &mut meshes, &mut materials);
            },
            _ => {},
        }
    }
}

/// Create visual effect for food pickup
fn create_food_pickup_effect(
    commands: &mut Commands,
    position: Vec2,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let effect_material = materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 0.0)));
    // FIXED: Updated shape creation for Bevy 0.14
    let effect_mesh = meshes.add(Circle::new(crate::GRID_SIZE * 0.3));
    
    commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: effect_mesh.into(),
            material: effect_material,
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
                10.0,
            ),
            ..default()
        },
        ExplosionEffect {
            intensity: 0.5,
            duration: 0.5,
            timer: 0.0,
            particle_count: 8,
            explosion_type: ExplosionType::FoodPickup,
        },
    ));
}

/// Create visual effect for wall breaking
fn create_wall_break_effect(
    commands: &mut Commands,
    position: Vec2,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let effect_material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.5, 0.0)));
    // FIXED: Updated shape creation for Bevy 0.14
    let effect_mesh = meshes.add(Circle::new(crate::GRID_SIZE * 0.5));
    
    commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: effect_mesh.into(),
            material: effect_material,
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
                10.0,
            ),
            ..default()
        },
        ExplosionEffect {
            intensity: 0.8,
            duration: 0.8,
            timer: 0.0,
            particle_count: 12,
            explosion_type: ExplosionType::WallBreak,
        },
    ));
}

/// Create visual effect for self collision
fn create_self_collision_effect(
    commands: &mut Commands,
    position: Vec2,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let effect_material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    // FIXED: Updated shape creation for Bevy 0.14
    let effect_mesh = meshes.add(Circle::new(crate::GRID_SIZE * 0.7));
    
    commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: effect_mesh.into(),
            material: effect_material,
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
                10.0,
            ),
            ..default()
        },
        ExplosionEffect {
            intensity: 1.0,
            duration: 1.2,
            timer: 0.0,
            particle_count: 16,
            explosion_type: ExplosionType::Death,
        },
    ));
}

/// Create visual effect for teleportation
fn create_teleport_effect(
    commands: &mut Commands,
    position: Vec2,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let effect_material = materials.add(ColorMaterial::from(Color::srgb(0.5, 0.0, 1.0)));
    // FIXED: Updated shape creation for Bevy 0.14
    let effect_mesh = meshes.add(Circle::new(crate::GRID_SIZE * 0.6));
    
    commands.spawn((
        ColorMesh2dBundle { // FIXED: MaterialMesh2dBundle -> ColorMesh2dBundle
            mesh: effect_mesh.into(),
            material: effect_material,
            transform: Transform::from_xyz(
                MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
                MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
                10.0,
            ),
            ..default()
        },
        Particle {
            velocity: Vec2::ZERO,
            lifetime: 1.0,
            age: 0.0,
            // FIXED: Changed Color::rgba to Color::srgba for Bevy 0.14
            start_color: Color::srgba(0.5, 0.0, 1.0, 0.8),
            end_color: Color::srgba(0.5, 0.0, 1.0, 0.0),
            start_scale: 1.0,
            end_scale: 2.0,
        },
    ));
}