//! Visual Effects System for Vypertron-Snake
//! 
//! This module handles all visual effects including:
//! - Snake death explosions with dramatic particles
//! - Food pickup sparkle effects
//! - Wall break destruction effects
//! - Teleporter portal effects
//! - Victory celebration effects
//! - Particle system management and cleanup

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::utils::*;
use crate::audio::*;
use crate::systems::snake::*;
use rand::prelude::*;

// ===============================
// EFFECT SPAWNING EVENTS
// ===============================

/// Event to trigger explosion effects
#[derive(Event, Debug)]
pub struct ExplosionEvent {
    pub position: Vec2,
    pub explosion_type: ExplosionType,
    pub intensity: f32,
    pub character_color: Option<Color>,
}

/// Event to trigger particle effects
#[derive(Event, Debug)]
pub struct ParticleEvent {
    pub position: Vec2,
    pub particle_type: ParticleType,
    pub count: u32,
    pub color: Color,
    pub velocity_range: Vec2,
}

/// Types of particle effects
#[derive(Debug, Clone)]
pub enum ParticleType {
    Sparkle,
    Smoke,
    Fire,
    Electric,
    Magic,
    Debris,
}

// ===============================
// SNAKE DEATH EXPLOSION SYSTEM
// ===============================

/// Trigger dramatic explosion when snake dies
/// FIXED: Made snake_death_events mutable for Bevy 0.14
pub fn trigger_death_explosion(
    mut commands: Commands,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut particle_events: EventWriter<ParticleEvent>,
    mut snake_death_events: EventReader<SnakeDeathEvent>,
    _character_selection: Res<CharacterSelection>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    _time: Res<Time>,
) {
    for death_event in snake_death_events.read() {
        let character_color = ColorUtils::get_character_color(death_event.character_id);
        
        info!("Triggering death explosion at {:?}", death_event.death_position);
        
        // Main explosion
        explosion_events.send(ExplosionEvent {
            position: death_event.death_position,
            explosion_type: ExplosionType::Death,
            intensity: 1.0,
            character_color: Some(character_color),
        });
        
        // Multiple particle bursts with character color
        for i in 0..3 {
            let _delay = i as f32 * 0.2;
            particle_events.send(ParticleEvent {
                position: death_event.death_position + Vec2::new(
                    (i as f32 - 1.0) * 20.0,
                    (i as f32 - 1.0) * 10.0,
                ),
                particle_type: match death_event.death_cause {
                    DeathCause::WallCollision => ParticleType::Debris,
                    DeathCause::SelfCollision => ParticleType::Fire,
                    _ => ParticleType::Smoke,
                },
                count: 15 + (i * 5),
                color: character_color,
                velocity_range: Vec2::new(100.0 + i as f32 * 50.0, 150.0 + i as f32 * 30.0),
            });
        }
        
        // Play dramatic explosion sound sequence
        play_sound_events.send(
            PlaySoundEvent::new("explosion")
                .with_volume(1.0)
                .with_pitch(0.6)
                .at_position(death_event.death_position)
        );
        
        // Delayed second explosion for extra drama
        spawn_delayed_explosion(
            &mut commands,
            death_event.death_position,
            0.5, // 0.5 second delay
            character_color,
        );
    }
}

/// Spawn a delayed explosion effect
fn spawn_delayed_explosion(
    commands: &mut Commands,
    position: Vec2,
    delay: f32,
    color: Color,
) {
    commands.spawn((
        Transform::from_xyz(
            MathUtils::grid_to_world(position, crate::GRID_SIZE).x,
            MathUtils::grid_to_world(position, crate::GRID_SIZE).y,
            15.0,
        ),
        DelayedEffect {
            effect_type: DelayedEffectType::Explosion,
            delay_timer: delay,
            data: EffectData {
                color,
                intensity: 0.7,
                duration: 1.0,
            },
        },
    ));
}

