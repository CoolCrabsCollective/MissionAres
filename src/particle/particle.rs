use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::math::ops::fract;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Component, Entity, Mesh, Query, Res, ResMut, Time, Timer};

pub struct ParticlePlugin;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (particle_opacity_update, particle_remove));
    }
}

pub fn particle_opacity_update(
    mut query: Query<(&Particle, &mut MeshMaterial3d<StandardMaterial>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (particle, mut material) in query.iter_mut() {
        let fraction = particle.lifetime.fraction();
        let slerp_x = fract(2.0 * fraction);
        let slerp_val = 3.0 * slerp_x.powf(2.0) + 2.0 * slerp_x.powf(3.0);
        let opacity = if fraction < 0.5 {
            slerp_val
        } else {
            1.0 - slerp_val
        };

        // let m = meshes.get_mut(material);
    }
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
