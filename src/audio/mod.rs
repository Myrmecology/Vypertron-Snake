//! Audio System for Vypertron-Snake
//! 
//! This module manages all audio in the game including:
//! - Background music (snakesong.mp3)
//! - Procedurally generated sound effects using pure Rust
//! - Dynamic audio mixing and spatial sound
//! - Cross-platform audio support (desktop + web)

use bevy::prelude::*;
use bevy::audio::{AudioSink, AudioSinkPlayback, Volume};
use crate::resources::*;
use crate::states::*;
use std::collections::HashMap;
use std::f32::consts::PI;

// ===============================
// AUDIO PLUGIN
// ===============================

/// Main audio plugin for Vypertron-Snake
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AudioManager::default())
            .insert_resource(SoundEffectGenerator::new())
            .add_event::<PlaySoundEvent>()
            .add_event::<StopSoundEvent>()
            .add_event::<SetVolumeEvent>()
            .add_systems(Startup, initialize_audio_system)
            .add_systems(Update, (
                handle_audio_events,
                update_background_music,
                update_sound_effects,
                update_spatial_audio,
                generate_procedural_sfx,
            ))
            .add_systems(OnEnter(GameState::HomeScreen), start_background_music)
            .add_systems(OnEnter(GameState::Playing), adjust_music_for_gameplay)
            .add_systems(OnEnter(GameState::GameOver), play_game_over_sequence)
            .add_systems(OnEnter(GameState::LevelComplete), play_victory_music);
    }
}

// ===============================
// AUDIO MANAGER RESOURCE
// ===============================

/// Central audio management resource
#[derive(Resource, Debug)]
pub struct AudioManager {
    /// Background music sink for control
    pub music_sink: Option<AudioSink>,
    /// Sound effect sinks mapped by ID
    pub sfx_sinks: HashMap<String, AudioSink>,
    /// Currently playing background track
    pub current_music_track: Option<String>,
    /// Music volume (0.0 to 1.0)
    pub music_volume: f32,
    /// Sound effects volume (0.0 to 1.0)
    pub sfx_volume: f32,
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// Whether music is muted
    pub music_muted: bool,
    /// Whether sound effects are muted
    pub sfx_muted: bool,
    /// Audio library loaded assets
    pub loaded_sounds: HashMap<String, Handle<AudioSource>>,
    /// Procedural sound cache
    pub procedural_sounds: HashMap<String, Vec<f32>>,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            music_sink: None,
            sfx_sinks: HashMap::new(),
            current_music_track: None,
            music_volume: 0.6,
            sfx_volume: 0.8,
            master_volume: 0.7,
            music_muted: false,
            sfx_muted: false,
            loaded_sounds: HashMap::new(),
            procedural_sounds: HashMap::new(),
        }
    }
}

// ===============================
// SOUND EFFECT GENERATOR
// ===============================

/// Procedural sound effect generator using pure Rust
#[derive(Resource, Debug)]
pub struct SoundEffectGenerator {
    /// Sample rate for generated audio
    pub sample_rate: f32,
    /// Random seed for consistent generation
    pub seed: u64,
}

