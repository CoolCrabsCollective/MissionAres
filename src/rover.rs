use crate::game_control::actions::{Action, ActionType};
use crate::hentai_anime::Animation;
use crate::level::{is_pos_in_level, GRADVM};
use crate::level_spawner::{ActiveLevel, TILE_SIZE};
use crate::puzzle_evaluation::{PuzzleEvaluationRequestEvent, PuzzleResponseEvent};
use crate::title_screen::GameState;
use bevy::math::ops::abs;
use bevy::math::I8Vec2;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

const SPEED: f32 = 7.5;
const WAIT_ACTION_TIME: f32 = 0.5;
const TURN_SPEED: f32 = 5.0;

const WAIT_BETWEEN_TURNS: f32 = 0.25;

#[derive(Clone)]
pub enum RoverStates {
    Standby,
    Moving,
}

#[derive(Component, Clone)]
pub struct RoverEntity {
    pub is_acting: bool,
    pub is_turn_done: bool,
    pub base_color: Color,
    pub gltf_handle: Handle<Gltf>,
    pub logical_position: I8Vec2,
    pub battery_level: u8,
    pub identifier: u8,
    pub heading: f32,
    pub rover_state: RoverStates,
    pub collided: bool,
    pub spawned_fail_particle: bool,
    pub spawned_wait_particle: bool,
    pub is_done: bool,
}

#[derive(Component)]
pub struct RoverCollectable;

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
    pub is_built: bool,
    pub is_evaluating: bool,
    pub action_states: Vec<RoverActionState>,
}

#[derive(Component)]
pub struct BetweenTurnsTimer {
    timer: Timer,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_execution.run_if(not(in_state(GameState::TitleScreen))),
                action_execution.run_if(not(in_state(GameState::TitleScreen))),
                continue_execution.run_if(not(in_state(GameState::TitleScreen))),
                update_rover_collectables.run_if(not(in_state(GameState::TitleScreen))),
                update_rover_sounds.run_if(not(in_state(GameState::TitleScreen))),
                detect_move_done.run_if(in_state(GameState::Execution)),
                update_betweenturns_timer.run_if(in_state(GameState::Execution)),
            ),
        );
        app.insert_resource(ActionExecution {
            is_built: false,
            is_evaluating: false,
            action_states: vec![],
        });
        app.add_event::<ActionListExecute>();
    }
}

