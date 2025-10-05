use crate::rover::ActionListExecute;
use bevy::prelude::*;

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
    pub moves: (ActionType, usize),
}

#[derive(Resource, Event, Clone)]
pub struct ActionList {
    pub actions: Vec<Vec<Action>>,
    pub current_selection: usize,
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

impl Plugin for ActionController {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionList>();
        app.insert_resource(ActionList {
            actions: vec![vec![]],
            current_selection: 0,
        });
    }
}

fn execute(mut commands: Commands, action_list: ResMut<ActionList>) {
    let execute_event = ActionListExecute {
        action_list: action_list.actions.clone(),
    };
    commands.send_event(execute_event);
}
