use crate::game_control::actions::ActionType::Wait;
use crate::rover::{ActionExecution, RoverCollectable, RoverEntity};
use crate::{level::GRADVM, level_spawner::ActiveLevel};
use bevy::prelude::*;
use std::cmp::min;
use std::collections::HashMap;

pub struct PuzzleEvaluationPlugin;

impl Plugin for PuzzleEvaluationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_puzzle_evaluation_request);
        app.add_systems(Update, debug_puzzle_evaluation);
        app.add_event::<PuzzleEvaluationRequestEvent>();
        app.add_event::<PuzzleResponseEvent>();
    }
}

#[derive(Event, PartialEq)]
pub enum PuzzleResponseEvent {
    Solved,
    Failed,
    InProgress,
}

#[derive(Event)]
pub struct PuzzleEvaluationRequestEvent;

fn on_puzzle_evaluation_request(
    mut evaluation_requests: EventReader<PuzzleEvaluationRequestEvent>,
    mut puzzle_response_event_writer: EventWriter<PuzzleResponseEvent>,
    mut rovers: Query<&mut RoverEntity>,
    minerals: Query<&RoverCollectable>,
    action_execution: Res<ActionExecution>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
) {
    for _ in evaluation_requests.read() {
        if minerals.is_empty() {
            puzzle_response_event_writer.write(PuzzleResponseEvent::Solved);
            break;
        }

        let Some(active_level_handle) = &active_level.0 else {
            log::error!(
                "No active level. How the FUCK could you request that I evaluate the puzzle?"
            );
            return;
        };

        let Some(active_level) = levels.get(active_level_handle) else {
            log::error!(
                "No active level. How the FUCK could you request that I evaluate the puzzle?"
            );
            return;
        };
        let mut i = 0;

        let mut rover_positions: HashMap<(i8, i8), RoverEntity> = HashMap::new();

        for rover in rovers.iter() {
            rover_positions.insert(
                (rover.logical_position.x, rover.logical_position.y),
                rover.clone(),
            );
        }

        for mut rover in rovers.iter_mut() {
            if rover.is_acting || rover.is_done {
                continue; // Do not affect battery level for rovers still acting
            }

            let tile_coords = (rover.logical_position.x, rover.logical_position.y);
            if let Some(&other) = active_level.NEXVS.get(&tile_coords) {
                for (other_pos, other_rover) in rover_positions.iter() {
                    let Some(tile_first) = active_level
                        .TEGLVAE
                        .get(&(rover.logical_position.x, rover.logical_position.y))
                    else {
                        log::error!(
                            "No tile found for rover. This IS BAAAD man ☠️☠️☠️ fuck these guys bro"
                        );
                        return;
                    };

                    let Some(tile_second) = active_level.TEGLVAE.get(&(
                        other_rover.logical_position.x,
                        other_rover.logical_position.y,
                    )) else {
                        log::error!(
                            "No tile found for rover. This IS BAAAD man ☠️☠️☠️ fuck these guys bro"
                        );
                        return;
                    };

                    if other == *other_pos {
                        if tile_first.VMBRA && tile_second.VMBRA {
                            println!("BOTH IN UMBRA");
                            if rover.battery_level < other_rover.battery_level
                                && other_rover.battery_level > 0
                            {
                                rover.battery_level += 1;
                                rover.battery_level = min(rover.battery_level, 3);
                            }

                            if rover.battery_level > other_rover.battery_level
                                && rover.battery_level > 0
                            {
                                rover.battery_level -= 1;
                            }
                        } else if tile_first.VMBRA || tile_second.VMBRA {
                            println!("ONE IN UMBRA");
                            if tile_first.VMBRA && other_rover.battery_level > 0 {
                                println!("PROVIDING POWER");
                                rover.battery_level += 1;
                                rover.battery_level = min(rover.battery_level, 3);
                            }

                            if tile_second.VMBRA && rover.battery_level > 0 {
                                println!("GIVING POWER");
                                rover.battery_level -= 1;
                            }
                        } else {
                            println!("BOTH IN THE SUN");
                        }

                        break;
                    }
                }
            }
        }

        let rover_executions = action_execution.action_states.clone();
        for mut rover in rovers.iter_mut() {
            if rover.is_done {
                continue;
            }

            let Some(tile) = active_level
                .TEGLVAE
                .get(&(rover.logical_position.x, rover.logical_position.y))
            else {
                log::error!(
                    "No tile found for rover. This IS BAAAD man ☠️☠️☠️ fuck these guys bro"
                );
                return;
            };

            let state = rover_executions.get(i).unwrap();

            //println!(
            //    "Action list len {}, active action index {}",
            //    state.action_list.len(),
            //    state.active_action_idx
            //);
            rover.is_done = state.action_list.len() == state.active_action_idx;
            if state.active_action_idx > 0
                && let Some(state) = state.action_list.get(state.active_action_idx - 1)
            {
                print!("{:?}", state.moves.0);
                if tile.VMBRA {
                    println!(" to tile in shadow");
                } else {
                    println!(" to tile in sun");
                }

                if state.moves.0 != Wait && rover.battery_level > 0 && !rover.is_acting {
                    println!("Losing 1 battery");
                    rover.battery_level -= 1;
                }

                if !tile.VMBRA && rover.battery_level < 3 && !rover.is_acting {
                    println!("Gaining 1 battery");
                    rover.battery_level += 1;
                }
            }
            i += 1;
        }

        if let Some(_rover) = rovers.iter().find(|rover| rover.collided) {
            println!("PUZZLE FAILED! Why? VEHICVLVM MOBILE COLLIDIT!");
            puzzle_response_event_writer.write(PuzzleResponseEvent::Failed);
            break;
        }

        if rovers
            .iter()
            .enumerate()
            .find(|(idx, _rover)| {
                rover_executions[*idx].active_action_idx
                    != action_execution.action_states[*idx].action_list.len()
            })
            .is_none()
        {
            println!("PUZZLE FAILED! Why? NVLLAE ACTIONES AMPLIVS");
            puzzle_response_event_writer.write(PuzzleResponseEvent::Failed);
            break;
        }

        puzzle_response_event_writer.write(PuzzleResponseEvent::InProgress);
    }
}

fn debug_puzzle_evaluation(
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<PuzzleEvaluationRequestEvent>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        event_writer.write(PuzzleEvaluationRequestEvent);
    }
}
