use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub current_weapon: Option<Entity>,
}