impl SoundEffectGenerator {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            seed: 12345,
        }
    }
    
    /// Generate a food pickup sound (bright, pleasant)
    pub fn generate_food_pickup(&self) -> Vec<f32> {
        let duration = 0.2; // 200ms
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Bright bell-like sound with harmonics
            let frequency_base = 800.0;
            let frequency_mod = frequency_base * (1.0 + 0.5 * (t * 20.0).sin());
            
            let wave1 = (t * frequency_mod * 2.0 * PI).sin() * 0.6;
            let wave2 = (t * frequency_mod * 3.0 * 2.0 * PI).sin() * 0.3;
            let wave3 = (t * frequency_mod * 5.0 * 2.0 * PI).sin() * 0.1;
            
            // Envelope: quick attack, smooth decay
            let envelope = if t < 0.05 {
                t / 0.05 // Attack
            } else {
                ((duration - t) / (duration - 0.05)).max(0.0) // Decay
            };
            
            let sample = (wave1 + wave2 + wave3) * envelope * 0.3;
            audio_data.push(sample);
        }
        
        audio_data
    }
    
    /// Generate snake movement sound (subtle, rhythmic)
    pub fn generate_snake_move(&self) -> Vec<f32> {
        let duration = 0.1; // 100ms
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Low frequency thump with slight pitch variation
            let frequency = 80.0 + 20.0 * (t * 40.0).sin();
            let wave = (t * frequency * 2.0 * PI).sin();
            
            // Sharp attack, quick decay
            let envelope = ((duration - t) / duration).powf(2.0);
            
            let sample = wave * envelope * 0.15; // Subtle volume
            audio_data.push(sample);
        }
        
        audio_data
    }
    
    /// Generate wall collision sound (harsh, impactful)
    pub fn generate_wall_hit(&self) -> Vec<f32> {
        let duration = 0.3; // 300ms
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Harsh noise with frequency sweep
            let frequency = 200.0 * (1.0 - t * 0.8); // Sweep down
            let noise = self.generate_noise(i) * 0.3;
            let tone = (t * frequency * 2.0 * PI).sin() * 0.7;
            
            // Sharp envelope
            let envelope = (1.0 - t / duration).powf(0.5);
            
            let sample = (tone + noise) * envelope * 0.4;
            audio_data.push(sample.clamp(-1.0, 1.0));
        }
        
        audio_data
    }
    
    /// Generate explosion sound (dramatic, explosive)
    pub fn generate_explosion(&self) -> Vec<f32> {
        let duration = 1.5; // 1.5 seconds
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Multiple explosion phases
            let phase1 = if t < 0.1 {
                // Initial bang
                let noise = self.generate_noise(i) * 0.8;
                let low_freq = (t * 60.0 * 2.0 * PI).sin() * 0.6;
                noise + low_freq
            } else if t < 0.5 {
                // Rumbling
                let rumble = (t * 40.0 * 2.0 * PI).sin() * 0.4;
                let noise = self.generate_noise(i) * 0.3;
                rumble + noise
            } else {
                // Fade out rumble
                let rumble = (t * 30.0 * 2.0 * PI).sin() * 0.2;
                rumble
            };
            
            // Envelope: sharp attack, long decay
            let envelope = if t < 0.05 {
                t / 0.05
            } else {
                ((duration - t) / (duration - 0.05)).max(0.0).powf(0.3)
            };
            
            let sample = phase1 * envelope * 0.6;
            audio_data.push(sample.clamp(-1.0, 1.0));
        }
        
        audio_data
    }
    
    /// Generate menu navigation sound (clean, digital)
    pub fn generate_menu_navigate(&self) -> Vec<f32> {
        let duration = 0.15; // 150ms
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Clean sine wave with slight frequency modulation
            let frequency = 600.0 + 100.0 * (t * 15.0).sin();
            let wave = (t * frequency * 2.0 * PI).sin();
            
            // Smooth envelope
            let envelope = (t * PI / duration).sin();
            
            let sample = wave * envelope * 0.25;
            audio_data.push(sample);
        }
        
        audio_data
    }
    
    /// Generate menu selection sound (confirmation, satisfying)
    pub fn generate_menu_select(&self) -> Vec<f32> {
        let duration = 0.4; // 400ms
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Chord progression: root, major third, perfect fifth
            let freq1 = 400.0;
            let freq2 = 400.0 * 1.25; // Major third
            let freq3 = 400.0 * 1.5;  // Perfect fifth
            
            let wave1 = (t * freq1 * 2.0 * PI).sin() * 0.4;
            let wave2 = (t * freq2 * 2.0 * PI).sin() * 0.3;
            let wave3 = (t * freq3 * 2.0 * PI).sin() * 0.3;
            
            // Bell-like envelope
            let envelope = ((duration - t) / duration).powf(0.7);
            
            let sample = (wave1 + wave2 + wave3) * envelope * 0.35;
            audio_data.push(sample);
        }
        
        audio_data
    }
    
    /// Generate level complete fanfare
    pub fn generate_level_complete(&self) -> Vec<f32> {
        let duration = 2.0; // 2 seconds
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        // Victory melody notes (in Hz)
        let melody = [523.25, 659.25, 783.99, 1046.50]; // C5, E5, G5, C6
        let note_duration = duration / melody.len() as f32;
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            let note_index = (t / note_duration) as usize;
            let note_t = (t % note_duration) / note_duration;
            
            if note_index < melody.len() {
                let frequency = melody[note_index];
                
                // Harmonic content
                let wave1 = (note_t * frequency * 2.0 * PI).sin() * 0.5;
                let wave2 = (note_t * frequency * 2.0 * 2.0 * PI).sin() * 0.3;
                let wave3 = (note_t * frequency * 3.0 * 2.0 * PI).sin() * 0.2;
                
                // Note envelope
                let envelope = if note_t < 0.1 {
                    note_t / 0.1
                } else {
                    ((note_duration - note_t * note_duration) / (note_duration * 0.9)).max(0.0)
                };
                
                let sample = (wave1 + wave2 + wave3) * envelope * 0.4;
                audio_data.push(sample);
            } else {
                audio_data.push(0.0);
            }
        }
        
        audio_data
    }
    
    /// Generate teleport sound (whoosh with pitch bend)
    pub fn generate_teleport(&self) -> Vec<f32> {
        let duration = 0.6; // 600ms
        let samples = (duration * self.sample_rate) as usize;
        let mut audio_data = Vec::with_capacity(samples);
        
        for i in 0..samples {
            let t = i as f32 / self.sample_rate;
            
            // Frequency sweep from high to low
            let frequency = 2000.0 * (1.0 - t / duration).powf(2.0) + 100.0;
            
            // Add some noise for texture
            let noise = self.generate_noise(i) * 0.2;
            let tone = (t * frequency * 2.0 * PI).sin() * 0.8;
            
            // Smooth envelope
            let envelope = (t * PI / duration).sin();
            
            let sample = (tone + noise) * envelope * 0.35;
            audio_data.push(sample);
        }
        
        audio_data
    }
    
    /// Generate simple white noise
    fn generate_noise(&self, sample_index: usize) -> f32 {
        // Simple linear congruential generator for deterministic noise
        let a = 1664525u64;
        let c = 1013904223u64;
        let m = 2u64.pow(32);
        
        let x = (a.wrapping_mul(self.seed.wrapping_add(sample_index as u64)).wrapping_add(c)) % m;
        (x as f32 / m as f32) * 2.0 - 1.0
    }
}

