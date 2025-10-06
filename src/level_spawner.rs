use crate::game_control::actions::ActionList;
use crate::hentai_anime::*;
use crate::level::{GRADVM, GRADVM_ONVSTVS, TEGVLA_TYPVS};
use crate::mesh_loader::{GLTFLoadConfig, MeshLoader, load_gltf};
use crate::particle::dust::DustSpawner;
use crate::particle::particle::Particle;
use crate::puzzle_evaluation::PuzzleResponseEvent;
use crate::rover::{RoverCollectable, RoverEntity, RoverPlugin, RoverStates};
use crate::title_screen::GameState;
use crate::ui::control_ui::{RoverColors, on_rover_click};
use crate::ui::win_screen::NextLevelRequestEvent;
use bevy::app::Startup;
use bevy::asset::{Handle, RenderAssetUsages};
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings, Volume};
use bevy::color::palettes::css::BLUE;
use bevy::core_pipeline::Skybox;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasing};
use bevy::image::{CompressedImageFormats, Image};
use bevy::math::ops::abs;
use bevy::math::{I8Vec2, Quat};
use bevy::pbr::{
    AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightShadowMap,
    PointLight, ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel,
};
use bevy::prelude::*;
use bevy::render::camera::TemporalJitter;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy::time::{Timer, TimerMode};
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
};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};
use rand::random;
use std::cmp::max;
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
            choose_level_by_num_keys.run_if(not(in_state(GameState::TitleScreen))),
        );
        app.add_systems(
            Update,
            load_level.run_if(not(in_state(GameState::TitleScreen))),
        );
        app.add_systems(OnExit(GameState::TitleScreen), spawn_initial_level);
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, handle_puzzle_solved_event);
        app.add_systems(Update, handle_next_level_request);
        app.add_systems(Update, (handle_puzzle_failed_event, update_reset_timer));

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

fn setup_scene(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("Space Program.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.3),
            ..default()
        },
    ));

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle.clone(),
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 700.0,
        affects_lightmapped_meshes: true,
    });
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 1000.0,
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
        Transform::default(),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 2000.0,
            rotation: Default::default(),
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

