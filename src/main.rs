use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use plugins::blob::BlobPlugin;
use plugins::camera::CameraPlugin;
use plugins::tree::TreePlugin;
use plugins::world::WorldPlugin;

mod plugins;

#[derive(Resource)]
struct Settings;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "gensim".to_string(),
                resolution: (800., 600.).into(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            TemporalAntiAliasPlugin,
            ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
            CameraPlugin,
            WorldPlugin,
            BlobPlugin,
            TreePlugin,
        ))
        .insert_resource(Settings)
        .run();
}
