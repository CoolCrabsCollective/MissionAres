use crate::game_control::actions::{Action, ActionList, ActionType};
use crate::level::{GRADVM, GRADVM_ONVSTVS, TEGVLA_TYPVS};
use crate::mesh_loader::{load_gltf, GLTFLoadConfig, MeshLoader};
use crate::puzzle_evaluation::PuzzleResponseEvent;
use crate::rover::{RoverEntity, RoverPlugin};
use crate::title_screen::GameState;
use bevy::app::Startup;
use bevy::asset::{Handle, RenderAssetUsages};
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasing};
use bevy::core_pipeline::Skybox;
use bevy::image::{CompressedImageFormats, Image};
use bevy::math::primitives::Sphere;
use bevy::math::{I8Vec2, Quat};
use bevy::pbr::{
    AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightShadowMap,
    DistanceFog, FogFalloff, ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel,
};
use bevy::prelude::{
    default, in_state, Camera, Camera3d, ClearColor, ClearColorConfig, ColorMaterial,
    DetectChanges, Gltf, IntoScheduleConfigs, Msaa, OnEnter, PerspectiveProjection, Projection,
    Reflect, Resource,
};
use bevy::render::camera::TemporalJitter;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
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
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};
use rand::random;
use std::cmp::{max, min};
use std::f32::consts::PI;

pub const CUBEMAPS: &[(&str, CompressedImageFormats)] =
    &[("test_skybox.png", CompressedImageFormats::NONE)];

#[derive(Resource)]
pub struct Cubemap {
    pub(crate) is_loaded: bool,
    pub(crate) image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct LevelElement;

pub const TILE_SIZE: f32 = 2.0;
pub const LEVEL_SHADOW_ALPHA_MASK: f32 = 0.5;
pub const ROCK_PADDING: i32 = 5;

pub struct LevelSpawnerPlugin;

#[derive(Event)]
pub struct LevelSpawnRequestEvent {
    level: Handle<GRADVM>,
}

#[derive(Event)]
pub struct AfterLevelSpawnEvent;

// tile entity
#[derive(Component)]
pub struct TileEntity;

#[derive(Resource)]
pub struct ActiveLevel(pub Option<Handle<GRADVM>>);

impl Plugin for LevelSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelSpawnRequestEvent>();
        app.add_event::<AfterLevelSpawnEvent>();
        app.add_systems(
            Update,
            choose_level_by_num_keys.run_if(in_state(GameState::Game)),
        );
        app.add_systems(Update, load_level.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), debug_add_fake_level_load_event);
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, handle_puzzle_solved_event);
        app.add_systems(Update, handle_puzzle_failed_event);

        app.add_systems(Update, asset_loaded);
        app.insert_resource(ActiveLevel(None));
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
        ));

        app.add_plugins(RoverPlugin);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(TemporalAntiAliasPlugin);

        app.add_systems(Update, debug_render_toggle)
            .insert_resource(ClearColor(Color::srgb(0.3, 0.6, 0.9)))
            .insert_resource(DirectionalLightShadowMap { size: 4096 });
    }
}

fn setup_scene(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("Space Program.ogg")),
        PlaybackSettings::LOOP,
    ));

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle.clone(),
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 250.0,
        affects_lightmapped_meshes: true,
    });
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 2000.0,
            shadows_enabled: true,
            affects_lightmapped_mesh_diffuse: true,
            ..Default::default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -std::f32::consts::PI / 2.0)),
        CascadeShadowConfigBuilder {
            maximum_distance: 100.0,
            ..default()
        }
        .build(),
    ));
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    let mut camera_bundle = commands.spawn((
        Camera3d::default(),
        Camera {
            // renders after / on top of the main camera
            order: 1,
            hdr: true,
            // don't clear the color while rendering this camera
            clear_color: ClearColorConfig::Default,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 55.0f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(-0.5, 5.0, 10.5).with_rotation(Quat::from_axis_angle(Vec3::Y, 0.0)),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            rotation: Default::default(),
        },
        DistanceFog {
            color: Color::srgb(0.8, 0.35, 0.2),
            falloff: FogFalloff::Linear {
                start: 500.0,
                end: 600.0,
            },
            ..default()
        },
        Msaa::Off,
        ScreenSpaceAmbientOcclusion {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
            ..default()
        },
        Bloom::default(),
    ));

    #[cfg(not(target_arch = "wasm32"))]
    camera_bundle.insert((TemporalAntiAliasing::default(), TemporalJitter::default()));
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

    if input.just_pressed(KeyCode::Numpad4) || input.just_pressed(KeyCode::Digit4) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[3].clone(),
        });
    }
}