// ===============================
// AUDIO EVENTS
// ===============================

/// Event to play a sound effect
#[derive(Event, Debug, Clone)]
pub struct PlaySoundEvent {
    pub sound_id: String,
    pub volume: f32,
    pub pitch: f32,
    pub spatial_position: Option<Vec2>,
}

impl PlaySoundEvent {
    pub fn new(sound_id: &str) -> Self {
        Self {
            sound_id: sound_id.to_string(),
            volume: 1.0,
            pitch: 1.0,
            spatial_position: None,
        }
    }
    
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
    
    pub fn with_pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch;
        self
    }
    
    pub fn at_position(mut self, position: Vec2) -> Self {
        self.spatial_position = Some(position);
        self
    }
}

/// Event to stop a sound
#[derive(Event, Debug, Clone)]
pub struct StopSoundEvent {
    pub sound_id: String,
}

/// Event to set volume
#[derive(Event, Debug, Clone)]
pub struct SetVolumeEvent {
    pub volume_type: VolumeType,
    pub volume: f32,
}

#[derive(Debug, Clone)]
pub enum VolumeType {
    Master,
    Music,
    SoundEffects,
}

// ===============================
// AUDIO SYSTEMS
// ===============================

/// Initialize the audio system and load sounds
pub fn initialize_audio_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_manager: ResMut<AudioManager>,
    mut sound_generator: ResMut<SoundEffectGenerator>,
) {
    info!("Initializing Vypertron-Snake audio system");
    
    // Load background music
    let music_handle = asset_server.load("audio/music/snakesong.mp3");
    audio_manager.loaded_sounds.insert("background_music".to_string(), music_handle);
    
    // Generate all procedural sound effects
    let sfx_list = [
        "food_pickup",
        "snake_move", 
        "wall_hit",
        "explosion",
        "menu_navigate",
        "menu_select",
        "level_complete",
        "teleport",
    ];
    
    for sfx_name in sfx_list.iter() {
        let audio_data = match *sfx_name {
            "food_pickup" => sound_generator.generate_food_pickup(),
            "snake_move" => sound_generator.generate_snake_move(),
            "wall_hit" => sound_generator.generate_wall_hit(),
            "explosion" => sound_generator.generate_explosion(),
            "menu_navigate" => sound_generator.generate_menu_navigate(),
            "menu_select" => sound_generator.generate_menu_select(),
            "level_complete" => sound_generator.generate_level_complete(),
            "teleport" => sound_generator.generate_teleport(),
            _ => vec![],
        };
        
        audio_manager.procedural_sounds.insert(sfx_name.to_string(), audio_data);
    }
    
    info!("Generated {} procedural sound effects", sfx_list.len());
}

/// Start background music when entering home screen
pub fn start_background_music(
    mut commands: Commands,
    audio_manager: Res<AudioManager>,
    audio: Res<Audio>,
) {
    if let Some(music_handle) = audio_manager.loaded_sounds.get("background_music") {
        info!("Starting background music: snakesong.mp3");
        
        let sink = audio.play_with_settings(
            music_handle.clone(),
            PlaybackSettings::LOOP.with_volume(Volume::new_relative(audio_manager.music_volume)),
        );
        
        // Store sink in resource for later control
        // Note: In a real implementation, we'd need to properly manage the sink
    }
}

