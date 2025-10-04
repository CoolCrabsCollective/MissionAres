use std::collections::HashMap;
use bevy::prelude::*;
use crate::title_screen::GameState;

#[derive(Clone)]
enum ROBOT
{
    ROVER1,
    ROVER2,
    DRONE1,
    DRONE2
}

#[derive(Clone)]
enum ActionType
{
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub struct ActionController;

#[derive(Clone)]
struct Action
{
    moves: HashMap<ActionType, ROBOT>,
}

#[derive(Resource, Event, Clone)]
pub struct ActionList {
    actions: Vec<Action>,
}

impl Plugin for ActionController {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_actions);
        app.add_event::<ActionList>();
        app.insert_resource(ActionList { actions: vec![] });
    }
}

fn setup_actions(mut commands: Commands, action_list: Res<ActionList>) {
    let action_event = action_list.clone();
    commands.send_event(action_event);
}