fn load_level(
    mut commands: Commands,
    mut events: EventReader<LevelSpawnRequestEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
    mut active_level: ResMut<ActiveLevel>,
    mut action_list: ResMut<ActionList>,
    levels: Res<Assets<GRADVM>>,
    level_elements: Query<Entity, With<LevelElement>>,
) {
    for event in events.read() {
        // remove all tiles and rovers
        for level_element in level_elements.iter() {
            commands.entity(level_element).despawn();
        }

        let level = levels.get(&event.level);

        if level.is_none() {
            continue;
        }

        let level = level.unwrap();

        log::info!("Level loaded with {} tiles", level.TEGLVAE.len());

        let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
        let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

        commands.spawn((
            LevelElement,
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.0)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.35, 0.2), // Mars-colored (reddish-orange)
                perceptual_roughness: 0.9,
                metallic: 0.0,
                ..Default::default()
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
        let mut num_rovers = 0;
        // Spawn cylinders at each tile position
        for ((x, z), tile) in level.TEGLVAE.iter() {
            let logical_x = *x as i32;
            let logical_z = *z as i32;
            let effective_x =
                (*x as f32 * TILE_SIZE - effective_level_width / 2.0) + TILE_SIZE / 2.0;
            // mirror along the z to align correctly with how it looks in the level
            let effective_z =
                (-*z as f32 * TILE_SIZE + effective_level_height / 2.0) + TILE_SIZE / 2.0;

            spawn_tile_cylinder(
                &mut commands,
                &mut meshes,
                &mut materials,
                effective_x,
                effective_z,
                tile.VMBRA,
            );

            // Store rover spawn position for the start tile

            if matches!(tile.TEGVLA_TYPVS(), TEGVLA_TYPVS::INITIVM) {
                num_rovers += 1;
                load_gltf(
                    String::from("rover.glb"),
                    GLTFLoadConfig {
                        entity_initializer: Box::new(move |commands: &mut EntityCommands| {
                            commands
                                .insert(
                                    // should spawn at the tile position
                                    Transform::from_xyz(effective_x, 0.0, effective_z)
                                        .with_scale(Vec3::splat(0.15 * TILE_SIZE))
                                        .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
                                )
                                .insert(RoverEntity {
                                    is_setup: false,
                                    base_color: Color::srgb(0.5, 0.2, 0.8),
                                    gltf_handle: Default::default(),
                                    logical_position: I8Vec2::new(
                                        logical_x.try_into().unwrap(),
                                        logical_z.try_into().unwrap(),
                                    ),
                                    battery_level: 3,
                                    identifier: num_rovers - 1,
                                })
                                .insert(LevelElement);
                        }),
                        ..Default::default()
                    },
                    &asset_server,
                    &mut mesh_loader,
                );
            }

            if matches!(tile.TEGVLA_TYPVS(), TEGVLA_TYPVS::FINIS) {
                load_gltf(
                    String::from("mineral.glb"),
                    GLTFLoadConfig {
                        entity_initializer: Box::new(move |commands: &mut EntityCommands| {
                            commands
                                .insert(
                                    // should spawn at the tile position
                                    Transform::from_xyz(effective_x, 0.0, effective_z)
                                        .with_scale(Vec3::splat(0.05 * TILE_SIZE))
                                        .with_rotation(Quat::from_rotation_y(
                                            random::<f32>() * PI * 2.0,
                                        )),
                                )
                                .insert(LevelElement);
                        }),
                        ..Default::default()
                    },
                    &asset_server,
                    &mut mesh_loader,
                );
            }

            if matches!(tile.TEGVLA_TYPVS(), TEGVLA_TYPVS::SATVRNALIA) {
                load_gltf(
                    String::from("dish.glb"),
                    GLTFLoadConfig {
                        entity_initializer: Box::new(move |commands: &mut EntityCommands| {
                            commands
                                .insert(
                                    // should spawn at the tile position
                                    Transform::from_xyz(effective_x, 0.0, effective_z)
                                        .with_scale(Vec3::splat(0.5 * TILE_SIZE)),
                                )
                                .insert(LevelElement);
                        }),
                        ..Default::default()
                    },
                    &asset_server,
                    &mut mesh_loader,
                );
            }

            if matches!(tile.TEGVLA_TYPVS(), TEGVLA_TYPVS::CRATER) {
                load_gltf(
                    String::from("crater.glb"),
                    GLTFLoadConfig {
                        entity_initializer: Box::new(move |commands: &mut EntityCommands| {
                            commands
                                .insert(
                                    // should spawn at the tile position
                                    Transform::from_xyz(effective_x, 0.0, -effective_z)
                                        .with_scale(Vec3::splat(0.5 * TILE_SIZE)),
                                )
                                .insert(LevelElement);
                        }),
                        ..Default::default()
                    },
                    &asset_server,
                    &mut mesh_loader,
                );
            }
        }

        log::info!("Level size: {}x{}", level.ALTIVIDO, level.LATIVIDO);

        // Spawn boundary rocks
        for x in -ROCK_PADDING..level.LATIVIDO as i32 + ROCK_PADDING {
            for y in -ROCK_PADDING..level.ALTIVIDO as i32 + ROCK_PADDING {
                let key = (x as i8, level.ALTIVIDO - y as i8);
                if x >= 0 && x < level.LATIVIDO as i32 && y >= 0 && y < level.ALTIVIDO as i32
                //&& level.TEGLVAE.contains_key(&key)
                {
                    continue;
                }
                let distance_x = if x < 0 {
                    -x
                } else {
                    x - level.LATIVIDO as i32 + 1
                };
                let distance_y = if y < 0 {
                    -y
                } else {
                    y - level.ALTIVIDO as i32 + 1
                };
                let distance = max(0, max(distance_x, distance_y));

                if distance == 1 {
                    continue;
                }

                let effective_x =
                    (x as f32 * TILE_SIZE - effective_level_width / 2.0) + TILE_SIZE / 2.0;
                let effective_z =
                    (y as f32 * TILE_SIZE - effective_level_height / 2.0) + TILE_SIZE / 2.0;

                spawn_rock(
                    effective_x,
                    effective_z,
                    distance,
                    &asset_server,
                    &mut mesh_loader,
                );
            }
        }

        let plane_mesh_handle = meshes.add(create_mappae_umbrae_mesh(Vec2::new(
            effective_level_width,
            effective_level_height,
        )));
        commands.spawn((
            LevelElement,
            TileEntity,
            Mesh3d::from(plane_mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(level.MAPPAE_VREMBRAE.clone()),
                alpha_mode: AlphaMode::Mask(LEVEL_SHADOW_ALPHA_MASK),
                cull_mode: None,
                unlit: true,
                ..Default::default()
            })),
            Transform::from_xyz(0.0, 30.0, 0.0),
        ));

        // debug sphere to show the center of the level
        commands.spawn((
            LevelElement,
            TileEntity,
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                ..Default::default()
            })),
            Transform::from_xyz(random::<f32>(), 0.0, random::<f32>()),
        ));

        active_level.0 = Some(event.level.clone());

        action_list.actions.clear();
        for i in 0..num_rovers {
            action_list.actions.push(vec![]);
        }
        let action_event = action_list.clone();
        println!("Sending event with {} rovers", action_list.actions.len());
        commands.send_event(action_event);

        commands.send_event(AfterLevelSpawnEvent);
    }
}

