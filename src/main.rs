use bevy::prelude::*;
use plugins::blob::BlobPlugin;
use plugins::camera::CameraPlugin;

mod plugins;

#[derive(Resource)]
struct Settings {
    screen_size: Vec2,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, BlobPlugin))
        .run();
}
