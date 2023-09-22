use bevy::prelude::*;
use blob::BlobPlugin;
use camera::CameraPlugin;

mod blob;
mod camera;

#[derive(Resource)]
struct Settings {
    screen_size: Vec2,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, BlobPlugin))
        .run();
}
