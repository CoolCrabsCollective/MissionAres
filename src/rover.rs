use crate::game_control::actions::{Action, ActionType};
use crate::hentai_anime::Animation;
use crate::level::{is_pos_in_level, GRADVM};
use crate::level_spawner::{ActiveLevel, TILE_SIZE};
use crate::mesh_loader::MeshLoader;
use crate::puzzle_evaluation::{PuzzleEvaluationRequestEvent, PuzzleResponseEvent};
use crate::title_screen::GameState;
use bevy::math::ops::abs;
use bevy::math::EulerRot::XYZ;
use bevy::math::I8Vec2;
use bevy::prelude::*;
use std::f32::consts::PI;

const SPEED: f32 = 5.0;
const WAIT_ACTION_TIME: f32 = 1.0;
const TURN_SPEED: f32 = 2.5;

const WAIT_BETWEEN_ACTS: f32 = 0.5;

#[derive(Clone)]
pub enum CardinalDirection {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}

#[derive(Clone)]
pub enum RoverStates {
    Standby,
    Moving, /*(CardinalDirection)*/
}

#[derive(Component, Clone)]
pub struct RoverEntity {
    pub is_setup: bool,
    pub base_color: Color,
    pub gltf_handle: Handle<Gltf>,
    pub logical_position: I8Vec2,
    pub battery_level: u8,
    pub identifier: u8,
    pub heading: f32,
    pub rover_state: RoverStates,
}

#[derive(Event)]
pub struct ActionListExecute {
    pub action_list: Vec<Vec<Action>>,
}

#[derive(Clone, Debug)]
pub struct RoverActionState {
    pub action_list: Vec<Action>,
    pub active_action_idx: usize,
    pub wait_time_start: f32,
    pub is_waiting: bool,
    pub is_turning: bool,
    pub wait_time: f32,
}

#[derive(Resource, Clone, Debug)]
pub struct ActionExecution {
    pub is_active: bool,
    pub action_states: Vec<RoverActionState>,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, action_execution.run_if(in_state(GameState::Game)));
        app.add_systems(Update, continue_execution.run_if(in_state(GameState::Game)));
        app.insert_resource(ActionExecution {
            is_active: false,
            action_states: vec![],
        });
        app.add_event::<ActionListExecute>();
    }
}