fn spawn_initial_level(
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

    if input.just_pressed(KeyCode::Numpad5) || input.just_pressed(KeyCode::Digit5) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[4].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad6) || input.just_pressed(KeyCode::Digit6) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[5].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::Digit7) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[6].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad8) || input.just_pressed(KeyCode::Digit8) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[7].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::Digit9) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[8].clone(),
        });
    }

    if input.just_pressed(KeyCode::Numpad0) || input.just_pressed(KeyCode::Digit0) {
        events.write(LevelSpawnRequestEvent {
            level: levels.GRADVS[9].clone(),
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
    mut rover_colors: ResMut<RoverColors>,
    levels: Res<Assets<GRADVM>>,
    level_elements: Query<Entity, With<LevelElement>>,
    mut camera_transform: Query<(&Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    particles: Query<Entity, (With<Particle>, Without<LevelElement>)>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if events.is_empty() {
        return;
    }

    for particle in particles.iter() {
        commands.entity(particle).despawn();
    }
    // remove all tiles and rovers
    for level_element in level_elements.iter() {
        commands.entity(level_element).despawn();
    }
    let event = events.read().last().unwrap();
    let level = levels.get(&event.level);

    if level.is_none() {
        return;
    }

    let level = level.unwrap();

    let level_width = level.LATIVIDO as f32 * TILE_SIZE;
    let level_height = level.ALTIVIDO as f32 * TILE_SIZE;

    for (cam, mut trans, g_trans) in camera_transform.iter_mut() {
        trans.translation = Vec3::new(0.0, 8.0, 5.0);
        trans.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

        let mut g_transform = *g_trans;

        loop {
            let mut any_out = false;
            let out = Vec3::new(-2.0, -2.0, -2.0);

            let ndc1 = cam
                .world_to_ndc(
                    &g_transform,
                    Vec3::new(-level_width / 2.0, 0.0, -level_height / 2.0),
                )
                .unwrap_or(out);

            let ndc2 = cam
                .world_to_ndc(
                    &g_transform,
                    Vec3::new(level_width / 2.0, 0.0, -level_height / 2.0),
                )
                .unwrap_or(out);

            let ndc3 = cam
                .world_to_ndc(
                    &g_transform,
                    Vec3::new(-level_width / 2.0, 0.0, level_height / 2.0),
                )
                .unwrap_or(out);

            let ndc4 = cam
                .world_to_ndc(
                    &g_transform,
                    Vec3::new(level_width / 2.0, 0.0, level_height / 2.0),
                )
                .unwrap_or(out);

            any_out |= abs(ndc1.x) > 0.6 || abs(ndc1.y) > 1.0;
            any_out |= abs(ndc2.x) > 0.6 || abs(ndc2.y) > 1.0;
            any_out |= abs(ndc3.x) > 0.6 || abs(ndc3.y) > 1.0;
            any_out |= abs(ndc4.x) > 0.6 || abs(ndc4.y) > 1.0;

            if !any_out {
                break;
            }

            trans.translation.y += 2.0;
            trans.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
            g_transform = GlobalTransform::default();
            g_transform = g_transform.mul_transform(*trans);
        }
        trans.translation.x -= trans.translation.y / 4.0;
    }

    let mars_texture = asset_server.load("mars.png");
    commands.spawn((
        LevelElement,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            //base_color: Color::srgb(0.8, 0.35, 0.2), // Mars-colored (reddish-orange)
            base_color_texture: Some(mars_texture),
            perceptual_roughness: 0.9,
            metallic: 0.5,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let mut num_rovers = 0;
    // Spawn cylinders at each tile position
    for ((x, z), tile) in level.TEGLVAE.iter() {
        let logical_x = *x as i32;
        let logical_z = *z as i32;
        let effective_x = (*x as f32 * TILE_SIZE - level_width / 2.0) + TILE_SIZE / 2.0;
        // mirror along the z to align correctly with how it looks in the level
        let effective_z = (-*z as f32 * TILE_SIZE + level_height / 2.0) + TILE_SIZE / 2.0;

        let tile_pos = (*x, *z);

        match tile.TYPVS {
            TEGVLA_TYPVS::SATVRNALIA => {}
            TEGVLA_TYPVS::CRATERA => {}
            TEGVLA_TYPVS::INGENII => {}
            _ => {
                spawn_tile(
                    &mut commands,
                    &asset_server,
                    &mut mesh_loader,
                    effective_x,
                    effective_z,
                    tile.VMBRA,
                    level.NEXVS.contains_key(&tile_pos),
                );
            }
        }

        // Store rover spawn position for the start tile

        if matches!(tile.TYPVS, TEGVLA_TYPVS::INITIVM) {
            num_rovers += 1;
            let rover_index = num_rovers - 1;
            let rover_colors_cloned = rover_colors.0.clone();
            load_gltf(
                String::from("rover.glb"),
                GLTFLoadConfig {
                    entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                        commands
                            .insert(
                                // should spawn at the tile position
                                Transform::from_xyz(effective_x, 0.09, effective_z)
                                    .with_scale(Vec3::splat(0.15 * TILE_SIZE))
                                    .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
                            )
                            .insert(RoverEntity {
                                is_acting: false,
                                is_turn_done: false,
                                base_color: Color::srgb(0.5, 0.2, 0.8),
                                gltf_handle: Default::default(),
                                logical_position: I8Vec2::new(
                                    logical_x.try_into().unwrap(),
                                    logical_z.try_into().unwrap(),
                                ),
                                battery_level: 3,
                                identifier: rover_index,
                                heading: -PI / 2.0,
                                rover_state: RoverStates::Standby,
                                collided: false,
                                spawned_fail_particle: false,
                                spawned_wait_particle: false,
                                is_done: false,
                            })
                            .insert(LevelElement)
                            .insert(DustSpawner {
                                timer: Timer::from_seconds(0.4, TimerMode::Repeating),
                            })
                            .insert(Pickable::default())
                            .observe(play_all_animations_when_ready)
                            .observe(on_rover_click);
                    })),
                    scene_color_override: Some(
                        rover_colors_cloned
                            .get(rover_index as usize)
                            .cloned()
                            .or_else(|| rover_colors_cloned.get(0).cloned())
                            // pink
                            .unwrap_or(Color::srgb(1.0, 0.0, 1.0)),
                    ),
                    ..Default::default()
                },
                &asset_server,
                &mut mesh_loader,
            );
        }

        if matches!(tile.TYPVS, TEGVLA_TYPVS::FINIS) {
            load_gltf(
                String::from("mineral.glb"),
                GLTFLoadConfig {
                    entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                        commands
                            .insert(
                                // should spawn at the tile position
                                Transform::from_xyz(effective_x, 0.0, effective_z)
                                    .with_scale(Vec3::splat(0.05 * TILE_SIZE))
                                    .with_rotation(Quat::from_rotation_y(
                                        random::<f32>() * PI * 2.0,
                                    )),
                            )
                            .insert(LevelElement)
                            .insert(RoverCollectable);
                    })),
                    ..Default::default()
                },
                &asset_server,
                &mut mesh_loader,
            );

            commands.spawn((
                Transform::from_xyz(effective_x, 0.0, effective_z)
                    .with_scale(Vec3::splat(0.05 * TILE_SIZE))
                    .with_rotation(Quat::from_rotation_y(random::<f32>() * PI * 2.0)),
                LevelElement,
                PointLight {
                    intensity: 1_000_000.0,
                    color: BLUE.into(),
                    shadows_enabled: false,
                    ..default()
                },
            ));
        }

        if matches!(tile.TYPVS, TEGVLA_TYPVS::SATVRNALIA) {
            load_gltf(
                String::from("dish.glb"),
                GLTFLoadConfig {
                    entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                        commands
                            .insert(
                                // should spawn at the tile position
                                Transform::from_xyz(effective_x, 0.0, effective_z)
                                    .with_scale(Vec3::splat(0.25 * TILE_SIZE)),
                            )
                            .insert(LevelElement)
                            .observe(play_all_animations_when_ready);
                    })),
                    ..Default::default()
                },
                &asset_server,
                &mut mesh_loader,
            );
        }

        if matches!(tile.TYPVS, TEGVLA_TYPVS::INGENII) {
            load_gltf(
                String::from("ingenuity.glb"),
                GLTFLoadConfig {
                    entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                        commands
                            .insert(
                                // should spawn at the tile position
                                Transform::from_xyz(effective_x, 1.0 * TILE_SIZE, effective_z)
                                    .with_scale(Vec3::splat(0.2 * TILE_SIZE))
                                    .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
                            )
                            .insert(LevelElement)
                            .observe(play_all_animations_when_ready);
                    })),
                    ..Default::default()
                },
                &asset_server,
                &mut mesh_loader,
            );
        }

        if matches!(tile.TYPVS, TEGVLA_TYPVS::CRATERA) {
            load_gltf(
                String::from("crater.glb"),
                GLTFLoadConfig {
                    entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                        commands
                            .insert(
                                // should spawn at the tile position
                                Transform::from_xyz(effective_x, 0.0, effective_z)
                                    .with_scale(Vec3::splat(0.25 * TILE_SIZE)),
                            )
                            .insert(LevelElement);
                    })),
                    ..Default::default()
                },
                &asset_server,
                &mut mesh_loader,
            );
        }
    }

    for (start, end) in level.NEXVS.iter() {
        if start > end {
            continue;
        }

        let start = Vec2::new(
            (start.0 as f32 * TILE_SIZE - level_width / 2.0) + TILE_SIZE / 2.0,
            (-start.1 as f32 * TILE_SIZE + level_height / 2.0) + TILE_SIZE / 2.0,
        );
        let end = Vec2::new(
            (end.0 as f32 * TILE_SIZE - level_width / 2.0) + TILE_SIZE / 2.0,
            (-end.1 as f32 * TILE_SIZE + level_height / 2.0) + TILE_SIZE / 2.0,
        );
        spawn_wire(&mut commands, &mut meshes, &mut materials, start, end);
    }

    let rock_padding_x = max(ROCK_PADDING, (level.LATIVIDO / 4) as i32);
    let rock_padding_y = max(ROCK_PADDING, (level.ALTIVIDO / 4) as i32);

    // Spawn boundary rocks
    for x in -rock_padding_x..level.LATIVIDO as i32 + rock_padding_x {
        for y in -rock_padding_y..level.ALTIVIDO as i32 + rock_padding_y {
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

            let effective_x = (x as f32 * TILE_SIZE - level_width / 2.0) + TILE_SIZE / 2.0;
            let effective_z = (y as f32 * TILE_SIZE - level_height / 2.0) + TILE_SIZE / 2.0;

            spawn_rock(
                effective_x,
                effective_z,
                distance,
                &asset_server,
                &mut mesh_loader,
            );
        }
    }

    spawn_umbra(
        &mut commands,
        &mut meshes,
        &mut materials,
        level_width,
        level_height,
        level.MAPPAE_VREMBRAE.clone(),
    );

    active_level.0 = Some(event.level.clone());

    action_list.actions.clear();
    for i in 0..num_rovers {
        action_list.actions.push(vec![]);
    }
    let action_event = action_list.clone();
    commands.send_event(action_event);

    next_state.set(GameState::Programming);
    commands.send_event(AfterLevelSpawnEvent);
}

