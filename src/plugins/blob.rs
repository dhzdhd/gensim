use std::ops::Neg;

use bevy::prelude::*;

use crate::{
    bundles::{ColliderBundle, RigidBodyBundle},
    components::Health,
};

use super::tree::Tree;
use bevy_rapier3d::prelude::*;
use rand_distr::{Beta, Distribution};

pub struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blob)
            .add_systems(Update, (start_animations, move_blob))
            .insert_resource(HealthTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
    }
}

#[derive(Component, Default)]
struct Blob;

#[derive(Component, Default)]
enum BlobType {
    #[default]
    Prey,
    Predator,
}

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Bundle, Default)]
pub struct BlobBundle {
    scene: SceneBundle,
    speed: Speed,
    identity: Blob,
    rigid_body: RigidBodyBundle,
    health: Health,
    blob_type: BlobType,
    name: Name,
}

#[derive(Resource)]
struct HealthTimer(Timer);

const BLOB_AMOUNT: i32 = 5;

fn spawn_blob(mut commands: Commands, assets: Res<AssetServer>) {
    let mut animations = Vec::new();
    for i in 0..9 {
        animations.push(assets.load(format!("Green Blob.glb#Animation{i}")));
    }

    commands.insert_resource(Animations(animations));

    let beta = Beta::new(0.1, 0.1).unwrap();
    let positions: Vec<Vec3> = Vec::new();

    for i in 0..BLOB_AMOUNT {
        let x = beta.sample(&mut rand::thread_rng());
        let z = beta.sample(&mut rand::thread_rng());

        info!("x: {}, z: {}", x, z);

        let blob = SceneBundle {
            scene: assets.load(if i % 2 == 0 {
                "Green Spiky Blob.glb#Scene0"
            } else {
                "Green Blob.glb#Scene0"
            }),
            transform: Transform {
                translation: Vec3::new(x * 45.0, 0.0, z * 45.0),
                scale: if i % 2 == 0 {
                    Vec3::splat(0.6)
                } else {
                    Vec3::splat(1.0)
                },
                ..default()
            },
            ..default()
        };

        commands
            .spawn(BlobBundle {
                scene: blob,
                speed: Speed(5.0),
                blob_type: if i % 2 == 0 {
                    BlobType::Predator
                } else {
                    BlobType::Prey
                },
                name: Name::new(format!("Blob {i}")),
                ..default()
            })
            .with_children(|children| {
                children.spawn(ColliderBundle::new(
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.8, 0.0),
                ));
            });
    }
}

fn start_animations(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[4].clone_weak()).repeat();
    }
}

fn reduce_health(mut timer: ResMut<HealthTimer>) {}

fn move_blob(
    mut blobs: Query<(&mut Transform, &Speed), With<Blob>>,
    trees: Query<&Transform, (With<Tree>, Without<Blob>)>,
    time: Res<Time>,
) {
    if trees.is_empty() {
    } else {
        for (mut blob_transform, blob_speed) in blobs.iter_mut() {
            let nearest_tree = trees
                .iter()
                .reduce(|a, b| {
                    if a.translation.distance(blob_transform.translation)
                        <= b.translation.distance(blob_transform.translation)
                    {
                        a
                    } else {
                        b
                    }
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
                // info!(
                //     "{}",
                //     nearest_tree
                //         .translation
                //         .distance_squared(blob_transform.translation)
                // );
                blob_transform.translation +=
                    target_direction * blob_speed.0 * time.delta_seconds();
            } else {
                blob_transform.translation += Vec3::ZERO;
            }
        }
    }
}
