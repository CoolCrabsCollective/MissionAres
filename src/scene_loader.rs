use crate::mesh_loader::{self, GLTFLoadConfig, MeshLoader, load_gltf};
use bevy::core_pipeline::Skybox;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasing};
use bevy::image::CompressedImageFormats;
use bevy::pbr::{
    CascadeShadowConfigBuilder, DirectionalLightShadowMap, ScreenSpaceAmbientOcclusion,
    ScreenSpaceAmbientOcclusionQualityLevel,
};
use bevy::prelude::*;
use bevy::render::camera::TemporalJitter;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy_rapier3d::prelude::*;
use bevy_water::{WaterPlugin, WaterQuality, WaterSettings};

pub struct SceneLoaderPlugin;

pub const CUBEMAPS: &[(&str, CompressedImageFormats)] =
    &[("test_skybox.png", CompressedImageFormats::NONE)];

#[derive(Resource)]
pub struct Cubemap {
    pub(crate) is_loaded: bool,
    pub(crate) image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct SceneElement;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_basic.after(mesh_loader::setup));

        app.add_systems(Update, asset_loaded);
        app.add_systems(Update, scene_switcher);
        app.add_plugins((
            WaterPlugin,
            TemporalAntiAliasPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
        ))
        .add_systems(Update, debug_render_toggle)
        .insert_resource(WaterSettings {
            height: -10.0,
            edge_scale: 0.5,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.3, 0.6, 0.9)))
        .insert_resource(DirectionalLightShadowMap { size: 4096 });
    }
}

fn scene_switcher(
    input: Res<ButtonInput<KeyCode>>,
    mut scene_elements: Query<(Entity, &SceneElement)>,
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_level: ResMut<WaterSettings>,
) {
    if !input.pressed(KeyCode::ControlLeft) && !input.pressed(KeyCode::ControlRight) {
        return;
    }

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Digit1) {
        for (entity, _) in scene_elements.iter_mut() {
            commands.entity(entity).despawn();
        }
        setup_basic(
            commands,
            asset_server,
            mesh_loader,
            meshes,
            materials,
            water_level,
        );
        return;
    } else if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Digit2) {
        for (entity, _) in scene_elements.iter_mut() {
            commands.entity(entity).despawn();
        }
        setup_kirby(
            commands,
            asset_server,
            mesh_loader,
            meshes,
            materials,
            water_level,
        );
        return;
    }
}

/// set up a simple 3D scene
fn setup_basic(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    mut water_level: ResMut<WaterSettings>,
) {
    water_level.height = -10.0;
    water_level.edge_scale = 0.5;
    water_level.water_quality = WaterQuality::Ultra;
    water_level.clarity = 0.1;
    commands.spawn((
        SceneElement,
        AudioPlayer::new(asset_server.load("test_song.ogg")),
        PlaybackSettings::LOOP,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    });
    commands.spawn((
        SceneElement,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 5000.0,
            shadows_enabled: true,
            affects_lightmapped_mesh_diffuse: true,
            shadow_depth_bias: 1.0,
            shadow_normal_bias: 1.0,
        },
        CascadeShadowConfigBuilder {
            maximum_distance: 500.0,
            ..default()
        }
        .build(),
    ));
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle.clone(),
    });

    commands.spawn((
        SceneElement,
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
        Transform::from_xyz(-0.5, 0.3, 4.5).with_rotation(Quat::from_axis_angle(Vec3::Y, 0.0)),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            rotation: Default::default(),
        },
        DistanceFog {
            color: Color::srgb(5.0, 0.25, 0.25),
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
        TemporalAntiAliasing::default(),
        TemporalJitter::default(),
        Bloom::default(),
    ));

    load_gltf(
        String::from("test_scene.glb"),
        GLTFLoadConfig {
            spawn: true,
            entity_initializer: add_scene_tag,
            generate_static_collider: true,
            collision_groups: CollisionGroups {
                memberships: Default::default(),
                filters: Default::default(),
            },
        },
        &mut asset_server,
        &mut mesh_loader,
    );
}

fn add_scene_tag(commands: &mut EntityCommands) {
    commands.insert(SceneElement);
}

fn setup_kirby(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    mut water_level: ResMut<WaterSettings>,
) {
    water_level.water_quality = WaterQuality::Basic;
    water_level.clarity = 0.0;
    commands.spawn((
        SceneElement,
        AudioPlayer::new(asset_server.load("test_song.ogg")),
        PlaybackSettings::LOOP,
    ));

    commands.insert_resource(ClearColor(Color::srgb(0.3, 0.6, 0.9)));
    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    });
    commands.spawn((
        SceneElement,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 5000.0,
            shadows_enabled: true,
            affects_lightmapped_mesh_diffuse: true,
            shadow_depth_bias: 1.0,
            shadow_normal_bias: 1.0,
        },
        CascadeShadowConfigBuilder {
            maximum_distance: 500.0,
            ..default()
        }
        .build(),
    ));
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle.clone(),
    });

    commands.spawn((
        SceneElement,
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
        Transform::from_xyz(-0.5, 0.3, 4.5).with_rotation(Quat::from_axis_angle(Vec3::Y, 0.0)),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            rotation: Default::default(),
        },
        DistanceFog {
            color: Color::srgb(0.25, 0.25, 0.25),
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
        TemporalAntiAliasing::default(),
        TemporalJitter::default(),
    ));

    load_gltf(
        String::from("test_scene2.glb"),
        GLTFLoadConfig {
            spawn: true,
            entity_initializer: add_scene_tag,
            generate_static_collider: true,
            collision_groups: CollisionGroups {
                memberships: Default::default(),
                filters: Default::default(),
            },
        },
        &mut asset_server,
        &mut mesh_loader,
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