/// Component for effects that trigger after a delay
#[derive(Component, Debug)]
pub struct DelayedEffect {
    pub effect_type: DelayedEffectType,
    pub delay_timer: f32,
    pub data: EffectData,
}

/// Types of delayed effects
#[derive(Debug, Clone)]
pub enum DelayedEffectType {
    Explosion,
    Sparkle,
    Smoke,
}

/// Data for delayed effects
#[derive(Debug, Clone)]
pub struct EffectData {
    pub color: Color,
    pub intensity: f32,
    pub duration: f32,
}

// ===============================
// EXPLOSION EFFECT SYSTEM
// ===============================

/// Handle explosion events and create visual effects
pub fn handle_explosion_events(
    mut commands: Commands,
    mut explosion_events: EventReader<ExplosionEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    for explosion_event in explosion_events.read() {
        create_explosion_effect(
            &mut commands,
            explosion_event,
            &mut meshes,
            &mut materials,
        );
        
        // Play appropriate sound based on explosion type
        let (sound, volume, pitch) = match explosion_event.explosion_type {
            ExplosionType::Death => ("explosion", 0.9, 0.8),
            ExplosionType::FoodPickup => ("food_pickup", 0.6, 1.2),
            ExplosionType::WallBreak => ("wall_hit", 0.7, 1.5),
            ExplosionType::Victory => ("level_complete", 0.8, 1.0),
        };
        
        play_sound_events.send(
            PlaySoundEvent::new(sound)
                .with_volume(volume)
                .with_pitch(pitch)
                .at_position(explosion_event.position)
        );
    }
}

/// Create visual explosion effect
fn create_explosion_effect(
    commands: &mut Commands,
    explosion_event: &ExplosionEvent,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let world_pos = MathUtils::grid_to_world(explosion_event.position, crate::GRID_SIZE);
    
    // Main explosion circle
    /// FIXED: Changed Color::rgb to Color::srgb for Bevy 0.14
    let explosion_color = explosion_event.character_color.unwrap_or(match explosion_event.explosion_type {
        ExplosionType::Death => Color::srgb(1.0, 0.2, 0.0),
        ExplosionType::FoodPickup => Color::srgb(1.0, 1.0, 0.2),
        ExplosionType::WallBreak => Color::srgb(0.8, 0.4, 0.0),
        ExplosionType::Victory => Color::srgb(0.2, 1.0, 0.2),
    });
    
    let explosion_material = materials.add(ColorMaterial::from(explosion_color));
    let explosion_size = match explosion_event.explosion_type {
        ExplosionType::Death => crate::GRID_SIZE * 2.0,
        ExplosionType::FoodPickup => crate::GRID_SIZE * 0.8,
        ExplosionType::WallBreak => crate::GRID_SIZE * 1.2,
        ExplosionType::Victory => crate::GRID_SIZE * 3.0,
    };
    
    let explosion_mesh = meshes.add(Mesh::from(shape::Circle::new(explosion_size * 0.5)));
    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: explosion_mesh.into(),
            material: explosion_material,
            transform: Transform::from_xyz(world_pos.x, world_pos.y, 20.0),
            ..default()
        },
        ExplosionEffect {
            intensity: explosion_event.intensity,
            duration: match explosion_event.explosion_type {
                ExplosionType::Death => 2.0,
                ExplosionType::FoodPickup => 0.5,
                ExplosionType::WallBreak => 0.8,
                ExplosionType::Victory => 3.0,
            },
            timer: 0.0,
            particle_count: match explosion_event.explosion_type {
                ExplosionType::Death => 25,
                ExplosionType::FoodPickup => 8,
                ExplosionType::WallBreak => 15,
                ExplosionType::Victory => 40,
            },
            explosion_type: explosion_event.explosion_type.clone(),
        },
    ));
    
    // Create ring waves for death explosions
    if matches!(explosion_event.explosion_type, ExplosionType::Death) {
        create_shockwave_rings(commands, world_pos, explosion_color, meshes, materials);
    }
}

