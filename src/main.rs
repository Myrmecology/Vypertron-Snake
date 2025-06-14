use bevy::prelude::*;
use bevy::window::{WindowTheme, WindowResolution};
use vypertron_snake::GamePlugin;

fn main() {
    // Initialize panic handler for better error reporting in web
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // Create the Bevy app with custom configuration
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🐍⚡ Vypertron-Snake - Premium Snake Experience".into(),
                        resolution: WindowResolution::new(1200.0, 800.0),
                        theme: Some(WindowTheme::Dark),
                        resizable: true,
                        canvas: Some("#bevy".to_owned()), // For web deployment
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()) // Pixel-perfect sprites for retro feel
                .set(AssetPlugin {
                    // Configure asset loading for web compatibility
                    #[cfg(target_arch = "wasm32")]
                    file_path: "assets".to_string(),
                    ..default()
                }),
        )
        // Add our custom game plugin
        .add_plugins(GamePlugin)
        // Set background color to match our retro theme
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        // Configure audio for cross-platform compatibility
        .insert_resource(bevy::audio::GlobalVolume::new(0.7))
        // Run the game!
        .run();
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use wasm_bindgen::prelude::*;

    // Web-specific initialization
    #[wasm_bindgen(start)]
    pub fn run() {
        main();
    }
}

// Desktop-specific optimizations
#[cfg(not(target_arch = "wasm32"))]
mod desktop {
    use super::*;
    
    pub fn configure_desktop_settings(app: &mut App) {
        // Desktop-specific configurations can go here
        // For now, we'll use the default settings
    }
}