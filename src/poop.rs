use crate::game_control::actions::{Action, ActionType, Robot};
use crate::level::GRADVM;
use crate::level_spawner::{ActiveLevel, TILE_SIZE};
use crate::title_screen::GameState;
use bevy::math::I8Vec2;
use bevy::prelude::*;

enum RoverStates {
    Standby,
    Moving,
}

#[derive(Component)]
pub struct RoverEntity {
    pub is_setup: bool,
    pub base_color: Color,
    pub gltf_handle: Handle<Gltf>,
    pub logical_position: I8Vec2,
    pub battery_level: u8,
}

#[derive(Event)]
pub struct ActionListExecute {
    pub action_list: Vec<Vec<Action>>,
}

#[derive(Resource)]
pub struct ActionExecution {
    is_active: bool,
    action_list: Vec<Action>,
    active_action_idx: usize,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, action_execution.run_if(in_state(GameState::Game)));
        app.insert_resource(ActionExecution {
            is_active: false,
            action_list: vec![],
            active_action_idx: 0,
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

fn start_execution(
    mut events: EventReader<ActionListExecute>,
    mut action_execution: ResMut<ActionExecution>,
    mut rover_query: Query<(&mut RoverEntity)>,
) {
    for event in events.read() {
        action_execution.is_active = true;

        action_execution.action_list = event.action_list.clone();

        action_execution.active_action_idx = 0usize;

        match action_execution.action_list.get(0) {
            None => {}
            Some(action) => {
                let mut rovers = rover_query.iter_mut().collect::<Vec<_>>();
                let Some(rover) = (match action.moves.1 {
                    Robot::ROVER1 => rovers.get_mut(0),
                    Robot::ROVER2 => rovers.get_mut(1),
                    _ => None,
                }) else {
                    return;
                };

                match action.moves.0 {
                    ActionType::MoveUp => {
                        rover.logical_position += I8Vec2::new(0, 1);
                    }
                    ActionType::MoveDown => {
                        rover.logical_position -= I8Vec2::new(0, 1);
                    }
                    ActionType::MoveLeft => {
                        rover.logical_position += I8Vec2::new(1, 0);
                    }
                    ActionType::MoveRight => {
                        rover.logical_position -= I8Vec2::new(1, 0);
                    }
                    ActionType::Wait => {}
                }
            }
        }
    }
}

fn action_execution(
    mut commands: Commands,
    mut events: EventReader<ActionListExecute>,
    mut rover_query: Query<(Entity, &mut RoverEntity), With<RoverEntity>>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    mut action_execution: ResMut<ActionExecution>,
) {
    if action_execution.is_active {
        let Some(level_handle) = &active_level.0 else {
            return;
        };
        let level = levels.get(level_handle).unwrap();

        let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
        let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

        let action = action_execution
            .action_list
            .get(action_execution.active_action_idx)
            .unwrap();

        let mut rover_query_list = rover_query.iter_mut().collect::<Vec<_>>();

        let Some(rover_entry) = (match action.moves.1 {
            Robot::ROVER1 => rover_query_list.get_mut(0),
            Robot::ROVER2 => rover_query_list.get_mut(1),
            _ => None,
        }) else {
            return;
        };

        let pos = rover_entry.1.logical_position;

        let effective_x =
            (pos.x as f32 * TILE_SIZE - effective_level_width / 2.0) + TILE_SIZE / 2.0;
        // mirror along the z to align correctly with how it looks in the level
        let effective_z =
            (-pos.y as f32 * TILE_SIZE + effective_level_height / 2.0) + TILE_SIZE / 2.0;

        // Now move entity towards effective position
    }
}
