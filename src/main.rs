use bevy::prelude::*;
use bevy::window::{WindowResolution};
use vypertron_snake::GamePlugin;

#[cfg(not(target_arch = "wasm32"))]
mod desktop {
    use super::*;

    pub fn configure_desktop_settings(_app: &mut App) {
        // Add platform-specific setup here (e.g., set app icon)
        vypertron_snake::desktop::init();
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use wasm_bindgen::prelude::*;
    use console_error_panic_hook;

    #[wasm_bindgen(start)]
    pub fn run() {
        console_error_panic_hook::set_once();
        crate::main();
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    web::run();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = App::new();

        // Desktop-specific init
        desktop::configure_desktop_settings(&mut app);

        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15))) // FIXED: rgb -> srgb
            .insert_resource(bevy::audio::GlobalVolume::new(0.7))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "🐍⚡ Vypertron-Snake - Premium Snake Experience".into(),
                            resolution: WindowResolution::new(1200.0, 800.0),
                            // FIXED: Removed 'theme' field - not available in Bevy 0.14
                            resizable: true,
                            canvas: None, // only used on web
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest())
                    .set(AssetPlugin {
                        file_path: "assets".into(), // FIXED: asset_folder -> file_path
                        ..default()
                    }),
            )
            .add_plugins(GamePlugin)
            .run();
    }
}

