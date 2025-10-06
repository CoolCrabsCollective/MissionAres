use crate::game_control::actions::{Action, ActionList, ActionType};
use crate::level::GRADVM;
use crate::level_spawner::ActiveLevel;
use crate::rover::ActionListExecute;
use crate::title_screen::GameState;
use crate::ui::interactive_button::InteractiveButton;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
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
pub struct RobotButton(pub i32);

impl Plugin for ControlUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rebuild_control_ui.run_if(not(in_state(GameState::TitleScreen))),
                command_button_handler.run_if(in_state(GameState::Programming)),
                robot_button_handler.run_if(in_state(GameState::Programming)),
                delete_action_handler.run_if(in_state(GameState::Programming)),
            ),
        );
        app.add_systems(Update, execute_handler);
        app.add_systems(Update, update_scroll_position);

        app.insert_resource(UIRoverColors(vec![
            Color::srgba(0.25, 1.0, 0.25, 1.0), // green
            Color::srgba(0.25, 0.5, 1.0, 1.0),  // blue
            Color::srgba(1.0, 1.0, 0.25, 1.0),  // yellow
            Color::srgba(1.0, 0.0, 1.0, 1.0),   // purple
            Color::srgba(1.0, 0.0, 0.0, 1.0),   // red
        ]));

        app.insert_resource(RoverColors(vec![
            Color::srgba(75.0 / 255.0, 214.0 / 255.0, 75.0 / 255.0, 0.35), // green
            Color::srgba(68.0 / 255.0, 94.0 / 255.0, 221.0 / 255.0, 0.35), // blue
            Color::srgba(246.0 / 255.0, 219.0 / 255.0, 53.0 / 255.0, 0.35), // yellow
            Color::srgba(140.0 / 255.0, 29.0 / 255.0, 140.0 / 255.0, 0.35), // purple
            Color::srgba(233.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0, 0.35), // red
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
                            top_right: Val::Px(8.0),
                            bottom_right: Val::Px(8.0),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        build_control_panel(parent, &asset_server);

                        let rover_colors = &all_rover_colors.0[0..number_of_rovers];

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
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        for (robot_idx, color) in rover_colors.iter().enumerate() {
                                            let transparent = Color::srgba(0.0, 0.0, 0.0, 0.0);
                                            let robot_bg_color =
                                                if robot_idx == selected_robot_index {
                                                    Color::srgb(1.0, 1.0, 1.0)
                                                } else {
                                                    transparent
                                                };

                                            parent
                                                .spawn((
                                                    Button,
                                                    RobotButton(robot_idx as i32),
                                                    Node {
                                                        width: Val::Px(50.0),
                                                        height: Val::Px(50.0),
                                                        margin: UiRect::all(Val::Px(5.0)),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        border: UiRect::all(Val::Px(5.0)),
                                                        ..default()
                                                    },
                                                    BorderColor::from(robot_bg_color),
                                                    BorderRadius::all(Val::Px(25.0)),
                                                    InteractiveButton {
                                                        regular_background_color: transparent,
                                                        hover_background_color: transparent,
                                                        pressed_background_color: transparent,
                                                        regular_border_color: robot_bg_color,
                                                        hover_border_color: robot_bg_color
                                                            .darker(0.1),
                                                        pressed_border_color: robot_bg_color
                                                            .darker(0.2),
                                                        regular_image_color: *color,
                                                        hover_image_color: color.darker(0.1),
                                                        pressed_image_color: color.darker(0.2),
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
                                        }
                                    });

                                parent
                                    .spawn((Node {
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
                                                for action in event.actions[robot_idx].iter() {
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
        width: Val::Percent(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.0)),
        border: UiRect {
            right: Val::Px(6.0),
            top: Val::Px(6.0),
            bottom: Val::Px(6.0),
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
        height: Val::Px(24.0),
        width: Val::Px(24.0),
        aspect_ratio: Some(1.0f32),
        margin: UiRect::left(Val::Px(5.0)),
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
        width: Val::Px(56.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(12.0),
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
                height: Val::Px(60.0),
                min_height: Val::Px(60.0),
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
}

// handlers

fn execute_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExecuteButton>)>,
    mut events: EventWriter<ActionListExecute>,
    mut next_state: ResMut<NextState<GameState>>,
    action_list: Res<ActionList>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Execution);
            events.write(ActionListExecute {
                action_list: action_list.actions.clone(),
            });
        }
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
                    _ => (),
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
    for (interaction, mut button) in interaction_query.iter_mut() {
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
