use bevy::prelude::*;
use bevy::window::{WindowTheme, WindowResolution};
use vypertron_snake::GamePlugin;

fn main() {
    // Web-specific panic hook for better error messages in browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    // Apply desktop-specific settings
    #[cfg(not(target_arch = "wasm32"))]
    desktop::configure_desktop_settings(&mut app);

    app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🐍⚡ Vypertron-Snake - Premium Snake Experience".into(),
                        resolution: WindowResolution::new(1200.0, 800.0),
                        theme: Some(WindowTheme::Dark),
                        resizable: true,
                        canvas: Some("#bevy".to_owned()), // Used for wasm bindgen canvas
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()) // Pixel-perfect rendering
                .set(AssetPlugin {
                    // Ensure proper pathing for WebAssembly builds
                    #[cfg(target_arch = "wasm32")]
                    file_path: "assets".to_string(),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(bevy::audio::GlobalVolume::new(0.7))
        .run();
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        main();
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop {
    use super::*;

    pub fn configure_desktop_settings(_app: &mut App) {
        // Add platform-specific logic here if needed (e.g., setting icon)
    }
}
