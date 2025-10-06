use crate::game_control::actions::{Action, ActionList, ActionType};
use crate::help::help::{HelpButton, HelpDialog, show_help_for_empty_actions};
use crate::level::GRADVM;
use crate::level_spawner::{ActiveLevel, LevelElement};
use crate::mesh_loader::DebugLogEntityRequest;
use crate::rover::{ActionListExecute, RoverEntity};
use crate::title_screen::GameState;
use crate::ui::Px_dynamic;
use crate::ui::interactive_button::InteractiveButton;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::pbr::SpotLight;
use bevy::picking::events::{Click, Pointer};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;

pub struct ControlUIPlugin;

const MAX_COMMANDS: u16 = 12000;
const LINE_HEIGHT: f32 = 24.0;

#[derive(Component)]
pub struct ControlUI;

#[derive(Component)]
pub struct ExecuteButton;

#[derive(Resource)]
pub struct RoverColors(pub Vec<Color>);

#[derive(Resource)]
pub struct UIRoverColors(pub Vec<Color>);

#[derive(Component)]
pub struct CommandButton(pub ActionType);

#[derive(Component)]
pub struct ActionDeleteButton {
    rover_index: usize,
    action_index: usize,
}

#[derive(Component)]
pub struct ClearAllButton;

#[derive(Component)]
pub struct RobotButton(pub i32);

#[derive(Component)]
pub struct SelectionLight;

impl Plugin for ControlUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rebuild_control_ui.run_if(not(in_state(GameState::TitleScreen))),
                command_button_handler.run_if(in_state(GameState::Programming)),
                robot_button_handler.run_if(in_state(GameState::Programming)),
                delete_action_handler.run_if(in_state(GameState::Programming)),
                clear_all_handler.run_if(in_state(GameState::Programming)),
            ),
        );
        app.add_systems(Update, execute_handler);
        app.add_systems(Update, update_scroll_position);
        app.add_systems(Update, spawn_selection_light);
        app.add_systems(Update, update_selection_light);

        app.insert_resource(UIRoverColors(vec![
            Color::srgba(0.9, 0.9, 0.9, 1.0),
            Color::srgba(0.2, 0.65, 0.2, 1.0),   // green
            Color::srgba(0.2, 0.4, 0.75, 1.0),   // blue
            Color::srgba(0.8, 0.75, 0.2, 1.0),   // yellow
            Color::srgba(0.65, 0.25, 0.65, 1.0), // purple
            Color::srgba(0.8, 0.2, 0.2, 1.0),    // red
        ]));

        app.insert_resource(RoverColors(vec![
            Color::srgba(1.0, 1.0, 1.0, 1.0),
            Color::srgba(51.0 / 255.0, 166.0 / 255.0, 51.0 / 255.0, 0.35), // green
            Color::srgba(51.0 / 255.0, 102.0 / 255.0, 191.0 / 255.0, 0.35), // blue
            Color::srgba(204.0 / 255.0, 191.0 / 255.0, 51.0 / 255.0, 0.35), // yellow
            Color::srgba(166.0 / 255.0, 64.0 / 255.0, 166.0 / 255.0, 0.35), // purple
            Color::srgba(204.0 / 255.0, 51.0 / 255.0, 51.0 / 255.0, 0.35), // red
        ]));
    }
}

pub const CONTROL_UI_BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const CONTROL_UI_SECONDARY_BACKGROUND_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const CONTROL_UI_BORDER_COLOR: Color = Color::srgb(0.26, 0.26, 0.26);
pub const ACTION_SECTIONS_BORDER_COLOR: Color = Color::srgb(0.36, 0.36, 0.36);

// builders

