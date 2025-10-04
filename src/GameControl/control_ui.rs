use crate::GameControl::actions::{Action, ActionList};
use crate::title_screen::GameState;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub struct ControlUiPlugin;

#[derive(Component)]
pub struct ControlUi;

impl Plugin for ControlUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_action_list_ui.run_if(in_state(GameState::Game)),
        );
        // app.add_systems(OnExit(GameState::Game), clean)
    }
}

fn update_action_list_ui(
    mut commands: Commands,
    mut events: EventReader<ActionList>,
    current_ui_elem_query: Query<Entity, With<ControlUi>>,
    asset_server: Res<AssetServer>,
) {
    let font = TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 40.0,
        ..default()
    };

    for event in events.read() {
        for ui_element in current_ui_elem_query.iter() {
            commands.entity(ui_element).despawn();
        }

        let side_bar = commands
            .spawn((
                ControlUi,
                ui_sidebar_node(),
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Controls"),
                    font.clone(),
                    TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                    TextShadow::default(),
                ));
                parent.spawn((
                    Text::new("Current move"),
                    font.clone(),
                    TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                    TextShadow::default(),
                ));
                let mut ui_commands = ui_command_list(parent);

                for action in event.clone().actions.iter() {
                    ui_commands.with_children(|parent| {
                        ui_command_statement(parent, action, &font);
                    });
                }
            });
    }
}

fn ui_sidebar_node() -> Node {
    Node {
        height: Val::Percent(100.0),
        width: Val::Percent(20.0),
        display: Display::Grid,
        padding: UiRect::all(Val::Px(10.0)),
        grid_template_columns: vec![GridTrack::flex(1.0)],
        grid_template_rows: vec![
            GridTrack::percent(20.),
            GridTrack::percent(20.),
            GridTrack::flex(1.0),
        ],
        row_gap: Val::Px(5.0),
        column_gap: Val::Px(5.0),
        ..default()
    }
}

fn ui_command_statement(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    action: &Action,
    font_node: &TextFont,
) {
    // Action text
    parent.spawn((
        Text::new(action.moves.0.as_str()),
        font_node.clone(),
        TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
        TextShadow::default(),
    ));

    // Rover ID
    parent.spawn((
        Text::new(action.moves.1.as_str()),
        font_node.clone(),
        TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
        TextShadow::default(),
    ));
}

fn ui_command_list<'a>(parent: &'a mut RelatedSpawnerCommands<'_, ChildOf>) -> EntityCommands<'a> {
    parent.spawn((
        ControlUi,
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            display: Display::Grid,
            padding: UiRect::all(Val::Px(10.0)),
            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::min_content()],
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
    ))
}
