use crate::game_control::actions::{Action, ActionList, ActionType};
use crate::level::GRADVM;
use crate::level_spawner::ActiveLevel;
use crate::rover::ActionListExecute;
use crate::title_screen::GameState;
use crate::ui::interactive_button::InteractiveButton;
use bevy::color::palettes::css::ORANGE;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;

pub struct ControlUIPlugin;

const MAX_COMMANDS: u16 = 12;
const LINE_HEIGHT: f32 = 24.0;

#[derive(Component)]
pub struct ControlUI;

#[derive(Component)]
pub struct ExecuteButton;

#[derive(Resource)]
pub struct RoverColors(pub Vec<Color>);

#[derive(Component)]
pub struct CommandButton(pub ActionType);

#[derive(Component)]
pub struct RobotButton(pub i32);

impl Plugin for ControlUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_action_list_ui.run_if(in_state(GameState::Game)),
                command_button_feedback,
                robot_button_feedback,
            ),
        );
        app.add_systems(Update, execute_button_handler);
        app.add_systems(Update, update_scroll_position);
        app.insert_resource(RoverColors(vec![
            Color::srgba(0.25, 1.0, 0.25, 1.0),
            Color::srgba(0.25, 0.25, 1.0, 1.0),
            Color::srgba(1.0, 1.0, 0.25, 1.0),
        ]));
    }
}

pub const CONTROL_UI_BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
// Red: \(75\div 255\approx 0.2941\)
// Green: \(89\div 255\approx 0.3490\)
// Blue: \(62\div 255\approx 0.2431\)
// Therefore, the result is approximately (0.2941, 0.3490, 0.2431). RGB - Hexadecimal Color ConversionTo calculate hexadecimal colors: Each color will have numerical values for the amounts of Red, Green and Blue that make it up. The...Lycos SearchImage Classification with Convolutional Neural Networks: Introduction to Image DataMay 29, 2024 — By normalizing the RGB values, you ensure compatibility and seamless integration with these tools. The normalisatio...The Carpentries IncubatorELI5: Why do RGB values go from 0 to 255? : r/explainlikeimfiveOct 13, 2021 — RGB color scheme is 8-bit color per channel (R G B) this is known as 16 million colors. Each channel has 8 bit valu...RedditRGB - Hexadecimal Color ConversionTo calculate hexadecimal colors: Each color will have numerical values for the amounts of Red, Green and Blue that make it up. The...Lycos SearchImage Classification with Convolutional Neural Networks: Introduction to Image DataMay 29, 2024 — By normalizing the RGB values, you ensure compatibility and seamless integration with these tools. The normalisatio...The Carpentries IncubatorELI5: Why do RGB values go from 0 to 255? : r/explainlikeimfiveOct 13, 2021 — RGB color scheme is 8-bit color per channel (R G B) this is known as 16 million colors. Each channel has 8 bit valu...RedditShow all   Dive deeper in AI ModeAI responses may include mistakes. Learn morePositive feedbackNegative feedbackThank you
//      Your feedback helps Google improve. See our Privacy Policy.
// Share more feedbackReport a problemClose
pub const CONTROL_UI_SECONDARY_BACKGROUND_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const CONTROL_UI_BORDER_COLOR: Color = Color::srgb(0.26, 0.26, 0.26);
pub const ACTION_SECTIONS_BORDER_COLOR: Color = Color::srgb(0.36, 0.36, 0.36);

