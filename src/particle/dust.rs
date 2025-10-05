use crate::particle::particle::Particle;
use crate::rover::{RoverEntity, RoverStates};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

#[derive(Component)]
pub struct DustSpawner {
    pub(crate) timer: Timer,
}

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_dust);
    }
}

pub fn spawn_dust(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut DustSpawner, &RoverEntity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let (camera_transform) = camera_transform_query.single().unwrap();
    for (transform, mut dust, r_entity) in query.iter_mut() {
        match &r_entity.rover_state {
            RoverStates::Standby => {}
            RoverStates::Moving(direction) => {
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
                    billboard_transform.translation.y += 0.25;
                    billboard_transform.translation += transform.forward() * -1.0;

                    // GARBAGE
                    //billboard_transform.translation.x += match direction {
                    //    CardinalDirection::LEFT => 0.8 + rng.random_range(-0.2..0.2),
                    //    CardinalDirection::RIGHT => -0.8 + rng.random_range(-0.2..0.2),
                    //    _ => rng.random_range(-1.0..1.0),
                    //};
                    //billboard_transform.translation.z += match direction {
                    //    CardinalDirection::DOWN => 0.8 + rng.random_range(-0.2..0.2),
                    //    CardinalDirection::UP => -0.8 + rng.random_range(-0.2..0.2),
                    //    _ => rng.random_range(-1.0..1.0),
                    //};
                    //billboard_transform.translation.y += 0.2;

                    let lookat_pos =
                        billboard_transform.translation + camera_transform.forward() * 1.0;
                    billboard_transform.look_at(lookat_pos, camera_transform.up());

                    dust.timer.reset();
                    commands.spawn((
                        Particle {
                            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                            velocity: transform.forward() * -1.0,
                            angular_velocity: 20.0f32,
                        },
                        billboard_transform,
                        Mesh3d(quad),
                        MeshMaterial3d(dust_material_handle),
                        NotShadowCaster,
                    ));
                }
            }
        }
    }
}