/// Create expanding shockwave rings for dramatic death explosions
fn create_shockwave_rings(
    commands: &mut Commands,
    position: Vec2,
    color: Color,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    for i in 0..3 {
        // FIXED: Updated color access for Bevy 0.14
        let color_srgba = color.to_srgba();
        let ring_color = Color::srgba(color_srgba.red, color_srgba.green, color_srgba.blue, 0.6 - i as f32 * 0.2);
        let ring_material = materials.add(ColorMaterial::from(ring_color));
        let ring_mesh = meshes.add(Mesh::from(shape::Circle::new(crate::GRID_SIZE * 0.2)));
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: ring_mesh.into(),
                material: ring_material,
                transform: Transform::from_xyz(position.x, position.y, 18.0 - i as f32),
                ..default()
            },
            ShockwaveRing {
                start_radius: crate::GRID_SIZE * 0.2,
                target_radius: crate::GRID_SIZE * (3.0 + i as f32),
                expansion_speed: 200.0 + i as f32 * 50.0,
                lifetime: 1.5 + i as f32 * 0.3,
                age: 0.0,
            },
        ));
    }
}

/// Component for shockwave ring effects
#[derive(Component, Debug)]
pub struct ShockwaveRing {
    pub start_radius: f32,
    pub target_radius: f32,
    pub expansion_speed: f32,
    pub lifetime: f32,
    pub age: f32,
}

// ===============================
// PARTICLE SYSTEM
// ===============================

/// Handle particle events and spawn particle systems
pub fn handle_particle_events(
    mut commands: Commands,
    mut particle_events: EventReader<ParticleEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for particle_event in particle_events.read() {
        spawn_particle_system(
            &mut commands,
            particle_event,
            &mut meshes,
            &mut materials,
        );
    }
}

/// Spawn a particle system
fn spawn_particle_system(
    commands: &mut Commands,
    particle_event: &ParticleEvent,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let world_pos = MathUtils::grid_to_world(particle_event.position, crate::GRID_SIZE);
    let mut rng = thread_rng();
    
    for _ in 0..particle_event.count {
        let particle_color = RandomUtils::random_color_variation(
            particle_event.color,
            0.3,
            &mut rng,
        );
        
        let particle_material = materials.add(ColorMaterial::from(particle_color));
        let particle_size = match particle_event.particle_type {
            ParticleType::Sparkle => rng.gen_range(2.0..6.0),
            ParticleType::Smoke => rng.gen_range(8.0..16.0),
            ParticleType::Fire => rng.gen_range(4.0..10.0),
            ParticleType::Electric => rng.gen_range(1.0..4.0),
            ParticleType::Magic => rng.gen_range(6.0..12.0),
            ParticleType::Debris => rng.gen_range(3.0..8.0),
        };
        
        let particle_mesh = meshes.add(Mesh::from(shape::Circle::new(particle_size)));
        
        // Random velocity within range
        let velocity_magnitude = rng.gen_range(
            particle_event.velocity_range.x..particle_event.velocity_range.y
        );
        let velocity_angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let velocity = Vec2::new(
            velocity_angle.cos() * velocity_magnitude,
            velocity_angle.sin() * velocity_magnitude,
        );
        
        // Particle position with slight randomization
        let offset = Vec2::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        );
        
        // FIXED: Updated color access for Bevy 0.14
        let particle_srgba = particle_color.to_srgba();
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: particle_mesh.into(),
                material: particle_material,
                transform: Transform::from_xyz(
                    world_pos.x + offset.x,
                    world_pos.y + offset.y,
                    15.0 + rng.gen_range(0.0..5.0),
                ),
                ..default()
            },
            Particle {
                velocity,
                lifetime: match particle_event.particle_type {
                    ParticleType::Sparkle => rng.gen_range(0.5..1.5),
                    ParticleType::Smoke => rng.gen_range(1.0..3.0),
                    ParticleType::Fire => rng.gen_range(0.8..2.0),
                    ParticleType::Electric => rng.gen_range(0.2..0.8),
                    ParticleType::Magic => rng.gen_range(1.5..3.5),
                    ParticleType::Debris => rng.gen_range(1.0..2.5),
                },
                age: 0.0,
                start_color: particle_color,
                end_color: Color::srgba(particle_srgba.red, particle_srgba.green, particle_srgba.blue, 0.0),
                start_scale: 1.0,
                end_scale: match particle_event.particle_type {
                    ParticleType::Sparkle => 0.1,
                    ParticleType::Smoke => 2.0,
                    ParticleType::Fire => 0.3,
                    ParticleType::Electric => 0.0,
                    ParticleType::Magic => 1.5,
                    ParticleType::Debris => 0.5,
                },
            },
        ));
    }
}

