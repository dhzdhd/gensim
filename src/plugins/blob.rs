use bevy::prelude::*;

pub struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blob)
            .add_systems(Update, move_blob);
    }
}

#[derive(Component)]
struct Blob;

#[derive(Component)]
struct Speed(f32);

#[derive(Bundle)]
pub struct BlobBundle {
    pbr: PbrBundle,
    speed: Speed,
}

fn spawn_blob(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let blob = BlobBundle {
        pbr: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::ALICE_BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        speed: Speed(2.5),
    };

    commands.spawn((blob, Blob));
}

fn move_blob(
    mut blobs: Query<(&mut Transform, &Speed), With<Blob>>,
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
