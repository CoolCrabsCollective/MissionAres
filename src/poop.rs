use crate::game_control::actions::Action;
use crate::level_spawner::RoverEntity;
use crate::title_screen::GameState;
use bevy::prelude::*;
use bevy::{ecs::resource::Resource, math::IVec2};

#[derive(Resource)]
pub struct Rover {
    pub position: IVec2,
}

#[derive(Event)]
pub struct ActionListExecute {
    pub action_list: Vec<Action>,
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_actions.run_if(in_state(GameState::Game)));
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

fn execute_actions(
    mut commands: Commands,
    mut events: EventReader<ActionListExecute>,
    mut rover_query: Query<Entity, With<RoverEntity>>,
) {
    for event in events.read() {
        for action in event.action_list.iter() {}
    }
}
