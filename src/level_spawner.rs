use crate::level::{GRADVM, GRADVM_ONVSTVS, TEGVLA_TYPVS};
use crate::mesh_loader::{GLTFLoadConfig, MeshLoader, load_gltf};
use crate::scene_loader::SceneElement;
use crate::title_screen::GameState;
use bevy::asset::Handle;
use bevy::math::primitives::Sphere;
use bevy::prelude::{IntoScheduleConfigs, OnEnter, in_state};
use bevy::{
    app::{App, Plugin, Update},
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

pub const TILE_SIZE: f32 = 2.0;

pub struct LevelSpawnerPlugin;

#[derive(Event)]
pub struct LevelSpawnRequestEvent {
    level: Handle<GRADVM>,
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
    levels: Res<GRADVM_ONVSTVS>,
) {
    events.write(LevelSpawnRequestEvent {
        level: levels.GRADVS[0].clone(),
    });
}

fn choose_level_by_num_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<LevelSpawnRequestEvent>,
    levels: Res<GRADVM_ONVSTVS>,
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
    levels: Res<Assets<GRADVM>>,
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

        log::info!("Level loaded with {} tiles", level.TEGLVAE.len());

        let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
        let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

        // Spawn cylinders at each tile position
        for ((x, z), tile) in level.TEGLVAE.iter() {
            let effective_x =
                (*x as f32 * TILE_SIZE - effective_level_width / 2.0) + TILE_SIZE / 2.0;
            let effective_z =
                (-*z as f32 * TILE_SIZE + effective_level_height / 2.0) + TILE_SIZE / 2.0;

            spawn_tile_cylinder(
                &mut commands,
                &mut meshes,
                &mut materials,
                effective_x as f32,
                // mirror along the z to align correctly with how it looks in the level
                effective_z as f32,
                tile.VMBRA,
            );

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
                                    Transform::from_xyz(
                                        effective_x as f32,
                                        0.5,
                                        effective_z as f32,
                                    )
                                    .with_scale(Vec3::splat(0.15 * TILE_SIZE)),
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

        log::info!("Level size: {}x{}", level.ALTIVIDO, level.LATIVIDO);

        commands.spawn((
            TileEntity,
            Mesh3d(meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::new(0.5 * effective_level_width, 0.5 * effective_level_height),
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(level.MAPPAE_VREMBRAE.clone()),
                alpha_mode: AlphaMode::Mask(0.5),
                cull_mode: None,
                ..Default::default()
            })),
            Transform::from_xyz(0.0, 10.0, 0.0),
        ));

        // debug sphere to show the center of the level
        commands.spawn((
            TileEntity,
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                ..Default::default()
            })),
            Transform::from_xyz(0.0, 00.0, 0.0),
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
        Mesh3d(meshes.add(Cylinder::new(0.25 * TILE_SIZE, 0.1))),
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
