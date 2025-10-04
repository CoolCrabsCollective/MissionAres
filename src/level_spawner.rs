use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query},
    },
    log,
    prelude::{
        AlphaMode, Assets, ButtonInput, Color, Component, Cylinder, EntityCommands, KeyCode, Mesh,
        Mesh3d, MeshMaterial3d, Plane3d, Res, ResMut, StandardMaterial, Transform, Vec2, Vec3,
    },
};

use crate::sane_level::{Level, TileExt, TileType, level_1, level_2};
use crate::scene_loader::SceneElement;
use crate::{
    mesh_loader::{GLTFLoadConfig, MeshLoader, load_gltf},
    sane_level::LevelExt,
};

pub struct LevelSpawnerPlugin;

#[derive(Event)]
pub struct LevelLoadedEvent {
    level: Level,
}

// tile entity
#[derive(Component)]
pub struct TileEntity;

#[derive(Component)]
struct RoverEntity;

impl Plugin for LevelSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelLoadedEvent>();
        app.add_systems(Update, choose_level_by_num_keys);
        app.add_systems(Update, load_level);
        app.add_systems(Startup, debug_add_fake_level_load_event);
    }
}

fn debug_add_fake_level_load_event(mut commands: Commands) {
    commands.send_event(LevelLoadedEvent { level: level_1() });
}

fn choose_level_by_num_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<LevelLoadedEvent>,
) {
    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Digit1) {
        events.write(LevelLoadedEvent { level: level_1() });
    }

    if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Digit2) {
        events.write(LevelLoadedEvent { level: level_2() });
    }
}

fn load_level(
    mut commands: Commands,
    mut events: EventReader<LevelLoadedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
    tiles: Query<Entity, With<TileEntity>>,
    rovers: Query<Entity, With<RoverEntity>>,
) {
    for event in events.read() {
        // remove all tiles and rovers
        for tile in tiles.iter() {
            commands.entity(tile).despawn();
        }
        for rover in rovers.iter() {
            commands.entity(rover).despawn();
        }

        log::info!("Level loaded with {} tiles", event.level.TEGVLAE().len());

        // Spawn cylinders at each tile position
        for ((x, z), tile) in event.level.tiles().iter() {
            spawn_tile_cylinder(
                &mut commands,
                &mut meshes,
                &mut materials,
                *x as f32,
                *z as f32,
            );

            let x_copy = *x;
            let z_copy = *z;

            // Store rover spawn position for the start tile
            if matches!(tile.tile_type(), TileType::Start) {
                load_gltf(
                    String::from("pistol_shrimp.glb"),
                    GLTFLoadConfig {
                        entity_initializer: Box::new(move |commands: &mut EntityCommands| {
                            commands
                                .insert(SceneElement)
                                .insert(
                                    // should spawn at the tile position
                                    Transform::from_xyz(x_copy as f32, 0.5, z_copy as f32)
                                        .with_scale(Vec3::splat(0.25)),
                                )
                                .insert(RoverEntity);
                        }),
                        ..Default::default()
                    },
                    &mut asset_server,
                    &mut mesh_loader,
                );
            }
        }

        commands.spawn((
            TileEntity,
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(15.0)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("cutoff_texture.png")),
                alpha_mode: AlphaMode::Mask(0.5),
                cull_mode: None,
                ..Default::default()
            })),
            Transform::from_xyz(5.0, 10.0, 0.0),
        ));
    }
}

fn spawn_tile_cylinder(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    x: f32,
    z: f32,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.25, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.8),
            ..Default::default()
        })),
        Transform::from_xyz(x, 0.0, z),
        TileEntity,
    ));
}
