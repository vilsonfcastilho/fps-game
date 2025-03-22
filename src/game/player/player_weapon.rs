use bevy::prelude::*;

use super::input::*;

#[derive(Component)]
pub struct Weapon;

pub fn update_weapon(
    mut commands: Commands,
    mut input: ResMut<PlayerInput>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    let mut new_weapon_model: Option<Handle<Scene>> = None;
    let mut new_weapon_transform: Transform = Transform::default();

    // Glock
    if keys.just_pressed(KeyCode::Digit1) {
        new_weapon_model =
            Some(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/glock/scene.gltf")));
        new_weapon_transform = Transform {
            rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
            translation: Vec3::new(0.5, -0.5, -1.3),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..Default::default()
        };
    }

    // AK-47
    if keys.just_pressed(KeyCode::Digit2) {
        new_weapon_model =
            Some(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/ak_47/scene.gltf")));
        new_weapon_transform = Transform {
            rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
            translation: Vec3::new(0.5, -0.6, -1.3),
            scale: Vec3::new(0.004, 0.004, 0.004),
            ..Default::default()
        };
    }

    // M4-A1
    if keys.just_pressed(KeyCode::Digit3) {
        new_weapon_model =
            Some(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/m4_a1/scene.gltf")));
        new_weapon_transform = Transform {
            rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
            translation: Vec3::new(0.5, -0.6, -1.3),
            scale: Vec3::new(0.02, 0.02, 0.02),
            ..Default::default()
        };
    }

    if keys.just_pressed(KeyCode::Digit0) {
        new_weapon_model = Some(asset_server.load("models/basic/ak.glb#Scene0"));
        new_weapon_transform = Transform::IDENTITY;
    }

    if let Some(weapon_model) = new_weapon_model {
        if let Some(current_weapon) = input.current_weapon {
            commands.entity(current_weapon).despawn_recursive();
        }

        if let Ok(camera) = camera_query.get_single() {
            let new_weapon_entity: Entity = commands
                .spawn((
                    SceneBundle {
                        scene: weapon_model,
                        transform: new_weapon_transform,
                        ..Default::default()
                    },
                    Weapon,
                ))
                .id();

            input.current_weapon = Some(new_weapon_entity);
            commands.entity(camera).add_child(new_weapon_entity);
        }
    }
}
