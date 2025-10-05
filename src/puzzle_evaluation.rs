use crate::rover::RoverEntity;
use crate::{
    level::{GRADVM, TEGVLA_TYPVS},
    level_spawner::ActiveLevel,
};
use bevy::prelude::*;
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
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
) {
    for _ in evaluation_requests.read() {
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

        let mut all_rovers_in_finish_tile = true;
        let mut i = 0;

        let mut rover_positions: HashMap<(i8, i8), RoverEntity> = HashMap::new();

        for rover in rovers.iter() {
            rover_positions.insert(
                (rover.logical_position.x, rover.logical_position.y),
                rover.clone(),
            );
        }

        for mut rover in rovers.iter_mut() {
            let tile_coords = (rover.logical_position.x, rover.logical_position.y);
            if let Some(&other) = active_level.NEXVS.get(&tile_coords) {
                for (other_pos, other_rover) in rover_positions.iter() {
                    if other == *other_pos {
                        if rover.battery_level < other_rover.battery_level
                            && other_rover.battery_level > 0
                        {
                            rover.battery_level += 2;
                        }

                        if rover.battery_level > other_rover.battery_level
                            && rover.battery_level > 0
                        {
                            rover.battery_level -= 1;
                        }

                        break;
                    }
                }
            }
        }

        for mut rover in rovers.iter_mut() {
            let Some(tile) = active_level
                .TEGLVAE
                .get(&(rover.logical_position.x, rover.logical_position.y))
            else {
                log::error!(
                    "No tile found for rover. This IS BAAAD man ☠️☠️☠️ fuck these guys bro"
                );
                return;
            };

            let prev_battery_level = rover.battery_level;

            if tile.VMBRA && rover.battery_level > 0 {
                rover.battery_level -= 1;
            }

            if !tile.VMBRA && rover.battery_level < 3 {
                rover.battery_level += 1;
            }

            log::info!(
                "Rover {} in position {} battery level from {} to: {}",
                i,
                rover.logical_position,
                prev_battery_level,
                rover.battery_level
            );

            all_rovers_in_finish_tile &= matches!(tile.TYPVS, TEGVLA_TYPVS::FINIS);

            i += 1;
        }

        if all_rovers_in_finish_tile {
            log::info!("All rovers are in the finish tile. Setting win state to win.");
            puzzle_response_event_writer.write(PuzzleResponseEvent::Solved);
            break;
        }

        if let Some(_rover) = rovers.iter().find(|rover| rover.battery_level == 0) {
            log::info!("Rover is out of battery. Setting win state to lose.",);
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