fn create_mappae_umbrae_mesh(size: Vec2) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.5 * size.x, 0.0, -0.5 * size.y],
            [0.5 * size.x, 0.0, -0.5 * size.y],
            [0.5 * size.x, 0.0, 0.5 * size.y],
            [-0.5 * size.x, 0.0, 0.5 * size.y],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]))
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
        LevelElement,
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

fn spawn_rock(
    x: f32,
    z: f32,
    distance: i32,
    asset_server: &Res<AssetServer>,
    mesh_loader: &mut ResMut<MeshLoader>,
) {
    let base_size: f32 = distance as f32 * 0.15;
    load_gltf(
        String::from("rock.glb"),
        GLTFLoadConfig {
            entity_initializer: Box::new(move |commands: &mut EntityCommands| {
                commands
                    .insert(
                        // should spawn at the tile position
                        Transform::from_xyz(
                            x + random::<f32>() * 0.6 - 0.3,
                            0.0,
                            z + random::<f32>() * 0.6 - 0.3,
                        )
                        .with_scale(Vec3::splat(
                            ((1.0 + random::<f32>()) * base_size) * TILE_SIZE,
                        ))
                        .with_rotation(Quat::from_rotation_y(random::<f32>() * PI * 2.0)),
                    )
                    .insert(LevelElement);
            }),
            ..default()
        },
        &asset_server,
        mesh_loader,
    );
}

