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
        .add_plugins((
            DefaultPlugins,
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
