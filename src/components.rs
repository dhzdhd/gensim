use bevy::prelude::*;

#[derive(Component)]
pub struct Health(f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.0)
    }
}
