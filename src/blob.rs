use bevy::prelude::*;

pub struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blob);
    }
}

#[derive(Component)]
struct Blob;

#[derive(Component)]
struct Speed(f32);

#[derive(Bundle)]
pub struct BlobBundle {
    pbr: PbrBundle,
    blob: Blob,
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
        blob: Blob,
        speed: Speed(2.5),
    };

    commands.spawn(blob);
}
