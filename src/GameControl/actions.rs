use crate::title_screen::GameState;
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
    pub actions: Vec<Action>,
}

// TODO: instead of putting strings we should list icons
impl ActionType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ActionType::MoveUp => "UP",
            ActionType::MoveDown => "DOWN",
            ActionType::MoveLeft => "LEFT",
            ActionType::MoveRight => "RIGHT",
            ActionType::Wait => "WAIT",
        }
    }
}

impl Robot {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Robot::ROVER1 => "R1",
            Robot::ROVER2 => "R2",
            Robot::DRONE1 => "D1",
            Robot::DRONE2 => "D2",
        }
    }
}

impl Plugin for ActionController {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_actions);
        app.add_event::<ActionList>();
        app.insert_resource(ActionList { actions: vec![] });
    }
}

fn setup_actions(mut commands: Commands, mut action_list: ResMut<ActionList>) {
    // Temp insert actions immediately
    action_list.actions.push(Action {
        moves: (ActionType::MoveUp, Robot::ROVER1),
    });
    action_list.actions.push(Action {
        moves: (ActionType::MoveUp, Robot::ROVER1),
    });
    action_list.actions.push(Action {
        moves: (ActionType::MoveRight, Robot::ROVER1),
    });

    let action_event = action_list.clone();
    commands.send_event(action_event);
}
