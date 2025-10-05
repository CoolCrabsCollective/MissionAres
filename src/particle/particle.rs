use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::math::ops::fract;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    Camera3d, Color, Commands, Component, Entity, Query, Res, ResMut, Time, Timer, Transform, With,
    Without,
};
use bevy::text::cosmic_text::Angle;

pub struct ParticlePlugin;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec3,
    pub angular_velocity: f32,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (particle_remove, update_particle));
    }
}

pub fn update_particle(
    mut query: Query<
        (&mut Transform, &Particle, &MeshMaterial3d<StandardMaterial>),
        Without<Camera3d>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let (camera_transform) = camera_transform_query.single().unwrap();
    let δ = time.delta().as_secs_f32();
    for (mut transform, particle, mat) in query.iter_mut() {
        let lookat_pos = transform.translation + camera_transform.forward() * 1.0;
        transform.look_at(lookat_pos, camera_transform.up());

        let forward = transform.forward();
        let angle_of_rotation: Angle =
            Angle::from_degrees((δ * particle.angular_velocity).to_degrees());
        transform.rotate_axis(-forward, angle_of_rotation.to_radians());
        transform.scale = dust_scale(particle.lifetime.elapsed_secs());
        transform.translation += δ * particle.velocity;

        let fraction = particle.lifetime.fraction();
        let slerp_x = fract(2.0 * fraction);
        let slerp_val = 3.0 * slerp_x.powf(2.0) - 2.0 * slerp_x.powf(3.0);

        let opacity = if fraction < 0.5 {
            slerp_val
        } else {
            1.0 - slerp_val
        };
        materials.get_mut(&mat.0.clone()).unwrap().base_color =
            Color::srgba(1.0, 1.0, 1.0, 0.5 * opacity);
    }
}

fn dust_scale(elapsed_time: f32) -> Vec3 {
    Vec3::new(
        1.0f32.min(elapsed_time * elapsed_time),
        1.0f32.min(elapsed_time * elapsed_time),
        1.0f32.min(elapsed_time * elapsed_time),
    )
}

pub fn particle_remove(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
) {
    for (entity, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.get_entity(entity).unwrap().despawn();
        }
    }
}
