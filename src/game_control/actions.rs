use bevy::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Wait,
}

pub struct ActionController;

#[derive(Clone, Debug)]
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
            ActionType::MoveUp => "command_icons/arrow_up_outlined.png",
            ActionType::MoveDown => "command_icons/arrow_down_outlined.png",
            ActionType::MoveLeft => "command_icons/arrow_left_outlined.png",
            ActionType::MoveRight => "command_icons/arrow_right_outlined.png",
            ActionType::Wait => "command_icons/clock_outlined.png",
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
