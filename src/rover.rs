use crate::game_control::actions::{Action, ActionType};
use crate::hentai_anime::Animation;
use crate::level::{is_pos_in_level, GRADVM};
use crate::level_spawner::{ActiveLevel, TILE_SIZE};
use crate::puzzle_evaluation::{PuzzleEvaluationRequestEvent, PuzzleResponseEvent};
use crate::title_screen::GameState;
use bevy::math::ops::abs;
use bevy::math::EulerRot::XYZ;
use bevy::math::I8Vec2;
use bevy::prelude::*;
use std::f32::consts::PI;

const SPEED: f32 = 5.0;
const WAIT_TIME: f32 = 1.0;
const TURN_SPEED: f32 = 2.5;

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
    pub heading: f32,
}

#[derive(Event)]
pub struct ActionListExecute {
    pub action_list: Vec<Vec<Action>>,
}

#[derive(Resource, Clone, Debug)]
pub struct ActionExecution {
    is_active: bool,
    action_list: Vec<Vec<Action>>,
    active_action_idx: Vec<usize>,
    wait_time_start: Vec<f32>,
    is_waiting: Vec<bool>,
    is_turning: Vec<bool>,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, action_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, continue_execution.run_if(in_state(GameState::Game)));
        app.insert_resource(ActionExecution {
            is_active: false,
            action_list: vec![],
            active_action_idx: vec![0usize, 0usize],
            wait_time_start: vec![0.0],
            is_waiting: vec![false],
            is_turning: vec![false],
        });
        app.add_event::<ActionListExecute>();
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

fn setup_action_movements(
    rover: &mut RoverEntity,
    active_level: &Res<ActiveLevel>,
    levels: &Res<Assets<GRADVM>>,
    action_execution: &mut ResMut<ActionExecution>,
    robot_num: usize,
    time: &Res<Time>,
) {
    // Setup first action movements, validate level boundary
    let mut is_action_valid = true;
    let current_log_pos = rover.logical_position;

    let Some(level_handle) = &active_level.0 else {
        return;
    };
    let level = levels.get(level_handle).unwrap();

    let actions = &action_execution.action_list[robot_num];

    let action = actions
        .get(action_execution.active_action_idx[robot_num])
        .unwrap();

    let mut new_heading = rover.heading;

    match action.moves.0 {
        ActionType::MoveUp => {
            rover.logical_position += I8Vec2::new(0, 1);

            new_heading = -PI / 2.0;

            if !is_pos_in_level(level, &rover.logical_position) {
                is_action_valid = false;
            }
        }
        ActionType::MoveDown => {
            if rover.logical_position.y == 0 {
                is_action_valid = false;
            } else {
                rover.logical_position -= I8Vec2::new(0, 1);

                new_heading = PI / 2.0;

                if !is_pos_in_level(level, &rover.logical_position) {
                    is_action_valid = false;
                }
            }
        }
        ActionType::MoveLeft => {
            if rover.logical_position.x == 0 {
                is_action_valid = false;
            } else {
                rover.logical_position -= I8Vec2::new(1, 0);

                new_heading = -PI;

                if !is_pos_in_level(level, &rover.logical_position) {
                    is_action_valid = false;
                }
            }
        }
        ActionType::MoveRight => {
            rover.logical_position += I8Vec2::new(1, 0);

            new_heading = PI;

            if !is_pos_in_level(level, &rover.logical_position) {
                is_action_valid = false;
            }
        }
        ActionType::Wait => {
            action_execution.wait_time_start[robot_num] = time.elapsed_secs_wrapped();
            action_execution.is_waiting[robot_num] = true;
        }
    }

    if !is_action_valid {
        action_execution.wait_time_start[robot_num] = time.elapsed_secs_wrapped();
        action_execution.is_waiting[robot_num] = true;

        rover.logical_position = current_log_pos;
    } else {
        if rover.heading != new_heading {
            action_execution.is_turning[robot_num] = true;
            rover.heading = new_heading;
        }
    }
}

