use std::collections::HashMap;
use bevy::prelude::*;

enum ROBOT
{
    ROVER1,
    ROVER2,
    DRONE1,
    DRONE2
}

enum ActionType
{
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub struct ActionController;

struct Action
{
    moves: HashMap<ActionType, ROBOT>,
}

#[derive(Resource, Event)]
pub struct ActionList {
    actions: Vec<Action>,
}

impl Plugin for ActionController {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionList>();
        app.insert_resource(ActionList { actions: vec![] });
    }
}
