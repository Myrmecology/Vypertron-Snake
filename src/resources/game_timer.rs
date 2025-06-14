use bevy::prelude::*;

/// A repeating game timer resource used for time-based logic (e.g. tracking session duration).
#[derive(Resource, Debug)]
pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}
