use bevy::prelude::*;

use crate::bundles::{ColliderBundle, RigidBodyBundle};

use super::tree::Tree;
use bevy_rapier3d::prelude::*;

pub struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blob)
            .add_systems(Update, (start_animations, move_blob));
    }
}

#[derive(Component)]
struct Blob;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Bundle)]
pub struct BlobBundle {
    scene: SceneBundle,
    speed: Speed,
}

fn spawn_blob(mut commands: Commands, assets: Res<AssetServer>) {
    let blob = SceneBundle {
        scene: assets.load("Green Blob.glb#Scene0"),
        transform: Transform {
            translation: Vec3::splat(0.0),
            scale: Vec3::splat(1.0),
            ..default()
        },
        ..default()
    };

    let mut animations = Vec::new();
    for i in 0..9 {
        animations.push(assets.load(format!("Green Blob.glb#Animation{i}")));
    }

    commands.insert_resource(Animations(animations));
    commands
        .spawn((blob, Speed(5.0), Blob, RigidBodyBundle::default()))
        .with_children(|children| {
            children.spawn(ColliderBundle::new(
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(0.0, 0.8, 0.0),
            ));
        });
}

fn start_animations(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[4].clone_weak()).repeat();
    }
}

fn move_blob(
    mut blobs: Query<(&mut Transform, &Speed), With<Blob>>,
    mut _trees: Query<&Transform, (With<Tree>, Without<Blob>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, speed) in &mut blobs {
        if input.pressed(KeyCode::W) {
            transform.translation.z -= speed.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.z += speed.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += speed.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= speed.0 * time.delta_seconds();
        }
    }
}

// fn orient_blob(mut blobs: Query<&mut Transform, With<Blob>>) {
//     for mut transform in &mut blobs {
//         // transform.
//     }
// }
