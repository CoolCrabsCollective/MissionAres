use bevy::prelude::*;

use crate::{
    level::{GRADVM, TEGVLA_TYPVS},
    level_spawner::{ActiveLevel, LevelElement, RoverEntity},
};

pub struct PuzzleEvaluationPlugin;

impl Plugin for PuzzleEvaluationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_puzzle_evaluation_request);
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
    let Some(active_level_handle) = &active_level.0 else {
        log::error!("No active level. How the FUCK could you request that I evaluate the puzzle?");
        return;
    };

    let Some(active_level) = levels.get(active_level_handle) else {
        log::error!("No active level. How the FUCK could you request that I evaluate the puzzle?");
        return;
    };

    for event in events.read() {
        // loop through the rovers and drain 1 battery if they are in shadow
        // if any rover reaches 0 batteries, set the win state to lose
        // if all rovers are in a finish tile, set the win state to win
        let mut all_rovers_in_finish_tile = true;
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
            }

            all_rovers_in_finish_tile &= matches!(tile.TYPVS, TEGVLA_TYPVS::FINIS);
        }

        if all_rovers_in_finish_tile {
            puzzle_state.win_state = WinState::Win;
            break;
        }

        if rovers.iter().all(|rover| rover.battery_level == 0) {
            puzzle_state.win_state = WinState::Lose;
        }
    }
}
