use std::ops::Neg;

use bevy::prelude::*;

use crate::bundles::{ColliderBundle, RigidBodyBundle};

use super::tree::Tree;
use bevy_rapier3d::prelude::*;

pub struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blob)
            .add_systems(Update, (start_animations, move_blob))
            .insert_resource(MoveTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
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

#[derive(Resource)]
struct MoveTimer(Timer);

fn spawn_blob(mut commands: Commands, assets: Res<AssetServer>) {
    let blob = SceneBundle {
        scene: assets.load("Green Blob.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(45.0, 0.0, 45.0),
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
    trees: Query<&Transform, (With<Tree>, Without<Blob>)>,
    time: Res<Time>,
) {
    // if timer.0.tick(time.delta()).just_finished() {
    //     info!("hello");
    // }
    for (mut blob_transform, blob_speed) in blobs.iter_mut() {
        let nearest_tree = trees
            .iter()
            .reduce(|a, b| {
                return if a.translation.distance(blob_transform.translation)
                    <= b.translation.distance(blob_transform.translation)
                {
                    a
                } else {
                    b
                };
            })
            .unwrap();

        let target_direction = (blob_transform.translation - nearest_tree.translation)
            .neg()
            .normalize();
        let current_direction = blob_transform.rotation * Vec3::new(0.0, 0.0, 1.0);

        blob_transform.rotate(Quat::from_rotation_arc(current_direction, target_direction));

        if nearest_tree
            .translation
            .distance_squared(blob_transform.translation)
            .floor()
            > 12.0
        {
            info!(
                "{}",
                nearest_tree
                    .translation
                    .distance_squared(blob_transform.translation)
            );
            blob_transform.translation += target_direction * blob_speed.0 * time.delta_seconds();
        } else {
            blob_transform.translation += Vec3::ZERO;
        }
    }
}