fn start_execution(
    mut events: EventReader<ActionListExecute>,
    mut action_execution: ResMut<ActionExecution>,
    mut rover_query: Query<(&mut RoverEntity)>,
    time: Res<Time>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    mut player_query: Query<(&mut AnimationPlayer, &mut Animation), With<RoverEntity>>,
) {
    for event in events.read() {
        if action_execution.is_active {
            return; // Avoid double execution
        }

        // Start animations
        for (mut player, animation) in player_query.iter_mut() {
            for hentai in &animation.animation_list {
                player.play(hentai.clone()).repeat();
            }
        }

        action_execution.is_active = true;

        action_execution.action_list = event.action_list.clone();

        action_execution.active_action_idx = vec![0usize; action_execution.action_list.len()];

        // Iterate through each robot
        for mut rover in rover_query.iter_mut() {
            let robot_num = rover.identifier as usize;

            // Setup first action movements, validate level boundary
            setup_action_movements(
                &mut rover,
                &active_level,
                &levels,
                &mut action_execution,
                robot_num,
                &time,
            );
        }
    }
}

fn action_execution(
    mut commands: Commands,
    mut rover_query: Query<(Entity, &mut RoverEntity, &mut Transform), With<RoverEntity>>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    mut action_execution: ResMut<ActionExecution>,
    time: Res<Time>,
    mut player_query: Query<(&mut AnimationPlayer, &mut Animation), With<RoverEntity>>,
) {
    if action_execution.is_active {
        let Some(level_handle) = &active_level.0 else {
            return;
        };
        let level = levels.get(level_handle).unwrap();

        let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
        let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

        // Iterate through each robot and move them progressively towards the next tile based on action
        for (_, mut rover, mut trans) in rover_query.iter_mut() {
            let robot_num = rover.identifier as usize;

            // If in wait, skip rest of loop logic
            if action_execution.is_waiting[robot_num] {
                let current_time = time.elapsed_secs_wrapped();

                let wait_duration = current_time - action_execution.wait_time_start[robot_num];

                if wait_duration > WAIT_TIME {
                    action_execution.active_action_idx[robot_num] += 1;
                    action_execution.is_active = false; // Wait on permission to continue, if puzzle evaluation passes
                    commands.send_event(PuzzleEvaluationRequestEvent);

                    action_execution.is_waiting[robot_num] = false;
                }

                continue;
            }

            if action_execution.is_turning[robot_num] {
                let current_rot = &trans.rotation.to_euler(XYZ);
                let current_heading = current_rot.1;
                dbg!(&trans.rotation.to_euler(XYZ));

                let diff = trans
                    .rotation
                    .angle_between(Quat::from_rotation_y(rover.heading));

                if abs(diff) > 0.1 {
                    let step = TURN_SPEED * time.delta_secs();

                    trans.rotation = trans
                        .rotation
                        .slerp(Quat::from_rotation_y(rover.heading), step);
                } else {
                    trans.rotation = Quat::from_rotation_y(rover.heading);
                    action_execution.is_turning[robot_num] = false;
                }

                continue;
            }

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
        dbg!(&action_execution);
        let all_done = action_execution
            .active_action_idx
            .iter()
            .enumerate()
            .all(|(i, idx)| *idx >= action_execution.action_list[i].len());

        if all_done {
            action_execution.is_active = false;

            // Stop animations
            for (mut player, _) in player_query.iter_mut() {
                player.stop_all();
            }
        }
    }
}

fn continue_execution(
    mut events: EventReader<PuzzleResponseEvent>,
    mut action_execution: ResMut<ActionExecution>,
    mut rover_query: Query<&mut RoverEntity>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    time: Res<Time>,
) {
    for event in events.read() {
        if *event == PuzzleResponseEvent::InProgress {
            action_execution.is_active = true;

            // Iterate through each robot and move them progressively towards the next tile based on action
            for mut rover in rover_query.iter_mut() {
                let robot_num = rover.identifier as usize;

                // Setup first action movements, validate level boundary
                setup_action_movements(
                    &mut rover,
                    &active_level,
                    &levels,
                    &mut action_execution,
                    robot_num,
                    &time,
                );
            }
        }
    }
}