fn setup_action_movements(
    active_level: &Res<ActiveLevel>,
    levels: &Res<Assets<GRADVM>>,
    action_execution: &mut ResMut<ActionExecution>,
    time: &Res<Time>,
    rover_query: &mut Query<&mut RoverEntity>,
) {
    for rover in rover_query.iter() {
        if rover.is_acting {
            return;
        }
    }

    let mut prev_pos_vec = Vec::new();
    let mut position_vec = Vec::new();

    for mut rover in rover_query.iter_mut() {
        let robot_num = rover.identifier as usize;

        let mut is_action_valid = true;
        let prev_pos = rover.logical_position;

        let Some(level_handle) = &active_level.0 else {
            return;
        };
        let level = levels.get(level_handle).unwrap();

        let actions = &action_execution.action_states[robot_num].action_list;
        if actions.is_empty() {
            continue;
        }

        let Some(action) = actions.get(action_execution.action_states[robot_num].active_action_idx)
        else {
            continue;
        };

        let mut new_heading = rover.heading;
        let mut new_pos = rover.logical_position;

        // Predict new position
        let action_type = action.moves.0;
        match action_type {
            ActionType::MoveUp => {
                new_pos += I8Vec2::new(0, 1);
                new_heading = -PI / 2.0;
            }
            ActionType::MoveDown => {
                new_pos -= I8Vec2::new(0, 1);
                new_heading = PI / 2.0;
            }
            ActionType::MoveLeft => {
                new_pos -= I8Vec2::new(1, 0);
                new_heading = 0.0;
            }
            ActionType::MoveRight => {
                new_pos += I8Vec2::new(1, 0);
                new_heading = PI;
            }
            ActionType::Wait => {
                action_execution.action_states[robot_num].wait_time_start =
                    time.elapsed_secs_wrapped();
                action_execution.action_states[robot_num].wait_time = WAIT_ACTION_TIME;
                action_execution.action_states[robot_num].is_waiting = true;
            }
        }

        if !is_pos_in_level(level, &new_pos)
            || rover.battery_level == 0 && action_type != ActionType::Wait
        {
            is_action_valid = false;
        }

        if position_vec.contains(&new_pos) {
            is_action_valid = false;
        }

        let tuple = (new_pos, prev_pos);
        if prev_pos_vec.contains(&tuple) {
            is_action_valid = false;
        }

        rover.rover_state = RoverStates::Standby;

        if !is_action_valid {
            action_execution.action_states[robot_num].wait_time_start = time.elapsed_secs_wrapped();
            action_execution.action_states[robot_num].is_waiting = true;
            rover.logical_position = prev_pos;
            rover.collided = true;

            if rover.heading != new_heading {
                action_execution.action_states[robot_num].is_turning = true;
                rover.heading = new_heading;
                rover.is_acting = true;
            }
        } else {
            println!(
                "Setting position for rover {}, {}",
                rover.identifier, new_pos
            );
            rover.logical_position = new_pos;
            rover.rover_state = RoverStates::Moving;
            rover.is_acting = true;
            rover.is_turn_done = false;

            if rover.heading != new_heading {
                action_execution.action_states[robot_num].is_turning = true;
                rover.heading = new_heading;
            }
        }
        position_vec.push(rover.logical_position);
        prev_pos_vec.push((prev_pos, rover.logical_position));
    }
}

fn start_execution(
    mut events: EventReader<ActionListExecute>,
    mut action_execution: ResMut<ActionExecution>,
    mut rover_query: Query<(&mut RoverEntity)>,
    time: Res<Time>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    mut player_query: Query<&mut AnimationPlayer>,
    mut animation: Query<&Animation, With<RoverEntity>>,
) {
    for event in events.read() {
        if action_execution.is_built {
            return; // Avoid double execution
        }

        // Start animations
        for animation in animation.iter_mut() {
            if let Some(player_entity) = animation.player_entity {
                if let Ok(mut player) = player_query.get_mut(player_entity) {
                    for hentai in &animation.animation_list {
                        player.play(hentai.clone()).repeat();
                        //println!("Start rover anime");
                    }
                }
            }
        }

        action_execution.is_built = true;

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

        //println!("Start execution");
        //dbg!(&action_execution.action_states);
        setup_action_movements(
            &active_level,
            &levels,
            &mut action_execution,
            &time,
            &mut rover_query,
        );
    }
}

fn detect_move_done(
    mut commands: Commands,
    query: Query<&RoverEntity>,
    mut action_execution: ResMut<ActionExecution>,
) {
    if action_execution.is_evaluating || !action_execution.is_built {
        return;
    }

    if query.is_empty() {
        return;
    }

    let mut i = 0;
    for rover in query.iter() {
        i += 1;
        if !rover.is_turn_done {
            return;
        }
    }

    action_execution.is_evaluating = true;

    commands.spawn(BetweenTurnsTimer {
        timer: Timer::new(Duration::from_secs_f32(WAIT_BETWEEN_TURNS), TimerMode::Once),
    });
}

fn update_betweenturns_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BetweenTurnsTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(entity).despawn();
            commands.send_event(PuzzleEvaluationRequestEvent);
        }
    }
}

