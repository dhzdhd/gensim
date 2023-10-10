use bevy::prelude::*;
use rand_distr::{Distribution, Normal};

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tree);
    }
}

#[derive(Component)]
pub struct Tree;

fn spawn_tree(mut commands: Commands, assets: Res<AssetServer>) {
    let mut trees: Vec<(SceneBundle, Tree)> = Vec::new();
    let normal = Normal::new(0.0, 7.0).unwrap();

    for _ in 0..10 {
        let x = normal.sample(&mut rand::thread_rng());
        let z = normal.sample(&mut rand::thread_rng());

        let tree = SceneBundle {
            scene: assets.load("Tree Blob.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(x, 0.0, z),
                scale: Vec3::splat(1.2),
                ..default()
            },
            ..default()
        };
        trees.push((tree, Tree));
    }

    commands.spawn_batch(trees);
}