fn spawn_umbra(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    level_width: f32,
    level_height: f32,
    image: Handle<Image>,
) {
    let plane_mesh_handle = meshes.add(create_mappae_umbrae_mesh(Vec2::new(
        level_width,
        level_height,
    )));
    commands.spawn((
        LevelElement,
        TileEntity,
        Mesh3d::from(plane_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image),
            alpha_mode: AlphaMode::Mask(LEVEL_SHADOW_ALPHA_MASK),
            cull_mode: None,
            unlit: true,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 30.0, 0.0),
    ));
}

fn create_mappae_umbrae_mesh(size: Vec2) -> Mesh {
    let uv_padding = 20.0;
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [
                -0.5 * size.x * (uv_padding * 2.0 + 1.0),
                0.0,
                -0.5 * size.y * (uv_padding * 2.0 + 1.0),
            ],
            [
                0.5 * size.x * (uv_padding * 2.0 + 1.0),
                0.0,
                -0.5 * size.y * (uv_padding * 2.0 + 1.0),
            ],
            [
                0.5 * size.x * (uv_padding * 2.0 + 1.0),
                0.0,
                0.5 * size.y * (uv_padding * 2.0 + 1.0),
            ],
            [
                -0.5 * size.x * (uv_padding * 2.0 + 1.0),
                0.0,
                0.5 * size.y * (uv_padding * 2.0 + 1.0),
            ],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [-uv_padding, -uv_padding],
            [1.0 + uv_padding, -uv_padding],
            [1.0 + uv_padding, 1.0 + uv_padding],
            [-uv_padding, 1.0 + uv_padding],
        ],
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