fn rebuild_control_ui(
    mut commands: Commands,
    mut action_lists: EventReader<ActionList>,
    current_ui_elem_query: Query<Entity, With<ControlUI>>,
    all_rover_colors: Res<UIRoverColors>,
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
                            top_right: Px_dynamic(8.0),
                            bottom_right: Px_dynamic(8.0),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        build_control_panel(parent, &asset_server);

                        let rover_colors = &all_rover_colors.0[0..number_of_rovers];

                        parent
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0, // Take remaining space after other siblings
                                    flex_shrink: 1.0, // Allow shrinking if needed
                                    min_height: Px_dynamic(0.0), // Important: allows flex item to shrink below content size
                                    margin: UiRect {
                                        bottom: Px_dynamic(14.0),
                                        ..default()
                                    },
                                    padding: UiRect {
                                        top: Px_dynamic(8.0),
                                        ..default()
                                    },
                                    border: UiRect::all(Px_dynamic(2.0)),
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                                BackgroundColor(CONTROL_UI_SECONDARY_BACKGROUND_COLOR),
                                BorderColor(ACTION_SECTIONS_BORDER_COLOR),
                            ))
                            .with_children(|parent| {
                                // Clear all button - absolutely positioned in bottom right
                                // Only show if there are any commands
                                let has_commands =
                                    event.actions.iter().any(|actions| !actions.is_empty());

                                if has_commands {
                                    let clear_icon = asset_server.load("fail_particle.png");
                                    parent
                                        .spawn((
                                            Button,
                                            ClearAllButton,
                                            Node {
                                                position_type: PositionType::Absolute,
                                                bottom: Px_dynamic(6.0),
                                                right: Px_dynamic(6.0),
                                                width: Px_dynamic(40.0),
                                                height: Px_dynamic(40.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            InteractiveButton::simple(
                                                Color::srgba(0.8, 0.2, 0.2, 0.0),
                                                Color::srgba(1.0, 0.3, 0.3, 0.0),
                                                true,
                                            ),
                                            ZIndex(10),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                ImageNode {
                                                    image: clear_icon.clone(),
                                                    image_mode: NodeImageMode::Auto,
                                                    ..default()
                                                },
                                                Node {
                                                    width: Px_dynamic(26.0),
                                                    height: Px_dynamic(26.0),
                                                    ..default()
                                                },
                                            ));
                                        });
                                }

                                parent
                                    .spawn((Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Px_dynamic(0.0)),
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        for (robot_idx, color) in rover_colors.iter().enumerate() {
                                            let transparent = Color::srgba(0.0, 0.0, 0.0, 0.0);
                                            let is_selected = robot_idx == selected_robot_index;

                                            parent
                                                .spawn((Node {
                                                    display: Display::Flex,
                                                    flex_direction: FlexDirection::Column,
                                                    align_items: AlignItems::Center,
                                                    row_gap: Px_dynamic(4.0),
                                                    margin: UiRect::all(Px_dynamic(5.0)),
                                                    flex_grow: 1.0,
                                                    ..default()
                                                },))
                                                .with_children(|parent| {
                                                    parent
                                                        .spawn((
                                                            Button,
                                                            RobotButton(robot_idx as i32),
                                                            Node {
                                                                width: Px_dynamic(50.0),
                                                                height: Px_dynamic(50.0),
                                                                justify_content:
                                                                    JustifyContent::Center,
                                                                align_items: AlignItems::Center,
                                                                ..default()
                                                            },
                                                            BorderColor::from(transparent),
                                                            BorderRadius::all(Px_dynamic(25.0)),
                                                            InteractiveButton {
                                                                regular_background_color:
                                                                    transparent,
                                                                hover_background_color: transparent,
                                                                pressed_background_color:
                                                                    transparent,
                                                                regular_border_color: transparent,
                                                                hover_border_color: transparent,
                                                                pressed_border_color: transparent,
                                                                regular_image_color: *color,
                                                                hover_image_color: color
                                                                    .darker(0.1),
                                                                pressed_image_color: color
                                                                    .darker(0.2),
                                                                ..default()
                                                            },
                                                        ))
                                                        .with_children(|parent| {
                                                            parent.spawn((
                                                                ImageNode {
                                                                    image: image_robot.clone(),
                                                                    image_mode: NodeImageMode::Auto,
                                                                    color: *color,
                                                                    ..default()
                                                                },
                                                                Node {
                                                                    width: Val::Percent(100.0),
                                                                    height: Val::Percent(100.0),
                                                                    ..default()
                                                                },
                                                            ));
                                                        });

                                                    // Small circle indicator
                                                    if number_of_rovers > 1 {
                                                        parent.spawn((
                                                            Node {
                                                                width: Val::Px(8.0),
                                                                height: Val::Px(8.0),
                                                                border: UiRect::all(Val::Px(1.0)),
                                                                ..default()
                                                            },
                                                            BackgroundColor(if is_selected {
                                                                Color::srgb(0.83, 0.83, 0.83)
                                                            } else {
                                                                transparent
                                                            }),
                                                            BorderColor(Color::srgb(
                                                                0.83, 0.83, 0.83,
                                                            )),
                                                            BorderRadius::all(Val::Px(4.0)),
                                                        ));
                                                    }
                                                });
                                        }
                                    });

                                parent
                                    .spawn((Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect {
                                            bottom: Px_dynamic(8.0),
                                            ..default()
                                        },
                                        overflow: Overflow::scroll_y(),
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        for (robot_idx, _) in rover_colors.iter().enumerate() {
                                            let mut multi_robot_command_list = parent.spawn((
                                                multi_robot_command_list(),
                                                Pickable {
                                                    should_block_lower: false,
                                                    ..default()
                                                },
                                            ));
                                            multi_robot_command_list.with_children(|parent| {
                                                let mut i = 0;
                                                let mut prev_action = None;
                                                let mut current_count = 0;
                                                let Some(robot_actions) =
                                                    event.actions.get(robot_idx)
                                                else {
                                                    return;
                                                };
                                                for action in robot_actions.iter() {
                                                    if Some(action.moves.0) == prev_action {
                                                        current_count += 1;
                                                    } else {
                                                        if prev_action.is_some() {
                                                            build_deleteable_action_button(
                                                                parent,
                                                                robot_idx,
                                                                i - 1,
                                                                current_count,
                                                                prev_action.unwrap(),
                                                                &asset_server,
                                                            );
                                                        }
                                                        current_count = 1;
                                                    }

                                                    prev_action = Some(action.moves.0);
                                                    i += 1;
                                                }
                                                if prev_action.is_some() {
                                                    build_deleteable_action_button(
                                                        parent,
                                                        robot_idx,
                                                        i - 1,
                                                        current_count,
                                                        prev_action.unwrap(),
                                                        &asset_server,
                                                    );
                                                }
                                            });
                                        }
                                    });
                            });
                        build_execute_button(parent, &asset_server);
                    });
            });
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

    // Add "Add Rover Actions" text
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            margin: UiRect {
                bottom: Px_dynamic(8.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Plan Rover Commands"),
                TextFont {
                    font: asset_server.load("fonts/SpaceGrotesk-Medium.ttf"),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.83, 0.83, 0.83)),
            ));
        });

    parent
        .spawn((Node {
            // height: Px_dynamic(160.0),
            // min_height: Px_dynamic(160.0),
            width: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Px_dynamic(20.0),
            margin: UiRect {
                bottom: Px_dynamic(16.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            // Arrow buttons flexbox with three columns
            parent
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Px_dynamic(4.0),
                    ..default()
                },))
                .with_children(|parent| {
                    let node_for_img = Node {
                        width: Px_dynamic(45.0),
                        height: Px_dynamic(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
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

                    // Left column: left arrow
                    parent
                        .spawn((Node {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                CommandButton(ActionType::MoveLeft),
                                node_for_img.clone(),
                                img_left.clone(),
                                Transform::default(),
                            ));
                        });

                    // Middle column: up and down arrows
                    parent
                        .spawn((Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: Px_dynamic(12.0),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                CommandButton(ActionType::MoveUp),
                                node_for_img.clone(),
                                img_up.clone(),
                                Transform::default(),
                            ));
                            parent.spawn((
                                Button,
                                CommandButton(ActionType::MoveDown),
                                node_for_img.clone(),
                                img_down.clone(),
                                Transform::default(),
                            ));
                        });

                    // Right column: right arrow
                    parent
                        .spawn((Node {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                CommandButton(ActionType::MoveRight),
                                node_for_img.clone(),
                                img_right.clone(),
                                Transform::default(),
                            ));
                        });
                });

            // Wait button with text
            parent
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Px_dynamic(4.0),
                    ..default()
                },))
                .with_children(|parent| {
                    let img_wait = ImageNode {
                        image: image_wait.clone(),
                        image_mode: NodeImageMode::Sliced(slicer.clone()),
                        ..default()
                    };
                    let node_for_img = Node {
                        width: Px_dynamic(40.0),
                        height: Px_dynamic(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Px_dynamic(5.0)),
                        ..default()
                    };

                    parent.spawn((
                        Button,
                        CommandButton(ActionType::Wait),
                        node_for_img.clone(),
                        img_wait.clone(),
                        Transform::default(),
                    ));

                    parent.spawn((
                        Text::new("Wait"),
                        TextFont {
                            font: asset_server.load("fonts/SpaceGrotesk-Light.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.83, 0.83, 0.83)),
                    ));
                });
        });
}

