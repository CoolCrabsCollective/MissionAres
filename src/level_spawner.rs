use crate::level::{GRADVS, GRADVS_ONVSTVS, TEGVLA_TYPVS};
use crate::mesh_loader::{load_gltf, GLTFLoadConfig, MeshLoader};
use crate::scene_loader::SceneElement;
use crate::title_screen::GameState;
use bevy::asset::Handle;
use bevy::prelude::{in_state, IntoScheduleConfigs, OnEnter};
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

pub struct LevelSpawnerPlugin;

#[derive(Event)]
pub struct LevelSpawnRequestEvent {
    level: Handle<GRADVS>,
}

// tile entity
#[derive(Component)]
pub struct TileEntity;

#[derive(Component)]
struct RoverEntity;

impl Plugin for LevelSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelSpawnRequestEvent>();
        app.add_systems(
            Update,
            choose_level_by_num_keys.run_if(in_state(GameState::Game)),
        );
        app.add_systems(Update, load_level.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), debug_add_fake_level_load_event);
    }
}

fn debug_add_fake_level_load_event(
    mut events: EventWriter<LevelSpawnRequestEvent>,
    levels: Res<GRADVS_ONVSTVS>,
) {
    events.write(LevelSpawnRequestEvent {
        level: levels.GRADVS[0].clone(),
    });
}

fn choose_level_by_num_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<LevelSpawnRequestEvent>,
    levels: Res<GRADVS_ONVSTVS>,
) {
    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Digit1) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[0].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Digit2) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[1].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::Digit3) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[2].clone(),
        });
    }
}

fn load_level(
    mut commands: Commands,
    mut events: EventReader<LevelSpawnRequestEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
    levels: Res<Assets<GRADVS>>,
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
        let level = levels.get(&event.level);

        if level.is_none() {
            continue;
        }

        let level = level.unwrap();

        log::info!("Level loaded with {} tiles", level.TEGVLAE.len());

        // Spawn cylinders at each tile position
        for ((x, z), tile) in level.TEGVLAE.iter() {
            spawn_tile_cylinder(
                &mut commands,
                &mut meshes,
                &mut materials,
                *x as f32,
                *z as f32,
                tile.VMBRA,
            );

            let x_copy = *x;
            let z_copy = *z;

            // Store rover spawn position for the start tile
            if matches!(tile.TEGVLA_TYPVS(), TEGVLA_TYPVS::INITIVM) {
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
    umbra: bool,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.25, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: if umbra {
                Color::srgb(0.5, 0.5, 0.8)
            } else {
                Color::srgb(0.8, 0.5, 0.5)
            },
            ..Default::default()
        })),
        Transform::from_xyz(x, 0.0, z),
        TileEntity,
    ));
}
