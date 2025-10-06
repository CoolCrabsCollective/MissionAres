use crate::particle::particle::Particle;
use crate::rover::RoverEntity;
use crate::title_screen::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, NotShadowCaster, StandardMaterial};
use bevy::prelude::*;

pub struct FailParticlePlugin;

impl Plugin for FailParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            fail_particle_spawner.run_if(in_state(GameState::Execution)),
        );
    }
}

fn fail_particle_spawner(
    mut commands: Commands,
    mut query: Query<(&mut RoverEntity, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
) {
    for (mut rover, transform) in query.iter_mut() {
        if !rover.collided || rover.spawned_fail_particle {
            continue;
        }
        rover.spawned_fail_particle = true;

        let texture_handle = asset_server.load("fail_particle.png");
        let quad = meshes.add(Rectangle::new(2.0, 2.0));
        let dust_material_handle = materials.add(StandardMaterial {
            base_color: Color::srgba(0.737, 0.518, 0.261, 0.4),
            base_color_texture: Some(texture_handle),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        let mut billboard_transform = transform.clone();
        billboard_transform.translation.y += 0.5;

        let (camera_transform) = camera_transform_query.single().unwrap();
        let lookat_pos = billboard_transform.translation + camera_transform.forward() * 1.0;
        billboard_transform.look_at(lookat_pos, camera_transform.up());

        commands.spawn((
            Particle {
                lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                velocity: Vec3::Y,
                angular_velocity: 0.0,
                opacity_function: Box::new(|p| 1.0),
                scale_function: Box::new(|p| p),
            },
            billboard_transform,
            Mesh3d(quad),
            MeshMaterial3d(dust_material_handle),
            NotShadowCaster,
        ));
    }
}