// ===============================
// EFFECT UPDATE SYSTEMS
// ===============================

/// Update explosion effects
pub fn animate_explosion_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut ExplosionEffect), With<ExplosionEffect>>,
) {
    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer += time.delta_seconds();
        let progress = explosion.timer / explosion.duration;
        
        if progress >= 1.0 {
            // Explosion finished
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        // Update explosion visual based on type and progress
        match explosion.explosion_type {
            ExplosionType::Death => {
                // Expanding circle with fade
                let scale = 1.0 + progress * 2.0;
                let _alpha = (1.0 - progress).powf(0.5);
                transform.scale = Vec3::splat(scale);
                // Note: In a real implementation, we'd also update material alpha
            },
            ExplosionType::FoodPickup => {
                // Quick burst and fade
                let scale = if progress < 0.3 {
                    1.0 + progress * 3.0
                } else {
                    2.0 - (progress - 0.3) * 1.4
                };
                transform.scale = Vec3::splat(scale.max(0.1));
            },
            ExplosionType::WallBreak => {
                // Jagged expansion
                let base_scale = 1.0 + progress * 1.5;
                let jitter = (explosion.timer * 20.0).sin() * 0.1;
                transform.scale = Vec3::splat(base_scale + jitter);
            },
            ExplosionType::Victory => {
                // Pulsing celebration
                let pulse = (explosion.timer * 4.0).sin() * 0.3 + 1.0;
                let scale = pulse * (1.0 + progress * 0.5);
                transform.scale = Vec3::splat(scale);
            },
        }
    }
}

