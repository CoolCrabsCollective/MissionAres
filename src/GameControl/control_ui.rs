use bevy::color::palettes::css::{DARK_BLUE, DARK_GRAY};
use bevy::prelude::*;
use crate::GameControl::actions::ActionList;
use crate::title_screen::{GameState, TitleScreenUI};

pub struct ControlUiPlugin;

#[derive(Component)]
pub struct ControlUi;

impl Plugin for ControlUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_action_list_ui.run_if(in_state(GameState::Game)));
        // app.add_systems(OnExit(GameState::Game), clean)
    }
}

fn update_action_list_ui(mut commands: Commands,
                         mut events: EventReader<ActionList>,
                         current_ui_elem_query: Query<Entity, With<ControlUi>>,
                         asset_server: Res<AssetServer>) {
    for event in events.read() {
        // Clear old UI rendering
        for ui_element in current_ui_elem_query.iter() {
            commands.entity(ui_element).despawn();
        }

        // Rebuild UI with new actions list
        let mut main_ui_commands = commands.spawn((ControlUi,
                                                   Node {
                                            top: Val::Percent(0.0),
                                            left: Val::Percent(0.0),
                                            width: Val::Percent(15.0),
                                            height: Val::Percent(100.0),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::FlexEnd,
                                            ..default()
                                        },
                                                   BackgroundColor(Color::from(DARK_GRAY)),
        ));

        for (index, action) in event.clone().actions.iter().enumerate() {
            main_ui_commands.with_children(|parent| {
                parent.spawn((
                             Node {
                                 top: Val::Percent(40.0 + 5.0*(index as f32)),
                                 left: Val::Percent(0.0),
                                 width: Val::Percent(100.0),
                                 height: Val::Percent(10.0),
                                 align_items: AlignItems::Center,
                                 justify_content: JustifyContent::Center,
                                 ..default()
                             },
                             BackgroundColor(Color::from(DARK_BLUE)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(action.moves.0.as_str()),
                        TextFont {
                            font: asset_server.load("font.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                        TextShadow::default()));
                });
            });
        }
    }
}