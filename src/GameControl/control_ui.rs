use bevy::prelude::*;
use crate::GameControl::actions::ActionList;
use crate::title_screen::{GameState, TitleScreenUI};

pub struct ControlUiPlugin;

#[derive(Component)]
pub struct ControlUi;

impl Plugin for ControlUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_action_list_iu.run_if(in_state(GameState::Game)));
        // app.add_systems(OnExit(GameState::Game), clean)
    }
}

fn update_action_list_iu(mut commands: Commands,
                         mut events: EventReader<ActionList>,
                         current_ui_elem_query: Query<Entity, With<TitleScreenUI>>,) {
    for event in events.read() {
        // Clear old UI rendering
        for ui_element in current_ui_elem_query.iter() {
            commands.entity(ui_element).despawn();
        }

        // Rebuild UI with new actions list
        commands.spawn((ControlUi,
                                        Node {
                                            top: Val::Percent(0.0),
                                            left: Val::Percent(0.0),
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
        ))
            .with_children(|parent| {

            });
    }
}