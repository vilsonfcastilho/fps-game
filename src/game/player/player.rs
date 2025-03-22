use core::f32;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    camera_controller,
    input::PlayerInput,
    player_movement,
    player_shooting::{self, TracerSpawnSpot},
    player_weapon::{self, Weapon},
};

use crate::game::{math::coordinates::blender_to_world, shooting::tracer};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tracer::TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (
                    camera_controller::update_camera_controller,
                    player_shooting::update_player,
                    player_movement::update_movement_input,
                    player_weapon::update_weapon,
                ),
            )
            .add_systems(FixedUpdate, player_movement::update_movement); // physics timestamp
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}

fn init_player(
    mut commands: Commands,
    mut input: ResMut<PlayerInput>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    let camera_entity: Entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 10.),
                    ..Default::default()
                },
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            camera_controller::CameraController {
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
                sensitivity: 0.035,
            },
        ))
        .id();

    // Camera - Weapon
    let weapon_model: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/glock/scene.gltf")); // -> Glock model
    let weapon_entity: Entity = commands
        .spawn((
            SceneBundle {
                scene: weapon_model,
                transform: Transform {
                    rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
                    translation: Vec3::new(0.5, -0.5, -1.3),
                    scale: Vec3::new(0.1, 0.1, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            },
            Weapon,
        ))
        .id();

    // Camera - Tracer Spawn Spot
    let spawn_spot: Vec3 = blender_to_world(Vec3::new(0.530154, 1.409, -0.313846)); // -> m4a1 tracer spot
    let tracer_spawn_entity: Entity = commands
        .spawn((
            TransformBundle {
                local: Transform::from_translation(spawn_spot),
                ..Default::default()
            },
            TracerSpawnSpot,
        ))
        .id();

    // Player
    let player_entity: Entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.,
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 30., 0.)),
                ..Default::default()
            },
            Collider::cuboid(1., 10., 1.),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..Default::default()
            },
        ))
        .id();

    input.current_weapon = Some(weapon_entity);

    commands
        .entity(camera_entity)
        .push_children(&[weapon_entity, tracer_spawn_entity]);
    commands.entity(player_entity).add_child(camera_entity);
}
