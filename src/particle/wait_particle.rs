use crate::particle::particle::Particle;
use crate::rover::{ActionExecution, RoverEntity};
use crate::title_screen::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, NotShadowCaster, StandardMaterial};
use bevy::prelude::*;

pub struct WaitParticlePlugin;

impl Plugin for WaitParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            wait_particle_spawner.run_if(in_state(GameState::Execution)),
        );
    }
}

fn wait_particle_spawner(
    mut commands: Commands,
    mut query: Query<(&mut RoverEntity, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    action_execution: Res<ActionExecution>,
) {
    for (mut rover, transform) in query.iter_mut() {
        let robot_num = rover.identifier as usize;

        if rover.is_done {
            continue;
        }

        // Check if this rover is currently waiting
        if robot_num >= action_execution.action_states.len() {
            continue;
        }

        let is_waiting = action_execution.action_states[robot_num].is_waiting;

        // If not waiting, reset the flag
        if !is_waiting {
            rover.spawned_wait_particle = false;
            continue;
        }

        // If already spawned a particle for this wait, skip
        if rover.spawned_wait_particle {
            continue;
        }

        rover.spawned_wait_particle = true;

        let texture_handle = asset_server.load("command_icons/clock_outlined.png");
        let quad = meshes.add(Rectangle::new(0.5, 0.5));
        let wait_material_handle = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.9),
            base_color_texture: Some(texture_handle),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        let mut billboard_transform = transform.clone();
        billboard_transform.translation.y += 2.0;

        let camera_transform = camera_transform_query.single().unwrap();
        let lookat_pos = billboard_transform.translation + camera_transform.forward() * 1.0;
        billboard_transform.look_at(lookat_pos, camera_transform.up());

        commands.spawn((
            Particle {
                lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                velocity: Vec3::Y * 0.5,
                angular_velocity: 0.0,
                opacity_function: Box::new(|p| 1.0 - p),
                scale_function: Box::new(|p| 1.0 + p * 0.4),
            },
            billboard_transform,
            Mesh3d(quad),
            MeshMaterial3d(wait_material_handle),
            NotShadowCaster,
        ));
    }
}