fn spawn_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mesh_loader: &mut ResMut<MeshLoader>,
    x: f32,
    z: f32,
    umbra: bool,
    connected: bool,
) {
    load_gltf(
        String::from("path.glb"),
        GLTFLoadConfig {
            entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                commands
                    .insert(
                        // should spawn at the tile position
                        Transform::from_xyz(x, 0.0, z)
                            .with_scale(Vec3::splat(0.4 * TILE_SIZE))
                            .with_rotation(Quat::from_rotation_y(random::<i8>() as f32 * PI / 2.0)),
                    )
                    .insert(LevelElement)
                    .insert(TileEntity);
            })),
            ..default()
        },
        &asset_server,
        mesh_loader,
    );

    if connected {
        load_gltf(
            String::from("plate.glb"),
            GLTFLoadConfig {
                entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
                    commands
                        .insert(
                            // should spawn at the tile position
                            Transform::from_xyz(x, 0.05, z)
                                .with_scale(Vec3::splat(0.2 * TILE_SIZE))
                                .with_rotation(Quat::from_rotation_y(
                                    random::<i8>() as f32 * PI / 2.0,
                                )),
                        )
                        .insert(LevelElement)
                        .insert(TileEntity);
                })),
                ..default()
            },
            &asset_server,
            mesh_loader,
        );
    }
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
            entity_initializer: Some(Box::new(move |commands: &mut EntityCommands| {
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
            })),
            ..default()
        },
        &asset_server,
        mesh_loader,
    );
}

