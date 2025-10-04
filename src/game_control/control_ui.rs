use crate::game_control::actions::{Action, ActionList};
use crate::title_screen::GameState;
use bevy::color::palettes::css::{GOLD, ORANGE};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub struct ControlUiPlugin;

const MAX_COMMANDS: u16 = 8;

#[derive(Component)]
pub struct ControlUi;

#[derive(Resource)]
pub struct RoverColors(Vec<Color>);

impl Plugin for ControlUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_action_list_ui.run_if(in_state(GameState::Game)),
                button_feedback,
            ),
        );
        app.insert_resource(RoverColors(vec![
            Color::srgba(1.0, 0.0, 0.0, 1.0),
            Color::srgba(0.0, 0.0, 1.0, 1.0),
            Color::srgba(0.0, 1.0, 0.0, 1.0),
        ]));
        // app.add_systems(OnExit(GameState::Game), clean)
    }
}

fn update_action_list_ui(
    mut commands: Commands,
    mut action_lists: EventReader<ActionList>,
    current_ui_elem_query: Query<Entity, With<ControlUi>>,
    all_rover_colors: Res<RoverColors>,
    asset_server: Res<AssetServer>,
) {
    let font = TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 40.0,
        ..default()
    };
    for event in action_lists.read() {
        let number_of_rovers = event.actions.len();
        println!("Num rovers: {}", number_of_rovers);
        for ui_element in current_ui_elem_query.iter() {
            if let Ok(_) = commands.get_entity(ui_element) {
                commands.entity(ui_element).despawn();
            }
        }

        let image_robot = asset_server.load("command_icons/robot.png");
        let side_bar = commands
            .spawn((
                ControlUi,
                ui_sidebar_node(),
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|parent| {
                ui_control_panel(parent, &asset_server);

                let rover_colors = &all_rover_colors.0[0..number_of_rovers];
                let columns_template = vec![GridTrack::flex(1.0); rover_colors.len()];
                parent
                    .spawn((
                        ControlUi,
                        Node {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            display: Display::Grid,
                            padding: UiRect::all(Val::Px(10.0)),
                            grid_template_columns: columns_template,
                            grid_template_rows: vec![GridTrack::flex(1.0)],
                            row_gap: Val::Px(0.0),
                            column_gap: Val::Px(5.0),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        let slicer = TextureSlicer {
                            border: Default::default(),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        };
                        let robot_node_for_img = Node {
                            width: Val::Px(96.0),
                            height: Val::Px(96.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        };

                        for color in rover_colors {
                            let img_robot_node = ImageNode {
                                image: image_robot.clone(),
                                image_mode: NodeImageMode::Sliced(slicer.clone()),
                                color: *color,
                                ..default()
                            };
                            parent.spawn((
                                ControlUi,
                                robot_node_for_img.clone(),
                                img_robot_node.clone(),
                            ));
                        }
                    });

                let mut multi_robot_command_list =
                    parent.spawn((ControlUi, multi_robot_command_list(number_of_rovers)));
                multi_robot_command_list.with_children(|parent| {
                    for i in 0..number_of_rovers {
                        let mut ui_commands = ui_command_list(parent);
                        for action in event.clone().actions[i].iter() {
                            ui_commands.with_children(|parent| {
                                ui_command_statement(parent, action, &font);
                            });
                        }
                    }
                });
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
            GridTrack::flex(2.0),
            GridTrack::flex(1.0),
            GridTrack::flex(4.0),
        ],
        row_gap: Val::Px(15.0),
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
}

fn multi_robot_command_list(num_rovers: usize) -> Node {
    Node {
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        display: Display::Grid,
        grid_template_columns: vec![GridTrack::flex(1.0); num_rovers],
        grid_template_rows: GridTrack::flex(1.0),
        row_gap: Val::Px(0.0),
        column_gap: Val::Px(0.0),
        ..default()
    }
}
fn ui_command_list<'a>(parent: &'a mut RelatedSpawnerCommands<'_, ChildOf>) -> EntityCommands<'a> {
    parent.spawn((
        ControlUi,
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0)],
            grid_template_rows: RepeatedGridTrack::flex(MAX_COMMANDS, 1.0),
            row_gap: Val::Px(0.0),
            column_gap: Val::Px(5.0),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
    ))
}

fn button_feedback(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >,
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

fn ui_control_panel(parent: &mut RelatedSpawnerCommands<ChildOf>, asset_server: &Res<AssetServer>) {
    let image_move_up = asset_server.load("command_icons/move_up.png");
    let image_move_right = asset_server.load("command_icons/move_right.png");
    let image_wait = asset_server.load("command_icons/wait.png");
    let slicer = TextureSlicer {
        border: Default::default(),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    parent
        .spawn((
            ControlUi,
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::flex(1.0),
                    GridTrack::min_content(),
                    GridTrack::flex(1.0),
                ],
                grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                row_gap: Val::Px(0.0),
                column_gap: Val::Px(0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((ControlUi, Node::default()));
            parent
                .spawn((
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
                ))
                .with_children(|parent| {
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
