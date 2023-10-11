use bevy::prelude::*;
use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

use super::blob::Speed;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        // .add_systems(Update, update_taa);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn((
        camera,
        Speed(2.0),
        ThirdPersonCamera {
            cursor_lock_active: false,
            zoom: Zoom::new(10.0, 30.0),
            ..default()
        },
    ));
}

// fn update_taa(mut camera: Query<Entity, With<Camera>>, mut commands: Commands) {
//     let camera_entity = camera.single_mut();
//     let mut camera = commands.entity(camera_entity);

//     camera.remove::<Fxaa>();
//     camera.insert(TemporalAntiAliasBundle::default());
// }
// fn move_camera(
//     input: Res<Input<KeyCode>>,
//     time: Res<Time>,
//     camera_q: Query<(&Transform, &Speed), With<Camera3d>>,
// ) {
//     let (camera_t, camera_speed) = match camera_q.get_single() {
//         Ok(c) => c,
//         Err(e) => Err(info!("{e}")).unwrap(),
//     };
//     let mut direction = Vec3::ZERO;

//     if input.pressed(KeyCode::W) {
//         direction += camera_t.forward();
//     }
//     if input.pressed(KeyCode::S) {
//         direction += camera_t.back();
//     }
//     if input.pressed(KeyCode::D) {
//         direction += camera_t.right();
//     }
//     if input.pressed(KeyCode::A) {
//         direction += camera_t.left();
//     }

//     direction.y = 0.0;
//     let movement = direction.normalize_or_zero() * camera_speed.0 * time.delta_seconds();
// }
