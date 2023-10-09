use bevy::prelude::*;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tree);
    }
}

#[derive(Component)]
struct Tree;

fn spawn_tree(mut commands: Commands, assets: Res<AssetServer>) {
    let tree = SceneBundle {
        scene: assets.load("Tree Blob.glb#Scene0"),
        transform: Transform {
            translation: Vec3::ZERO,
            scale: Vec3::splat(1.2),
            ..default()
        },
        ..default()
    };
    commands.spawn((tree, Tree));
}