fn ui_sidebar_container_node() -> Node {
    Node {
        height: Val::Percent(100.0),
        width: Px_dynamic(300.0),
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn ui_sidebar_node() -> Node {
    Node {
        height: Val::Percent(80.0),
        width: Val::Percent(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Px_dynamic(14.0)),
        border: UiRect {
            right: Px_dynamic(6.0),
            top: Px_dynamic(6.0),
            bottom: Px_dynamic(6.0),
            ..default()
        },
        ..default()
    }
}

fn build_deleteable_action_button(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    rover_index: usize,
    action_index: usize,
    action_count: usize,
    action: ActionType,
    asset_server: &Res<AssetServer>,
) {
    let image_move = asset_server.load(action.img_path());
    let move_node_for_img = Node {
        height: Px_dynamic(24.0),
        width: Px_dynamic(24.0),
        aspect_ratio: Some(1.0f32),
        margin: UiRect::left(Px_dynamic(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let img_move_node = ImageNode {
        image: image_move.clone(),
        image_mode: NodeImageMode::Auto,
        ..default()
    };
    parent
        .spawn(Node { ..default() })
        .insert(Pickable {
            should_block_lower: false,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(24.0),
                    ..default()
                },
                Text::from(action_count.to_string() + "x"),
                TextFont {
                    font: asset_server.load("fonts/SpaceGrotesk-Light.ttf"),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(if action_count == 1 {
                    Color::srgba(0.0, 0.0, 0.0, 0.0)
                } else {
                    Color::srgba(0.9, 0.9, 0.9, 1.0)
                }),
                TextShadow {
                    offset: Vec2::splat(2.0),
                    color: if action_count == 1 {
                        Color::srgba(0.0, 0.0, 0.0, 0.0)
                    } else {
                        Color::linear_rgba(0., 0., 0., 0.75)
                    },
                },
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
            ));
            parent.spawn((
                Button,
                ActionDeleteButton {
                    rover_index,
                    action_index,
                },
                InteractiveButton::simple_image(
                    Color::srgba(0.0, 0.0, 0.0, 0.0),
                    Color::WHITE,
                    Color::srgba(1.0, 0.25, 0.25, 1.0),
                    Color::srgba(1.0, 0.25, 0.25, 1.0),
                    true,
                ),
                move_node_for_img.clone(),
                img_move_node.clone(),
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
            ));
        });
}

fn multi_robot_command_list() -> Node {
    Node {
        width: Px_dynamic(56.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Px_dynamic(12.0),
        padding: UiRect {
            top: Px_dynamic(8.0),
            bottom: Px_dynamic(4.0),
            ..default()
        },
        flex_grow: 1.0,
        ..default()
    }
}

fn build_execute_button(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
            ExecuteButton,
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Px_dynamic(60.0),
                min_height: Px_dynamic(60.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            Transform::default(),
            BackgroundColor::from(Color::srgba(0.7, 0.15, 0.15, 1.0)),
            InteractiveButton::simple(
                Color::srgba(0.7, 0.15, 0.15, 1.0),
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
                // TextShadow {
                //     offset: Vec2::splat(2.0),
                //     color: Color::linear_rgba(0., 0., 0., 0.75),
                // },
            ));
        });
}

// handlers

fn execute_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExecuteButton>)>,
    mut events: EventWriter<ActionListExecute>,
    mut next_state: ResMut<NextState<GameState>>,
    action_list: Res<ActionList>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut help_button_query: Query<&mut HelpButton>,
    dialog_query: Query<Entity, With<HelpDialog>>,
) {
    let has_no_actions = action_list.actions.iter().all(|v| v.is_empty());

    let mut should_execute = false;
    let mut tried_to_execute = false;

    // Check button press
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            tried_to_execute = true;
            if !has_no_actions {
                should_execute = true;
            }
        }
    }

    // Check space key press (only in Programming state)
    if *game_state.get() == GameState::Programming && keyboard_input.just_pressed(KeyCode::Space) {
        tried_to_execute = true;
        if !has_no_actions {
            should_execute = true;
        }
    }

    if tried_to_execute && has_no_actions {
        show_help_for_empty_actions(
            &mut commands,
            &asset_server,
            &mut help_button_query,
            &dialog_query,
        );
        return;
    }

    if should_execute {
        next_state.set(GameState::Execution);
        events.write(ActionListExecute {
            action_list: action_list.actions.clone(),
        });
    }
}