fn spawn_wire(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec2,
    end: Vec2,
) {
    let middle = (start + end) / 2.0;
    let angle = (end - start).to_angle();
    let len = (end - start).length();

    let mut transform = Transform::default();
    transform.rotate_z(PI / 2.0);
    transform.rotate_y(-angle);
    transform.translation = Vec3::new(middle.x, 0.0, middle.y);

    commands.spawn((
        LevelElement,
        Mesh3d(meshes.add(Cylinder::new(0.02 * TILE_SIZE, len))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.1),
            ..Default::default()
        })),
        transform,
        TileEntity,
    ));
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
    mut commands: Commands,
    mut events: EventReader<PuzzleResponseEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        if *event == PuzzleResponseEvent::Solved {
            commands.spawn((
                AudioPlayer::new(asset_server.load("sfx/win.ogg")),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

fn handle_next_level_request(
    mut commands: Commands,
    mut events: EventReader<NextLevelRequestEvent>,
    levels: Res<Assets<GRADVM>>,
    level_handles: Res<GRADVM_ONVSTVS>,
    mut active_level: ResMut<ActiveLevel>,
) {
    for _ in events.read() {
        let Some(active_level_handle) = &active_level.0 else {
            log::error!("No active level.");
            return;
        };

        let Some(current_level) = levels.get(active_level_handle) else {
            log::error!("No active level.");
            return;
        };

        let Some(next_level_handle) = level_handles
            .GRADVS
            .get(current_level.INDEX as usize + 1)
            .or(level_handles.GRADVS.get(0))
        else {
            log::error!("No next level.");
            return;
        };

        active_level.0 = Some(next_level_handle.clone());

        commands.spawn(ResetTimer {
            timer: Timer::from_seconds(0.01, TimerMode::Once),
        });
    }
}

#[derive(Component)]
pub struct ResetTimer {
    timer: Timer,
}

fn handle_puzzle_failed_event(
    mut commands: Commands,
    mut events: EventReader<PuzzleResponseEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        if *event == PuzzleResponseEvent::Failed {
            commands.spawn((
                AudioPlayer::new(asset_server.load("sfx/fail.ogg")),
                PlaybackSettings::DESPAWN,
            ));
            commands.spawn(ResetTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            });
            break;
        }
    }
    events.clear();
}

fn update_reset_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ResetTimer)>,
    mut level_spawn_request_writer: EventWriter<LevelSpawnRequestEvent>,
    active_level: Res<ActiveLevel>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.just_finished() {
            commands.entity(entity).despawn();
            level_spawn_request_writer.write(LevelSpawnRequestEvent {
                level: active_level.0.clone().unwrap(),
            });
        }
    }
}