fn action_execution(
    mut rover_query: Query<(Entity, &mut RoverEntity, &mut Transform)>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    mut action_execution: ResMut<ActionExecution>,
    time: Res<Time>,
) {
    if !action_execution.is_built {
        return;
    }
    let Some(level_handle) = &active_level.0 else {
        return;
    };
    let level = levels.get(level_handle).unwrap();

    let effective_level_width = level.LATIVIDO as f32 * TILE_SIZE;
    let effective_level_height = level.ALTIVIDO as f32 * TILE_SIZE;

    // Iterate through each robot and move them progressively towards the next tile based on action
    for (_, mut rover, mut trans) in rover_query.iter_mut() {
        if rover.is_turn_done {
            continue;
        }
        let robot_num = rover.identifier as usize;

        //dbg!(&action_execution.action_states[robot_num]);

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
                    rover.is_acting = false;
                    rover.is_turn_done = true;
                }

                action_execution.action_states[robot_num].is_waiting = false;
            }

            continue;
        }

        if action_execution.action_states[robot_num].is_turning {
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
        let end_z =
            (-logical_pos.y as f32 * TILE_SIZE + effective_level_height / 2.0) + TILE_SIZE / 2.0;
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
            rover.is_acting = false;
            rover.is_turn_done = true;
            println!("End of moving for rover {}", rover.identifier);
            println!(
                "New Action Idx {}",
                action_execution.action_states[robot_num].active_action_idx
            );
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
                events.clear();
                action_execution.is_built = false;
                action_execution.is_evaluating = false;
                break;
            }
            PuzzleResponseEvent::Failed => {
                events.clear();
                action_execution.is_built = false;
                action_execution.is_evaluating = false;
                break;
            }
            PuzzleResponseEvent::InProgress => {
                for mut rover in rover_query.iter_mut() {
                    rover.is_turn_done = false;
                }

                //println!("In Progress!");
                //dbg!(&action_execution.action_states);

                // Setup first action movements, validate level boundary
                setup_action_movements(
                    &active_level,
                    &levels,
                    &mut action_execution,
                    &time,
                    &mut rover_query,
                );

                action_execution.is_evaluating = false;

                // // Iterate through each robot and move them progressively towards the next tile based on action
                // for mut rover in rover_query.iter_mut() {
                //     let robot_num = rover.identifier as usize;
                //
                //     // Make rover wait before performing next action
                //     action_execution.action_states[robot_num].wait_time_start =
                //         time.elapsed_secs_wrapped();
                //     action_execution.action_states[robot_num].wait_time = WAIT_BETWEEN_ACTS;
                //     action_execution.action_states[robot_num].is_waiting = true;
                //     println!("bruhhhhhhh");
                // }
            }
        }
    }
}

fn update_rover_collectables(
    mut commands: Commands,
    collectable_queries: Query<(Entity, &Transform, &RoverCollectable)>,
    rovers: Query<(&Transform, &RoverEntity), Without<RoverCollectable>>,
) {
    for (rover_transform, rover) in rovers.iter() {
        for (collectable_entity, collectable_transform, rover_collectable) in
            collectable_queries.iter()
        {
            if rover_transform
                .translation
                .distance(collectable_transform.translation)
                < 1.0
            {
                commands.entity(collectable_entity).despawn();
            }
        }
    }
}

fn update_rover_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &RoverEntity)>,
    children: Query<&Children>,
    audio_players: Query<Entity, With<AudioPlayer>>,
) {
    for (entity, rover) in query.iter_mut() {
        // Check if this rover already has an AudioPlayer child
        let mut has_audio = false;
        if let Ok(child_list) = children.get(entity) {
            for child in child_list.iter() {
                if audio_players.get(child).is_ok() {
                    has_audio = true;
                    // If in Standby, despawn the player
                    if matches!(rover.rover_state, RoverStates::Standby) {
                        commands.entity(child).despawn();
                    }
                }
            }
        }

        // If rover is moving and has no audio, spawn it
        if matches!(rover.rover_state, RoverStates::Moving) && !has_audio {
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    AudioPlayer::new(asset_server.load("sfx/rover.ogg")),
                    PlaybackSettings::LOOP,
                ));
            });
        }
    }
}
