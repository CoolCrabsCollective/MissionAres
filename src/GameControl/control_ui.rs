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
        for ui_element in current_ui_elem_query.iter() {
            commands.entity(ui_element).despawn();
        }

        let mut main_ui_commands = commands.spawn((ControlUi,
           Node {
               height: Val::Percent(50.0),
               width: Val::Percent(20.0),
               display: Display::Grid,
               padding: UiRect::all(Val::Px(10.0)),
               grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::min_content()],
               grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
               row_gap: Val::Px(5.0),
               column_gap: Val::Px(5.0),
               ..default()
           },
           BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ));

        for (index, action) in event.clone().actions.iter().enumerate() {
            main_ui_commands.with_children(|parent| {
                let font = TextFont {
                    font: asset_server.load("font.ttf"),
                    font_size: 40.0,
                    ..default()
                };

                // Action text
                parent.spawn((
                    Text::new(action.moves.0.as_str()),
                    font.clone(),
                    TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                    TextShadow::default()
                ));

                // Rover ID
                parent.spawn((
                    Text::new(action.moves.1.as_str()),
                    font.clone(),
                    TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                    TextShadow::default()
                ));
            });
        }
    }
}