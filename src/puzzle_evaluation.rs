use bevy::prelude::*;

use crate::poop::RoverEntity;
use crate::{
    level::{GRADVM, TEGVLA_TYPVS},
    level_spawner::{ActiveLevel, AfterLevelSpawnEvent},
};

pub struct PuzzleEvaluationPlugin;

impl Plugin for PuzzleEvaluationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_puzzle_evaluation_request);
        app.add_systems(Update, debug_puzzle_evaluation);
        app.add_event::<PuzzleEvaluationRequestEvent>();
        app.add_event::<PuzzleSolvedEvent>();
        app.add_event::<PuzzleFailedEvent>();
    }
}

#[derive(Event)]
pub struct PuzzleSolvedEvent;

#[derive(Event)]
pub struct PuzzleFailedEvent;

#[derive(Event)]
pub struct PuzzleEvaluationRequestEvent;

fn on_puzzle_evaluation_request(
    mut evaluation_requests: EventReader<PuzzleEvaluationRequestEvent>,
    mut puzzle_solved_event_writer: EventWriter<PuzzleSolvedEvent>,
    mut puzzle_failed_event_writer: EventWriter<PuzzleFailedEvent>,
    mut rovers: Query<&mut RoverEntity>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
) {
    for _ in evaluation_requests.read() {
        log::info!("Received puzzle evaluation request.");
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
        for mut rover in rovers.iter_mut() {
            let Some(tile) = active_level.TEGLVAE.get(&(
                rover.logical_position.x as i8,
                rover.logical_position.y as i8,
            )) else {
                log::error!(
                    "No tile found for rover. This IS BAAAD man ☠️☠️☠️ fuck these guys bro"
                );
                return;
            };

            if tile.VMBRA {
                rover.battery_level -= 1;
                log::info!(
                    "Rover {} in position {} battery level went down to: {}",
                    i,
                    rover.logical_position,
                    rover.battery_level
                );
            }

            all_rovers_in_finish_tile &= matches!(tile.TYPVS, TEGVLA_TYPVS::FINIS);

            i += 1;
        }

        if all_rovers_in_finish_tile {
            log::info!("All rovers are in the finish tile. Setting win state to win.");
            puzzle_solved_event_writer.write(PuzzleSolvedEvent);
            break;
        }

        if let Some(_rover) = rovers.iter().find(|rover| rover.battery_level == 0) {
            log::info!("Rover is out of battery. Setting win state to lose.",);
            puzzle_failed_event_writer.write(PuzzleFailedEvent);
        }
    }
}

fn debug_puzzle_evaluation(
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<PuzzleEvaluationRequestEvent>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        log::info!("Writing puzzle evaluation request event.");
        event_writer.write(PuzzleEvaluationRequestEvent);
    }
}
