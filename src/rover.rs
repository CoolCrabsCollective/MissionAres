use crate::game_control::actions::{Action, ActionList, ActionType};
use crate::level::GRADVM;
use crate::level_spawner::{ActiveLevel, TILE_SIZE};
use crate::puzzle_evaluation::{PuzzleEvaluationRequestEvent, PuzzleResponseEvent};
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
    pub identifier: u8,
}

#[derive(Event)]
pub struct ActionListExecute {
    pub action_list: Vec<Vec<Action>>,
}

#[derive(Resource, Clone)]
pub struct ActionExecution {
    is_active: bool,
    action_list: Vec<Vec<Action>>,
    active_action_idx: Vec<usize>,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            execute_action_by_key.run_if(in_state(GameState::Game)),
        );
        app.add_systems(Update, start_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, action_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, continue_execution.run_if(in_state(GameState::Game)));
        app.insert_resource(ActionExecution {
            is_active: false,
            action_list: vec![],
            active_action_idx: vec![0usize, 0usize],
        });
    }
}

// TODO: link to UI
fn execute_action_by_key(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<ActionListExecute>,
    action_list: Res<ActionList>,
) {
    if input.just_pressed(KeyCode::KeyN) {
        events.write(ActionListExecute {
            action_list: action_list.actions.clone(),
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
        for mut rover in rover_query.iter_mut() {
            let robot_num = rover.identifier as usize;

            let actions = &action_execution.action_list[robot_num];

            let action = actions
                .get(action_execution.active_action_idx[robot_num])
                .unwrap();

            // Setup first action movements, validate level boundary
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
    mut rover_query: Query<(Entity, &mut RoverEntity, &mut Transform), With<RoverEntity>>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    mut action_execution: ResMut<ActionExecution>,
    time: Res<Time>,
) {
    if action_execution.is_active {
        let Some(level_handle) = &active_level.0 else {
            return;
        };
        let level = levels.get(level_handle).unwrap();

        let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
        let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

        // Iterate through each robot and move them progressively towards the next tile based on action
        for (mut entity, mut rover, mut trans) in rover_query.iter_mut() {
            let robot_num = rover.identifier as usize;

            let actions = &action_execution.action_list[robot_num];

            let logical_pos = rover.logical_position;

            let translation = &mut trans.translation;

            let end_x =
                (logical_pos.x as f32 * TILE_SIZE - effective_level_width / 2.0) + TILE_SIZE / 2.0;
            // mirror along the z to align correctly with how it looks in the level
            let end_z = (-logical_pos.y as f32 * TILE_SIZE + effective_level_height / 2.0)
                + TILE_SIZE / 2.0;
            let target = Vec3::new(end_x, translation.y, end_z);

            let diff = target - *translation;

            // Movement logic
            let SPEED = 2.0; // move later, dgaf rn

            let distance = diff.length();
            let step = SPEED * time.delta_secs();

            if distance > 0.01 {
                let dir = diff.normalize();
                let new_pos = *translation + dir * step.min(distance);
                trans.translation = new_pos;
            } else {
                trans.translation = target;
                action_execution.active_action_idx[robot_num] += 1;
                action_execution.is_active = false; // Wait on permission to continue, if puzzle evaluation passes
                commands.send_event(PuzzleEvaluationRequestEvent);

                // TODO: avoid skipping of action steps in other rover that is still in movement
            }
        }

        // If all rovers finished their lists, deactivate execution
        let all_done = action_execution
            .active_action_idx
            .iter()
            .enumerate()
            .all(|(i, idx)| *idx >= action_execution.action_list[i].len());

        if all_done {
            action_execution.is_active = false;
        }
    }
}

fn continue_execution(
    mut events: EventReader<PuzzleResponseEvent>,
    mut action_execution: ResMut<ActionExecution>,
    mut rover_query: Query<(&mut RoverEntity)>,
) {
    for event in events.read() {
        if *event == PuzzleResponseEvent::InProgress {
            action_execution.is_active = true;

            // Iterate through each robot
            for (robot_num, actions) in action_execution.action_list.iter().enumerate() {
                let mut rovers = rover_query.iter_mut().collect::<Vec<_>>();
                let Some(rover) = rovers.get_mut(robot_num) else {
                    continue;
                };

                let action = actions
                    .get(action_execution.active_action_idx[robot_num])
                    .unwrap();

                // Setup first action movements, validate level boundary
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
