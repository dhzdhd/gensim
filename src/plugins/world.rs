use std::f32::consts::PI;

use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_rapier3d::prelude::*;

use crate::bundles::{ColliderBundle, RigidBodyBundle};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light, spawn_skybox))
            .add_systems(Update, load_skybox);
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

const PLANE_SIZE: f32 = 100.0;

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(PLANE_SIZE))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb_u8(50, 2, 64),
            perceptual_roughness: 0.7,
            reflectance: 0.15,
            ..default()
        }),
        ..default()
    };

    commands
        .spawn((floor, RigidBodyBundle::new(RigidBody::Fixed)))
        .with_children(|builder| {
            builder.spawn(ColliderBundle::new(
                Vec3::new(PLANE_SIZE / 2.0, 0.0, PLANE_SIZE / 2.0),
                Vec3::splat(0.0),
            ));
        });
}

fn spawn_light(mut commands: Commands) {
    // let light = PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 140000.0,
    //         shadows_enabled: true,
    //         range: 100.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 30.0, 0.0),
    //     ..default()
    // };
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    };

    commands.spawn(light);
}

// https://tools.wwwtyro.net/space-3d/index.html
fn spawn_skybox(mut commands: Commands, assets: Res<AssetServer>) {
    let skybox_handle = assets.load("textures/sky.png");

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

fn load_skybox(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.get_load_state(&cubemap.image_handle) == LoadState::Loaded
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
