use crate::rover::ActionListExecute;
use bevy::prelude::*;

#[derive(Clone)]
pub enum Robot {
    ROVER1,
    ROVER2,
    DRONE1,
    DRONE2,
}

#[derive(Clone)]
pub enum ActionType {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Wait,
}

pub struct ActionController;

#[derive(Clone)]
pub struct Action {
    pub moves: (ActionType, Robot),
}

#[derive(Resource, Event, Clone)]
pub struct ActionList {
    pub actions: Vec<Vec<Action>>,
}

impl ActionType {
    pub(crate) fn img_path(&self) -> &'static str {
        match self {
            ActionType::MoveUp => "command_icons/up.png",
            ActionType::MoveDown => "command_icons/down.png",
            ActionType::MoveLeft => "command_icons/left.png",
            ActionType::MoveRight => "command_icons/right.png",
            ActionType::Wait => "command_icons/wait.png",
        }
    }
}

impl Robot {}

impl Plugin for ActionController {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionList>();
        app.insert_resource(ActionList {
            actions: vec![vec![]],
        });
    }
}

fn execute(mut commands: Commands, action_list: ResMut<ActionList>) {
    let execute_event = ActionListExecute {
        action_list: action_list.actions.clone(),
    };
    commands.send_event(execute_event);
}