fn debug_render_toggle(mut context: ResMut<DebugRenderContext>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_released(KeyCode::F12) {
        context.enabled = !context.enabled;
    }
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

fn handle_puzzle_solved_event(
    mut events: EventReader<PuzzleResponseEvent>,
    mut level_spawn_request_writer: EventWriter<LevelSpawnRequestEvent>,
    levels: Res<Assets<GRADVM>>,
    level_handles: Res<GRADVM_ONVSTVS>,
    active_level: Res<ActiveLevel>,
) {
    for event in events.read() {
        if *event == PuzzleResponseEvent::Solved {
            log::info!("Puzzle solved event received.");

            let Some(active_level_handle) = &active_level.0 else {
                log::error!("No active level.");
                return;
            };

            let Some(active_level) = levels.get(active_level_handle) else {
                log::error!("No active level.");
                return;
            };

            log::info!("Active level index: {}", active_level.INDEX);
            log::info!("Next level index: {}", active_level.INDEX + 1);
            log::info!("Level handles: {:?}", level_handles.GRADVS.len());

            let Some(next_level_handle) = level_handles
                .GRADVS
                .get(active_level.INDEX as usize + 1)
                .or(level_handles.GRADVS.get(0))
            else {
                log::error!("No next level.");
                return;
            };

            level_spawn_request_writer.write(LevelSpawnRequestEvent {
                level: next_level_handle.clone(),
            });
        }
    }
}

fn handle_puzzle_failed_event(
    mut events: EventReader<PuzzleResponseEvent>,
    mut level_spawn_request_writer: EventWriter<LevelSpawnRequestEvent>,
    level_handles: Res<GRADVM_ONVSTVS>,
) {
    for event in events.read() {
        if *event == PuzzleResponseEvent::Failed {
            log::info!("Puzzle failed event received.");

            let Some(next_level_handle) = level_handles.GRADVS.get(0) else {
                log::error!("No next level.");
                return;
            };

            level_spawn_request_writer.write(LevelSpawnRequestEvent {
                level: next_level_handle.clone(),
            });
        }
    }
}