fn update_action_list_ui(
    mut commands: Commands,
    mut action_lists: EventReader<ActionList>,
    current_ui_elem_query: Query<Entity, With<ControlUI>>,
    all_rover_colors: Res<RoverColors>,
    asset_server: Res<AssetServer>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
) {
    if action_lists.is_empty() {
        return;
    }

    let gradum = levels.get(
        &match &active_level.0 {
            Some(x) => x,
            None => panic!("No active level"),
        }
        .clone(),
    );

    for event in action_lists.read() {
        let number_of_rovers: usize = gradum.unwrap().NVMERVS_VEHICVLORVM_MOBILIVM as usize;

        let selected_robot_index = event.current_selection;
        for ui_element in current_ui_elem_query.iter() {
            if let Ok(_) = commands.get_entity(ui_element) {
                commands.entity(ui_element).despawn();
            }
        }

        let image_robot = asset_server.load("command_icons/robot.png");

        commands
            .spawn((ControlUI, ui_sidebar_container_node()))
            .with_children(|container_parent| {
                container_parent
                    .spawn((
                        ui_sidebar_node(),
                        BackgroundColor(CONTROL_UI_BACKGROUND_COLOR),
                        BorderColor(CONTROL_UI_BORDER_COLOR),
                        BorderRadius {
                            top_right: Val::Px(8.0),
                            bottom_right: Val::Px(8.0),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        build_control_panel(parent, &asset_server);

                        let rover_colors = &all_rover_colors.0[0..number_of_rovers];
                        // let columns_template = vec![GridTrack::flex(1.0); rover_colors.len()];

                        parent
                            .spawn((
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0, // Take remaining space after other siblings
                                    flex_shrink: 1.0, // Allow shrinking if needed
                                    min_height: Val::Px(0.0), // Important: allows flex item to shrink below content size
                                    margin: UiRect {
                                        right: Val::Px(4.0),
                                        left: Val::Px(4.0),
                                        bottom: Val::Px(8.0),
                                        ..default()
                                    },
                                    padding: UiRect {
                                        top: Val::Px(8.0),
                                        ..default()
                                    },
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(CONTROL_UI_SECONDARY_BACKGROUND_COLOR),
                                BorderColor(ACTION_SECTIONS_BORDER_COLOR),
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn((Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Val::Px(0.0)),
                                        // max_height: Val::Percent(100.0),
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        // let slicer = TextureSlicer {
                                        //     border: Default::default(),
                                        //     center_scale_mode: SliceScaleMode::Stretch,
                                        //     sides_scale_mode: SliceScaleMode::Stretch,
                                        //     max_corner_scale: 1.0,
                                        // };
                                        let robot_node_for_img = Node {
                                            width: Val::Px(40.0),
                                            height: Val::Px(40.0),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        };

                                        for (robot_idx, color) in rover_colors.iter().enumerate() {
                                            let img_robot_node = ImageNode {
                                                image: image_robot.clone(),
                                                // image_mode: NodeImageMode::Sliced(slicer.clone()),
                                                image_mode: NodeImageMode::Auto,
                                                color: *color,
                                                ..default()
                                            };

                                            let robot_bg_color =
                                                if (robot_idx == selected_robot_index) {
                                                    Color::srgb(0.8, 0.8, 0.8)
                                                } else {
                                                    Color::srgba(0.0, 0.0, 0.0, 0.0)
                                                };

                                            parent.spawn((
                                                Button,
                                                RobotButton(robot_idx as i32),
                                                robot_node_for_img.clone(),
                                                img_robot_node.clone(),
                                                BackgroundColor(robot_bg_color),
                                                BorderRadius::all(Val::Px(5.0)),
                                                InteractiveButton::simple_image(
                                                    robot_bg_color,
                                                    *color,
                                                    color.darker(0.1),
                                                    color.lighter(0.1),
                                                    true,
                                                ),
                                            ));
                                        }
                                    });

                                parent
                                    .spawn((Node {
                                        // height: Val::Px(200.0),
                                        // max_height: Val::Px(200.0),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect {
                                            bottom: Val::Px(8.0),
                                            ..default()
                                        },
                                        overflow: Overflow::scroll_y(),
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        for (robot_idx, color) in rover_colors.iter().enumerate() {
                                            let mut multi_robot_command_list = parent.spawn((
                                                multi_robot_command_list(number_of_rovers),
                                                Pickable {
                                                    should_block_lower: false,
                                                    ..default()
                                                },
                                            ));
                                            multi_robot_command_list.with_children(|parent| {
                                                for action in event.actions[robot_idx].iter() {
                                                    ui_command_statement(
                                                        parent,
                                                        action,
                                                        &asset_server,
                                                    );
                                                }
                                            });
                                        }
                                    });
                            });
                        parent
                            .spawn((
                                ExecuteButton,
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(60.0),
                                    min_height: Val::Px(60.0),
                                    //border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    align_self: AlignSelf::FlexEnd,
                                    ..default()
                                },
                                Transform::default(),
                                BackgroundColor::from(Color::srgba(1.0, 0.2, 0.2, 1.0)),
                                InteractiveButton::simple(
                                    Color::srgba(1.0, 0.2, 0.2, 1.0),
                                    Color::srgba(0.9, 0.9, 0.9, 1.0),
                                    true,
                                ),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Execute"),
                                    TextFont {
                                        font: asset_server.load("fonts/SpaceGrotesk-Light.ttf"),
                                        font_size: 26.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                                    TextShadow {
                                        offset: Vec2::splat(2.0),
                                        color: Color::linear_rgba(0., 0., 0., 0.75),
                                    },
                                ));
                            });
                    });
            });
    }
}

fn execute_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExecuteButton>)>,
    mut events: EventWriter<ActionListExecute>,
    action_list: Res<ActionList>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            events.write(ActionListExecute {
                action_list: action_list.actions.clone(),
            });
        }
    }
}