/// Adjust music for gameplay
pub fn adjust_music_for_gameplay(
    audio_manager: ResMut<AudioManager>,
) {
    info!("Adjusting music for gameplay");
    // Could lower music volume during gameplay for better sound effect audibility
}

/// Play game over sound sequence
pub fn play_game_over_sequence(
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    // Play explosion followed by dramatic pause
    play_sound_events.send(PlaySoundEvent::new("explosion").with_volume(0.8));
}

/// Play victory music
pub fn play_victory_music(
    mut play_sound_events: EventWriter<PlaySoundEvent>,
) {
    play_sound_events.send(PlaySoundEvent::new("level_complete").with_volume(0.7));
}

/// Handle audio events
pub fn handle_audio_events(
    mut play_events: EventReader<PlaySoundEvent>,
    mut stop_events: EventReader<StopSoundEvent>,
    mut volume_events: EventReader<SetVolumeEvent>,
    mut audio_manager: ResMut<AudioManager>,
    audio: Res<Audio>,
) {
    // Handle play sound events
    for event in play_events.read() {
        if let Some(audio_data) = audio_manager.procedural_sounds.get(&event.sound_id) {
            info!("Playing procedural sound: {}", event.sound_id);
            // In a real implementation, we'd convert the Vec<f32> to an AudioSource
            // and play it with the specified volume and pitch
        }
    }
    
    // Handle stop sound events
    for event in stop_events.read() {
        if let Some(sink) = audio_manager.sfx_sinks.remove(&event.sound_id) {
            sink.stop();
        }
    }
    
    // Handle volume change events
    for event in volume_events.read() {
        match event.volume_type {
            VolumeType::Master => {
                audio_manager.master_volume = event.volume;
            },
            VolumeType::Music => {
                audio_manager.music_volume = event.volume;
                if let Some(sink) = &audio_manager.music_sink {
                    sink.set_volume(event.volume);
                }
            },
            VolumeType::SoundEffects => {
                audio_manager.sfx_volume = event.volume;
            },
        }
    }
}

/// Update background music based on game state
pub fn update_background_music(
    game_state: Res<State<GameState>>,
    level_manager: Res<LevelManager>,
    mut audio_manager: ResMut<AudioManager>,
) {
    // Adjust music based on current level or game state
    match game_state.get() {
        GameState::Playing => {
            // Could add level-specific music variations here
            let level = level_manager.current_level;
            if level >= 8 {
                // Increase music intensity for final levels
            }
        },
        GameState::GameOver => {
            // Lower music volume during game over
        },
        _ => {},
    }
}

/// Update spatial audio effects
pub fn update_spatial_audio(
    snake_query: Query<&Transform, (With<crate::components::Snake>, Without<Camera>)>,
    camera_query: Query<&Transform, With<Camera>>,
    mut audio_manager: ResMut<AudioManager>,
) {
    // Calculate spatial audio based on snake position relative to camera
    if let (Ok(snake_transform), Ok(camera_transform)) = (snake_query.get_single(), camera_query.get_single()) {
        let distance = snake_transform.translation.distance(camera_transform.translation);
        
        // Adjust volume based on distance (for spatial sound effects)
        let spatial_volume = (1.0 - (distance / 1000.0).min(1.0)).max(0.1);
        
        // Apply spatial volume to relevant sound effects
        // This would be used for sounds like food pickup, wall hits, etc.
    }
}

/// Update ongoing sound effects
pub fn update_sound_effects(
    time: Res<Time>,
    mut audio_manager: ResMut<AudioManager>,
) {
    // Update any time-based sound effects
    // For example, fading out explosion sounds, looping ambient sounds, etc.
}

/// Generate and queue procedural sound effects
pub fn generate_procedural_sfx(
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    // This system would respond to game events and generate appropriate sounds
) {
    // This system responds to various game events and triggers appropriate sounds
    // For example:
    // - Snake moves -> play subtle move sound
    // - Food eaten -> play pickup sound
    // - Wall hit -> play impact sound
    // - Level complete -> play fanfare
}

// ===============================
// HELPER FUNCTIONS
// ===============================

/// Helper function to easily play sounds from systems
pub fn play_sound(
    sound_id: &str,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    play_sound_events.send(PlaySoundEvent::new(sound_id));
}

/// Helper function to play sound with custom volume
pub fn play_sound_with_volume(
    sound_id: &str,
    volume: f32,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    play_sound_events.send(PlaySoundEvent::new(sound_id).with_volume(volume));
}

/// Helper function to play spatial sound
pub fn play_spatial_sound(
    sound_id: &str,
    position: Vec2,
    play_sound_events: &mut EventWriter<PlaySoundEvent>,
) {
    play_sound_events.send(PlaySoundEvent::new(sound_id).at_position(position));
}