use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(50.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb_u8(50, 2, 64),
            perceptual_roughness: 0.7,
            reflectance: 0.15,
            ..default()
        }),
        ..default()
    };

    commands.spawn(floor);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 140000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}