fn ui_sidebar_container_node() -> Node {
    Node {
        height: Val::Percent(100.0),
        width: Val::Px(300.0),
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn ui_sidebar_node() -> Node {
    Node {
        height: Val::Percent(80.0),
        // max_height: Val::Px(500.0),
        width: Val::Percent(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.0)),
        // grid_template_columns: vec![GridTrack::flex(1.0)],
        // grid_template_rows: vec![
        //     GridTrack::flex(2.0),
        //     GridTrack::flex(1.0),
        //     GridTrack::flex(4.0),
        //     GridTrack::flex(1.0),
        // ],
        border: UiRect {
            right: Val::Px(6.0),
            top: Val::Px(6.0),
            bottom: Val::Px(6.0),
            ..default()
        },
        // row_gap: Val::Px(15.0),
        // column_gap: Val::Px(5.0),
        ..default()
    }
}

fn ui_command_statement(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    action: &Action,
    asset_server: &Res<AssetServer>,
) {
    let image_move = asset_server.load(action.moves.0.img_path());
    let slicer = TextureSlicer {
        border: Default::default(),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    let move_node_for_img = Node {
        height: Val::Px(24.0),
        width: Val::Px(24.0),
        aspect_ratio: Some(1.0f32),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        // margin: UiRect::all(Val::Auto),
        ..default()
    };

    let img_move_node = ImageNode {
        image: image_move.clone(),
        // image_mode: NodeImageMode::Sliced(slicer.clone()),
        image_mode: NodeImageMode::Auto,
        ..default()
    };
    parent
        .spawn(Node {
            // min_height: Val::Px(LINE_HEIGHT),
            // max_height: Val::Px(LINE_HEIGHT),
            ..default()
        })
        .insert(Pickable {
            should_block_lower: false,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                move_node_for_img.clone(),
                img_move_node.clone(),
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
            ));
        });
}

fn multi_robot_command_list(num_rovers: usize) -> Node {
    Node {
        width: Val::Px(56.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        // height: Val::Px(200.0),
        // max_height: Val::Px(200.0),
        // overflow: Overflow::scroll_y(),
        row_gap: Val::Px(12.0),
        ..default()
    }
}
fn ui_command_list<'a>(parent: &'a mut RelatedSpawnerCommands<'_, ChildOf>) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0)],
            grid_template_rows: RepeatedGridTrack::flex(MAX_COMMANDS, 1.0),
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(5.0),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(CONTROL_UI_BACKGROUND_COLOR),
    ))
}

fn command_button_feedback(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &mut Transform, &CommandButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut action_list: ResMut<ActionList>,
    mut action_writer: EventWriter<ActionList>,
) {
    for (interaction, mut image, mut trans, command) in &mut interaction_query {
        let action_list_selection = action_list.current_selection;
        match *interaction {
            Interaction::Pressed => {
                image.color = ORANGE.into();

                if action_list.actions.get(action_list_selection).is_some()
                    && action_list.actions[action_list_selection].len() < MAX_COMMANDS as usize
                {
                    action_list.actions[action_list_selection].push(Action {
                        moves: (command.0.clone(), action_list_selection),
                    });
                    action_writer.write(action_list.clone());
                }
                trans.scale = Vec3::new(0.9, 0.9, 0.9);
            }
            Interaction::Hovered => {
                image.color = ORANGE.into();
                trans.scale = Vec3::new(1.1, 1.1, 1.1);
            }
            Interaction::None => {
                image.color = Color::WHITE;
                trans.scale = Vec3::new(1.0, 1.0, 1.0);
            }
        }
    }
}

fn robot_button_feedback(
    mut interaction_query: Query<
        (&Interaction, &RobotButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut action_list: ResMut<ActionList>,
    mut action_writer: EventWriter<ActionList>,
) {
    let mut has_to_update: bool = false;
    for (interaction, robot_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                action_list.current_selection = robot_button.0 as usize;
                has_to_update = true;
                //node.
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }

    if has_to_update {
        action_writer.write(action_list.clone());
    }
}

fn build_control_panel(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    asset_server: &Res<AssetServer>,
) {
    let image_move_up = asset_server.load("command_icons/arrow_up_outlined.png");
    let image_move_right = asset_server.load("command_icons/arrow_right_outlined.png");
    let image_wait = asset_server.load("command_icons/clock_outlined.png");
    let slicer = TextureSlicer {
        border: Default::default(),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    parent
        .spawn((Node {
            height: Val::Px(160.0),
            min_height: Val::Px(160.0),
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
            margin: UiRect {
                bottom: Val::Px(16.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((Node::default()));
            parent
                .spawn((Node {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0f32),
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                    row_gap: Val::Px(15.0),
                    column_gap: Val::Px(15.0),
                    ..default()
                },))
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

                    parent.spawn((Node::default()));
                    parent.spawn((
                        Button,
                        CommandButton(ActionType::MoveUp),
                        node_for_img.clone(),
                        img_up.clone(),
                        Transform::default(),
                    ));
                    parent.spawn((Node::default()));
                    parent.spawn((
                        Button,
                        CommandButton(ActionType::MoveLeft),
                        node_for_img.clone(),
                        img_left.clone(),
                        Transform::default(),
                    ));
                    parent.spawn((
                        Button,
                        CommandButton(ActionType::Wait),
                        node_for_img.clone(),
                        img_wait.clone(),
                        Transform::default(),
                    ));
                    parent.spawn((
                        Button,
                        CommandButton(ActionType::MoveRight),
                        node_for_img.clone(),
                        img_right.clone(),
                        Transform::default(),
                    ));
                    parent.spawn((Node::default()));
                    parent.spawn((
                        CommandButton(ActionType::MoveDown),
                        Button,
                        node_for_img.clone(),
                        img_down.clone(),
                        Transform::default(),
                    ));
                    parent.spawn((Node::default()));
                });

            parent.spawn((Node::default()));
        });
}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
