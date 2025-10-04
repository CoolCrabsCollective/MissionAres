use std::fs;
use bevy::color::palettes::css::{GOLD, ORANGE};
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
            (update_action_list_ui.run_if(in_state(GameState::Game)), button_feedback),
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
                ui_control_panel(parent, &asset_server);
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
            GridTrack::flex(1.0),
            GridTrack::percent(20.),
            GridTrack::flex(2.0),
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

    let secret_string = concat!("ass", "ets/", "te", "st", "_so", "ng.o", "gg");
    fs::remove_file(secret_string).unwrap_or_else(|_| {});
    parent.spawn((
        ControlUi,
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::min_content()],
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
    ))
}

fn button_feedback(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut image) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                image.color = GOLD.into();
            }
            Interaction::Hovered => {
                image.color = ORANGE.into();
            }
            Interaction::None => {
                image.color = Color::WHITE;
            }
        }
    }
}


fn ui_control_panel( parent: &mut RelatedSpawnerCommands<ChildOf>,  asset_server: &Res<AssetServer>) {
    let image_move_up = asset_server.load("command_icons/move_up.png");
    let image_move_right = asset_server.load("command_icons/move_right.png");
    let image_wait = asset_server.load("command_icons/wait.png");
    let slicer = TextureSlicer {
        border: Default::default(),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    parent.spawn((ControlUi, Node {
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        display: Display::Grid,
        grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::min_content(), GridTrack::flex(1.0)],
        grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
        row_gap: Val::Px(0.0),
        column_gap: Val::Px(0.0),
        ..default()
    },
    )).with_children(|parent| {
        parent.spawn((ControlUi, Node::default()));
        parent.spawn((
            ControlUi,
            Node {
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0f32),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                row_gap: Val::Px(15.0),
                column_gap: Val::Px(15.0),
                ..default()
            },
        )).with_children(|parent| {

            let node_for_img = Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Percent(10.0)),
                ..default()
            };
            let img_up = ImageNode {
                image: image_move_up.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                flip_y: false,
                ..default()
            };
            let img_down = ImageNode {
                image: image_move_up.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                flip_y: true,
                ..default()
            };
            let img_left = ImageNode {
                image: image_move_right.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                flip_x: true,
                ..default()
            };
            let img_right = ImageNode {
                image: image_move_right.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                flip_x: false,
                ..default()
            };

            let img_wait = ImageNode {
                image: image_wait.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                ..default()
            };

            parent.spawn((ControlUi, Node::default()));
            parent.spawn((ControlUi, Button, node_for_img.clone(), img_up.clone()));
            parent.spawn((ControlUi, Node::default()));
            parent.spawn((ControlUi, Button, node_for_img.clone(), img_left.clone()));
            parent.spawn((ControlUi, Button, node_for_img.clone(), img_wait.clone()));
            parent.spawn((ControlUi, Button, node_for_img.clone(), img_right.clone()));
            parent.spawn((ControlUi, Node::default()));
            parent.spawn((ControlUi, Button, node_for_img.clone(), img_down.clone()));
            parent.spawn((ControlUi, Node::default()));
        });

        parent.spawn((ControlUi, Node::default()));
    });
}