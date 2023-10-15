use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
    collider: Collider,
    transform: TransformBundle,
    collision_types: ActiveCollisionTypes,
}

impl ColliderBundle {
    pub fn new(dim: Vec3, transform: Vec3) -> Self {
        Self {
            collider: Collider::cuboid(dim.x, dim.y, dim.z),
            transform: TransformBundle::from(Transform::from_xyz(
                transform.x,
                transform.y,
                transform.z,
            )),
            collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC,
        }
    }
}

#[derive(Bundle)]
pub struct RigidBodyBundle {
    body: RigidBody,
    gravity: GravityScale,
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::Dynamic,
            gravity: GravityScale(0.0),
        }
    }
}

impl RigidBodyBundle {
    pub fn new(body_type: RigidBody) -> Self {
        Self {
            body: body_type,
            gravity: GravityScale(1.0),
        }
    }
}
