use crate::particle::particle::Particle;
use bevy::prelude::*;
use bevy::text::cosmic_text::Angle;
use rand::Rng;

#[derive(Component)]
pub struct DustSpawner {
    pub(crate) timer: Timer,
}

#[derive(Component, Copy, Clone)]
pub struct Dust;

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_dust, update_dust));
    }
}

pub fn spawn_dust(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut DustSpawner)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let (camera_transform) = camera_transform_query.single().unwrap();
    for (transform, mut dust) in query.iter_mut() {
        dust.timer.tick(time.delta());

        if (dust.timer.finished()) {
            let texture_handle = asset_server.load("dust.png");
            let quad = meshes.add(Rectangle::new(2.0, 2.0));
            let dust_material_handle = materials.add(StandardMaterial {
                base_color: Color::srgba(0.737, 0.518, 0.261, 0.4),
                base_color_texture: Some(texture_handle),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });

            let mut billboard_transform = transform.clone();
            billboard_transform.translation.x += rng.gen_range(-1.0..1.0);
            billboard_transform.translation.z += rng.gen_range(-1.0..1.0);
            billboard_transform.translation.y += 0.2;

            let lookat_pos = billboard_transform.translation + camera_transform.forward() * 1.0;
            billboard_transform.look_at(lookat_pos, camera_transform.up());

            dust.timer.reset();
            commands.spawn((
                Dust {},
                Particle {
                    lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                },
                billboard_transform,
                Mesh3d(quad),
                MeshMaterial3d(dust_material_handle),
            ));
        }
    }
}

pub fn update_dust(
    mut query: Query<&mut Transform, (With<Dust>, Without<Camera3d>)>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let (camera_transform) = camera_transform_query.single().unwrap();
    for (mut transform) in query.iter_mut() {
        let mut billboard_transform: Transform = transform.clone();

        let lookat_pos = billboard_transform.translation + camera_transform.forward() * 1.0;
        billboard_transform.look_at(lookat_pos, camera_transform.up());

        let angle_of_rotation: Angle =
            Angle::from_degrees((time.delta().as_secs_f32() * 1000.0f32).to_degrees());
        billboard_transform.rotate_axis(
            -billboard_transform.forward(),
            angle_of_rotation.to_radians(),
        );

        *transform = billboard_transform.clone();
    }
}