fn get_base_material(
    asset_path: String,
    mut mesh_loader: ResMut<MeshLoader>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    for loaded_gltf in mesh_loader.0.iter_mut() {
        if !loaded_gltf.processed {
            continue;
        }

        let Some(gltf) = gltf_assets.get(&loaded_gltf.gltf_handle) else {
            continue;
        };

        // if let Some(material) get_material_from_gltf_node(node_handle, &gltf_meshes, &nodes)
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
    println!("Setup Action Movements");

    // Setup first action movements, validate level boundary
    let mut is_action_valid = true;
    let current_log_pos = rover.logical_position;

    let Some(level_handle) = &active_level.0 else {
        return;
    };
    let level = levels.get(level_handle).unwrap();

    let actions = &action_execution.action_states[robot_num].action_list;

    if actions.is_empty() {
        // No actions to execute lol
        let all_done = action_execution
            .action_states
            .iter()
            .enumerate()
            .all(|(i, state)| {
                state.active_action_idx >= action_execution.action_states[i].action_list.len()
            });

        if all_done {
            action_execution.is_active = false;
        }

        return;
    }

    println!(
        "Active Action Idx {}",
        action_execution.action_states[robot_num].active_action_idx
    );
    let action = actions
        .get(action_execution.action_states[robot_num].active_action_idx)
        .unwrap();

    let mut new_heading = rover.heading;

    let action_attempted = action.moves.0.clone();

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
            action_execution.action_states[robot_num].wait_time_start = time.elapsed_secs_wrapped();

            action_execution.action_states[robot_num].wait_time = WAIT_ACTION_TIME;

            action_execution.action_states[robot_num].is_waiting = true;
        }
    }
    rover.rover_state = RoverStates::Standby;
    if !is_action_valid {
        action_execution.action_states[robot_num].wait_time_start = time.elapsed_secs_wrapped();
        action_execution.action_states[robot_num].is_waiting = true;

        rover.logical_position = current_log_pos;
    } else {
        rover.rover_state = RoverStates::Moving; /*(match action_attempted {
        ActionType::MoveUp => CardinalDirection::UP,
        ActionType::MoveDown => CardinalDirection::DOWN,
        ActionType::MoveLeft => CardinalDirection::LEFT,
        ActionType::MoveRight => CardinalDirection::RIGHT,
        ActionType::Wait => panic!("we're moving lol"), // TODO UP WAIT UP RIGHT on level 1 causes this panic
        });*/
        if rover.heading != new_heading {
            action_execution.action_states[robot_num].is_turning = true;
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

        action_execution.action_states.clear();
        for action_list in event.action_list.iter() {
            action_execution.action_states.push(RoverActionState {
                action_list: action_list.clone(),
                active_action_idx: 0,
                wait_time_start: 0.0,
                is_waiting: false,
                is_turning: false,
                wait_time: 0.0,
            })
        }

        println!("Number of rovers: {}", action_execution.action_states.len());
        // Iterate through each robot
        for mut rover in rover_query.iter_mut() {
            let robot_num = rover.identifier as usize;
            println!("Starting action");
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
            if action_execution.action_states[robot_num].is_waiting {
                let current_time = time.elapsed_secs_wrapped();

                let wait_duration =
                    current_time - action_execution.action_states[robot_num].wait_time_start;

                if wait_duration > action_execution.action_states[robot_num].wait_time {
                    if action_execution.action_states[robot_num].wait_time == WAIT_ACTION_TIME {
                        // Only perform the following if wait action was reason for wait
                        if action_execution.action_states[robot_num].active_action_idx
                            < action_execution.action_states[robot_num].action_list.len()
                        {
                            action_execution.action_states[robot_num].active_action_idx += 1;
                        }
                        action_execution.is_active = false; // Wait on permission to continue, if puzzle evaluation passes
                        commands.send_event(PuzzleEvaluationRequestEvent);
                    }

                    action_execution.action_states[robot_num].is_waiting = false;
                }

                continue;
            }

            if action_execution.action_states[robot_num].is_turning {
                let current_rot = &trans.rotation.to_euler(XYZ);

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
                    action_execution.action_states[robot_num].is_turning = false;
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
                if action_execution.action_states[robot_num].active_action_idx
                    < action_execution.action_states[robot_num].action_list.len()
                {
                    action_execution.action_states[robot_num].active_action_idx += 1;
                }
                action_execution.is_active = false; // Wait on permission to continue, if puzzle evaluation passes
                commands.send_event(PuzzleEvaluationRequestEvent);

                // TODO: avoid skipping of action steps in other rover that is still in movement
            }
        }

        // If all rovers finished their lists, deactivate execution
        //dbg!(&action_execution);
        let all_done = action_execution
            .action_states
            .iter()
            .enumerate()
            .all(|(i, state)| {
                state.active_action_idx >= action_execution.action_states[i].action_list.len()
            });

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
        match event {
            PuzzleResponseEvent::Solved => {
                println!("Solved!");
                events.clear();
                break;
            }
            PuzzleResponseEvent::Failed => {
                println!("Failed!");
                events.clear();
                break;
            }
            PuzzleResponseEvent::InProgress => {
                action_execution.is_active = true;

                // Iterate through each robot and move them progressively towards the next tile based on action
                for mut rover in rover_query.iter_mut() {
                    let robot_num = rover.identifier as usize;
                    println!("Continue action");
                    // Setup first action movements, validate level boundary
                    setup_action_movements(
                        &mut rover,
                        &active_level,
                        &levels,
                        &mut action_execution,
                        robot_num,
                        &time,
                    );

                    // Make rover wait before performing next action
                    action_execution.action_states[robot_num].wait_time_start =
                        time.elapsed_secs_wrapped();
                    action_execution.action_states[robot_num].wait_time = WAIT_BETWEEN_ACTS;
                    action_execution.action_states[robot_num].is_waiting = true;
                }
            }
        }
    }
}