fn command_button_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &mut Transform, &CommandButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut action_list: ResMut<ActionList>,
    colors: Res<UIRoverColors>,
    mut action_writer: EventWriter<ActionList>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut image, mut trans, command) in &mut interaction_query {
        let action_list_selection = action_list.current_selection;

        match *interaction {
            Interaction::Pressed => {
                match command.0 {
                    ActionType::MoveUp => {
                        commands.spawn((
                            AudioPlayer::new(asset_server.load("sfx/up.ogg")),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    ActionType::MoveDown => {
                        commands.spawn((
                            AudioPlayer::new(asset_server.load("sfx/down.ogg")),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    ActionType::MoveRight => {
                        commands.spawn((
                            AudioPlayer::new(asset_server.load("sfx/right.ogg")),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    ActionType::MoveLeft => {
                        commands.spawn((
                            AudioPlayer::new(asset_server.load("sfx/left.ogg")),
                            PlaybackSettings::DESPAWN,
                        ));
                    }

                    ActionType::Wait => {
                        commands.spawn((
                            AudioPlayer::new(asset_server.load("sfx/wait.ogg")),
                            PlaybackSettings::DESPAWN,
                        ));
                    }
                }

                image.color = *colors.0.get(action_list_selection).unwrap();

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
                image.color = *colors.0.get(action_list_selection).unwrap();
                trans.scale = Vec3::new(1.1, 1.1, 1.1);
            }
            Interaction::None => {
                image.color = Color::WHITE;
                trans.scale = Vec3::new(1.0, 1.0, 1.0);
            }
        }
    }
}

fn robot_button_handler(
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
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }

    if has_to_update {
        action_writer.write(action_list.clone());
    }
}

fn delete_action_handler(
    mut interaction_query: Query<(&Interaction, &ActionDeleteButton), Changed<Interaction>>,
    mut action_list: ResMut<ActionList>,
    mut action_writer: EventWriter<ActionList>,
) {
    let mut has_to_update: bool = false;
    for (interaction, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                action_list
                    .actions
                    .get_mut(button.rover_index)
                    .unwrap()
                    .remove(button.action_index);
                has_to_update = true;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }

    if has_to_update {
        action_writer.write(action_list.clone());
    }
}

fn clear_all_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ClearAllButton>)>,
    mut action_list: ResMut<ActionList>,
    mut action_writer: EventWriter<ActionList>,
) {
    let mut has_to_update: bool = false;
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for actions in action_list.actions.iter_mut() {
                    actions.clear();
                }
                has_to_update = true;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }

    if has_to_update {
        action_writer.write(action_list.clone());
    }
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

pub const SELECTION_LIGHT_INTENSITY: f32 = 500_000.0;

/// Spawns a selection light if one doesn't exist yet
fn spawn_selection_light(
    mut commands: Commands,
    selection_light_query: Query<Entity, With<SelectionLight>>,
    rover_query: Query<Entity, With<RoverEntity>>,
) {
    if selection_light_query.is_empty() && !rover_query.is_empty() {
        commands.spawn((
            SelectionLight,
            LevelElement,
            SpotLight {
                intensity: SELECTION_LIGHT_INTENSITY,
                color: Color::srgb(1.0, 0.95, 0.8),
                shadows_enabled: true,
                range: 20.0,
                radius: 0.5,
                outer_angle: 0.6,
                inner_angle: 0.4,
                ..default()
            },
            Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}

/// Updates the selection light position to be above the currently selected rover
fn update_selection_light(
    action_list: Res<ActionList>,
    rover_query: Query<(&RoverEntity, &Transform)>,
    mut light_query: Query<
        (&mut Transform, &mut SpotLight),
        (With<SelectionLight>, Without<RoverEntity>),
    >,
    game_state: Res<State<GameState>>,
) {
    if let Ok((mut light_transform, mut spotlight)) = light_query.single_mut() {
        // Turn off light during execution, turn on during programming
        if *game_state.get() == GameState::Execution {
            spotlight.intensity = 0.0;
            return;
        } else {
            spotlight.intensity = SELECTION_LIGHT_INTENSITY;
        }

        let selected_index = action_list.current_selection;

        for (rover, rover_transform) in rover_query.iter() {
            if rover.identifier as usize == selected_index {
                let target_position = rover_transform.translation + Vec3::new(0.0, 3.0, 0.0);
                let look_at_position = rover_transform.translation;

                light_transform.translation = target_position;
                light_transform.look_at(look_at_position, Vec3::Y);
                break;
            }
        }
    }
}

/// Handles clicking on rovers in the 3D world to select them
pub fn on_rover_click(
    click: Trigger<Pointer<Click>>,
    rover_query: Query<(Entity, &RoverEntity)>,
    mut debug_entity_request: EventWriter<DebugLogEntityRequest>,
    mut action_list: ResMut<ActionList>,
    mut action_writer: EventWriter<ActionList>,
    game_state: Res<State<GameState>>,
    children_query: Query<&Children>,
) {
    debug_entity_request.write(DebugLogEntityRequest(click.event().target));

    // Only allow selection during programming state
    if *game_state.get() != GameState::Programming {
        return;
    }

    let target_entity = click.event().target;

    for (rover_entity, rover) in rover_query.iter() {
        if let Ok(children) = children_query.get(rover_entity) {
            for child in children.iter() {
                if child == target_entity || is_descendant(child, target_entity, &children_query) {
                    let rover_index = rover.identifier as usize;
                    if action_list.current_selection != rover_index {
                        action_list.current_selection = rover_index;
                        action_writer.write(action_list.clone());
                    }
                    return;
                }
            }
        }
    }

    log::info!(
        "No RoverEntity found as ancestor of clicked entity {:?}",
        target_entity
    );
}

/// Helper function to check if target is a descendant of parent
fn is_descendant(parent: Entity, target: Entity, children_query: &Query<&Children>) -> bool {
    if parent == target {
        return true;
    }

    if let Ok(children) = children_query.get(parent) {
        for child in children.iter() {
            if is_descendant(child, target, children_query) {
                return true;
            }
        }
    }

    false
}
