use crate::game_control::actions::{Action, ActionType};
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
    action_list: Vec<Vec<Action>>,
    active_action_idx: Vec<usize>,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, action_execution.run_if(in_state(GameState::Game)));
        app.insert_resource(ActionExecution {
            is_active: false,
            action_list: vec![],
            active_action_idx: vec![0usize, 0usize],
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

        action_execution.active_action_idx = vec![0usize, 0usize];

        // Iterate through each robot
        for (robot_num, actions) in action_execution.action_list.iter().enumerate() {
            let mut rovers = rover_query.iter_mut().collect::<Vec<_>>();
            let Some(rover) = rovers.get_mut(robot_num) else {
                continue;
            };

            let action = actions
                .get(action_execution.active_action_idx[robot_num])
                .unwrap();

            // Setup first action movements
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

        // Iterate through each robot and move them progressively towards the next title based on action
        for (robot_num, actions) in action_execution.action_list.iter().enumerate() {
            let mut rovers = rover_query.iter_mut().collect::<Vec<_>>();
            let Some(rover_entry) = rovers.get_mut(robot_num) else {
                continue;
            };

            let action = actions
                .get(action_execution.active_action_idx[robot_num])
                .unwrap();

            let pos = rover_entry.1.logical_position;

            let effective_x =
                (pos.x as f32 * TILE_SIZE - effective_level_width / 2.0) + TILE_SIZE / 2.0;
            // mirror along the z to align correctly with how it looks in the level
            let effective_z =
                (-pos.y as f32 * TILE_SIZE + effective_level_height / 2.0) + TILE_SIZE / 2.0;

            // Now move entity towards effective position
        }
    }
}
