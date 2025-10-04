use crate::game_control::actions::Action;
use crate::level::GRADVM;
use crate::level_spawner::{ActiveLevel, TILE_SIZE};
use crate::title_screen::GameState;
use bevy::math::I8Vec2;
use bevy::prelude::*;
use bevy::{ecs::resource::Resource, math::IVec2};

enum RoverStates {
    Standby,
    Moving,
}

#[derive(Resource)]
pub struct Rover {
    pub position: IVec2,
    pub state: RoverStates,
}

#[derive(Component)]
pub struct RoverEntity {
    pub is_setup: bool,
    pub base_color: Color,
    pub gltf_handle: Handle<Gltf>,
    pub logical_position: I8Vec2,
    pub battery_level: u8,
}

#[derive(Resource)]
pub struct RoverList {
    pub list: Vec<Rover>,
}

#[derive(Event)]
pub struct ActionListExecute {
    pub action_list: Vec<Action>,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_actions.run_if(in_state(GameState::Game)));
        app.insert_resource(RoverList {
            list: vec![
                Rover {
                    position: Default::default(),
                    state: RoverStates::Standby,
                },
                Rover {
                    position: Default::default(),
                    state: RoverStates::Standby,
                },
                Rover {
                    position: Default::default(),
                    state: RoverStates::Standby,
                },
            ],
        });
    }
}

fn setup_rover_colors(
    mut commands: Commands,
    mut rover_query: Query<(Entity, &mut RoverEntity), With<RoverEntity>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut rover_entity) in rover_query.iter_mut() {
        if !rover_entity.is_setup {
            // entity.

            rover_entity.is_setup = true;
        }
    }
}

fn execute_actions(
    mut commands: Commands,
    mut events: EventReader<ActionListExecute>,
    mut rover_query: Query<Entity, With<RoverEntity>>,
    mut rover_list: ResMut<RoverList>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
) {
    for event in events.read() {
        let Some(level_handle) = &active_level.0 else {
            continue;
        };
        let level = levels.get(level_handle).unwrap();

        let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
        let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

        for action in event.action_list.iter() {
            let rover_entities = rover_query.iter_mut();
            for (idx, entity) in rover_entities.enumerate() {
                let effective_x = (rover_list.list.get(idx).unwrap().position.x as f32 * TILE_SIZE
                    - effective_level_width / 2.0)
                    + TILE_SIZE / 2.0;
                let effective_z = (-rover_list.list.get(idx).unwrap().position.y as f32
                    * TILE_SIZE
                    + effective_level_height / 2.0)
                    + TILE_SIZE / 2.0;

                // TODO: smoothly move between current and target locations, execute one action at a time
                commands.entity(entity).insert(Transform::from_xyz(
                    effective_x as f32,
                    0.5,
                    effective_z as f32,
                ));
            }
        }
    }
}
