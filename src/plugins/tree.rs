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

    let mut positions = Vec::new();

    for _ in 0..10 {
        let mut x = normal.sample(&mut rand::thread_rng());
        let mut z = normal.sample(&mut rand::thread_rng());

        if !positions.is_empty() {
            while positions
                .iter()
                .any(|f: &Vec2| f.distance(Vec2::new(x, z)) < 5.0)
            {
                x = normal.sample(&mut rand::thread_rng());
                z = normal.sample(&mut rand::thread_rng());
            }
        }
        error!("{positions:?}");
        positions.push(Vec2::new(x, z));

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
