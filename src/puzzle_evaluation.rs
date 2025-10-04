use bevy::prelude::*;

use crate::{
    level::{GRADVM, TEGVLA_TYPVS},
    level_spawner::{ActiveLevel, RoverEntity},
};

pub struct PuzzleEvaluationPlugin;

impl Plugin for PuzzleEvaluationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_puzzle_evaluation_request);
        app.add_systems(Update, debug_puzzle_evaluation);
        app.add_event::<PuzzleEvaluationRequestEvent>();
        app.insert_resource(PuzzleState {
            win_state: WinState::InProgress,
        });
    }
}

#[derive(Resource)]
pub struct PuzzleState {
    win_state: WinState,
}

#[derive(Resource)]
pub enum WinState {
    Win,
    InProgress,
    Lose,
}

#[derive(Event)]
pub struct PuzzleEvaluationRequestEvent;

fn on_puzzle_evaluation_request(
    mut events: EventReader<PuzzleEvaluationRequestEvent>,
    mut puzzle_state: ResMut<PuzzleState>,
    mut rovers: Query<&mut RoverEntity>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
) {
    for event in events.read() {
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
            puzzle_state.win_state = WinState::Win;
            break;
        }

        if let Some(_rover) = rovers.iter().find(|rover| rover.battery_level == 0) {
            log::info!("Rover is out of battery. Setting win state to lose.",);
            puzzle_state.win_state = WinState::Lose;
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
