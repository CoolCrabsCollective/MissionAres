use crate::mesh_loader::{load_gltf, GLTFLoadConfig, MeshLoader};
use crate::scene_loader::SceneElement;
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::{EntityCommands, KeyCode, Res, ResMut, Transform};
use bevy::utils::default;

pub struct ObjectSpawnerPlugin;

impl Plugin for ObjectSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, object_spawner);
    }
}

fn object_spawner(
    input: Res<ButtonInput<KeyCode>>,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
) {
    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight) {
        return;
    }

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Digit1) {
        spawn_fly(asset_server, mesh_loader);
        return;
    } else if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Digit2) {
        spawn_shrimp(asset_server, mesh_loader);
        return;
    }
}

fn spawn_fly(mut asset_server: ResMut<AssetServer>, mut mesh_loader: ResMut<MeshLoader>) {
    load_gltf(
        String::from("fruit_fly.glb"),
        GLTFLoadConfig {
            entity_initializer: add_object_tag,
            ..default()
        },
        &mut asset_server,
        &mut mesh_loader,
    );
}

fn spawn_shrimp(mut asset_server: ResMut<AssetServer>, mut mesh_loader: ResMut<MeshLoader>) {
    load_gltf(
        String::from("pistol_shrimp.glb"),
        GLTFLoadConfig {
            entity_initializer: add_object_tag,
            ..default()
        },
        &mut asset_server,
        &mut mesh_loader,
    );
}

fn add_object_tag(commands: &mut EntityCommands) {
    commands.insert(SceneElement).insert(Transform::from_xyz(
        rand::random::<f32>() * 100.0 - 50.0,
        10.0,
        rand::random::<f32>() * 100.0 - 50.0,
    ));
    //.insert(RigidBodyBuilder::new(RigidBodyType::Dynamic).build())
    //.insert(ColliderBuilder::cuboid(1.0, 1.0, 1.0).build());
}