/// Update shockwave ring effects
pub fn update_shockwave_rings(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut ShockwaveRing), With<ShockwaveRing>>,
) {
    for (entity, mut transform, mut ring) in query.iter_mut() {
        ring.age += time.delta_seconds();
        
        if ring.age >= ring.lifetime {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        // Expand ring
        let progress = ring.age / ring.lifetime;
        let _eased_progress = AnimationUtils::apply_easing(progress, &EasingType::EaseOut);
        
        let current_radius = ring.start_radius + 
            (ring.target_radius - ring.start_radius) * progress;
        
        // Update scale to represent radius
        let scale = current_radius / ring.start_radius;
        transform.scale = Vec3::splat(scale);
        
        // Fade out over time
        let _alpha = (1.0 - progress).powf(2.0);
        // Note: Alpha would be applied to material in real implementation
    }
}

/// Update particle effects
pub fn update_particle_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Particle), With<Particle>>,
) {
    for (entity, mut transform, mut particle) in query.iter_mut() {
        particle.age += time.delta_seconds();
        
        if particle.age >= particle.lifetime {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        let progress = particle.age / particle.lifetime;
        
        // Update position based on velocity
        transform.translation.x += particle.velocity.x * time.delta_seconds();
        transform.translation.y += particle.velocity.y * time.delta_seconds();
        
        // Apply gravity (particles fall down over time)
        particle.velocity.y -= 200.0 * time.delta_seconds();
        
        // Update scale based on particle lifecycle
        let scale = MathUtils::lerp(particle.start_scale, particle.end_scale, progress);
        transform.scale = Vec3::splat(scale);
        
        // Fade out over time
        let _alpha = (1.0 - progress).powf(0.5);
        // Note: Alpha would be applied to material color in real implementation
    }
}

/// Update delayed effects
pub fn update_delayed_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut DelayedEffect)>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut particle_events: EventWriter<ParticleEvent>,
) {
    for (entity, transform, mut delayed_effect) in query.iter_mut() {
        delayed_effect.delay_timer -= time.delta_seconds();
        
        if delayed_effect.delay_timer <= 0.0 {
            // Trigger the delayed effect
            let position = MathUtils::world_to_grid(
                transform.translation.truncate(),
                crate::GRID_SIZE,
            );
            
            match delayed_effect.effect_type {
                DelayedEffectType::Explosion => {
                    explosion_events.send(ExplosionEvent {
                        position,
                        explosion_type: ExplosionType::Death,
                        intensity: delayed_effect.data.intensity,
                        character_color: Some(delayed_effect.data.color),
                    });
                },
                DelayedEffectType::Sparkle => {
                    particle_events.send(ParticleEvent {
                        position,
                        particle_type: ParticleType::Sparkle,
                        count: 10,
                        color: delayed_effect.data.color,
                        velocity_range: Vec2::new(50.0, 100.0),
                    });
                },
                DelayedEffectType::Smoke => {
                    particle_events.send(ParticleEvent {
                        position,
                        particle_type: ParticleType::Smoke,
                        count: 15,
                        color: delayed_effect.data.color,
                        velocity_range: Vec2::new(30.0, 80.0),
                    });
                },
            }
            
            // Remove the delayed effect entity
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ===============================
// VICTORY EFFECTS SYSTEM
// ===============================

/// Create celebration effects for level completion
pub fn create_victory_effects(
    mut commands: Commands,
    mut _explosion_events: EventWriter<ExplosionEvent>,
    mut particle_events: EventWriter<ParticleEvent>,
    mut level_complete_events: EventReader<StateTransitionEvent>,
    _level_manager: Res<LevelManager>,
    character_selection: Res<CharacterSelection>,
) {
    for event in level_complete_events.read() {
        if let StateTransitionEvent::LevelComplete { score: _, level } = event {
            let character_color = ColorUtils::get_character_color(character_selection.selected_character);
            
            info!("Creating victory effects for level {}", level);
            
            // Multiple celebration explosions across the screen
            let positions = [
                Vec2::new(10.0, 15.0),
                Vec2::new(20.0, 10.0),
                Vec2::new(30.0, 18.0),
                Vec2::new(40.0, 12.0),
            ];
            
            for (i, &pos) in positions.iter().enumerate() {
                // Staggered victory explosions
                spawn_delayed_explosion(
                    &mut commands,
                    pos,
                    i as f32 * 0.3,
                    character_color,
                );
                
                // Victory sparkles
                particle_events.send(ParticleEvent {
                    position: pos,
                    particle_type: ParticleType::Magic,
                    count: 20,
                    color: ColorUtils::hsv_to_rgb(i as f32 * 90.0, 1.0, 1.0),
                    velocity_range: Vec2::new(80.0, 150.0),
                });
            }
        }
    }
}

// ===============================
// CLEANUP SYSTEMS
// ===============================

/// Clean up all effect entities
pub fn cleanup_effects(
    mut commands: Commands,
    explosion_query: Query<Entity, With<ExplosionEffect>>,
    particle_query: Query<Entity, With<Particle>>,
    shockwave_query: Query<Entity, With<ShockwaveRing>>,
    delayed_query: Query<Entity, With<DelayedEffect>>,
) {
    for entity in explosion_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    for entity in particle_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    for entity in shockwave_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    for entity in delayed_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    info!("All visual effects cleaned up");
}

/// Initialize effects system
pub fn initialize_effects_system(_commands: Commands) {
    info!("Visual effects system initialized");
}