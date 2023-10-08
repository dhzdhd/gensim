use bevy::prelude::*;
use plugins::blob::BlobPlugin;
use plugins::camera::CameraPlugin;
use plugins::world::WorldPlugin;

mod plugins;

#[derive(Resource)]
struct Settings;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin, BlobPlugin))
        .insert_resource(Settings)
        .run();
}
