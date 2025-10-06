use crate::title_screen::GameState;
use crate::ui::interactive_button::InteractiveButton;
use bevy::color::Srgba;
use bevy::prelude::*;

pub struct HelpPlugin;

const UI_WHITE: Color = Color::srgb(0.83, 0.83, 0.83);

#[derive(Component)]
pub struct HelpButton {
    pub help_visible: bool,
}

#[derive(Component)]
pub struct HelpDialog;

#[derive(Component)]
pub struct CloseHelpButton;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::TitleScreen), add_player_help);
        app.add_systems(OnEnter(GameState::TitleScreen), cleanup_help);
        app.add_systems(
            Update,
            (toggle_help_visible, close_help_handler).run_if(not(in_state(GameState::TitleScreen))),
        );
    }
}

fn cleanup_help(
    mut commands: Commands,
    button_query: Query<Entity, With<HelpButton>>,
    dialog_query: Query<Entity, With<HelpDialog>>,
) {
    for entity in button_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in dialog_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn add_player_help(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("help_plugin_assets/question.png");
    commands
        .spawn((Node {
            height: Val::Px(48.0),
            width: Val::Px(48.0),
            margin: UiRect::all(Val::Px(12.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|parent| {
            let move_node_for_img = Node {
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0f32),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };

            let img_move_node = ImageNode {
                image: image.clone(),
                image_mode: NodeImageMode::Auto,
                ..default()
            };

            parent.spawn((
                Button,
                HelpButton {
                    help_visible: false,
                },
                InteractiveButton::simple_image(
                    Color::srgba(0.0, 0.0, 0.0, 0.0),
                    UI_WHITE,
                    UI_WHITE,
                    UI_WHITE,
                    true,
                ),
                img_move_node.clone(),
                move_node_for_img.clone(),
            ));
        });
}

pub fn toggle_help_visible(
    mut commands: Commands,
    mut query: Query<
        (&mut HelpButton, &Interaction),
        (Changed<Interaction>, With<InteractiveButton>),
    >,
    query_dialog: Query<Entity, With<HelpDialog>>,
    asset_server: Res<AssetServer>,
) {
    for (mut help, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                help.help_visible = !help.help_visible;

                if help.help_visible {
                    let move_icon = asset_server.load("help_plugin_assets/move.png");
                    let low_credits_icon = asset_server.load("help_plugin_assets/low_credits.png");
                    let sun_icon = asset_server.load("help_plugin_assets/sun.png");
                    let high_credits_icon =
                        asset_server.load("help_plugin_assets/high_credits.png");

                    commands
                        .spawn((
                            HelpDialog,
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(0.0),
                                left: Val::Percent(0.0),
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor::from(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                            ZIndex(1000),
                        ))
                        .with_children(|parent| {
                            // Main content container
                            parent
                                .spawn((Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    row_gap: Val::Px(30.0),
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    // Movement drains battery section
                                    parent
                                        .spawn((Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(15.0),
                                            margin: UiRect::bottom(Val::Px(40.0)),
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new("Movement drains battery"),
                                                TextFont {
                                                    font: asset_server
                                                        .load("fonts/SpaceGrotesk-Medium.ttf"),
                                                    font_size: 32.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                                TextLayout::new_with_justify(JustifyText::Center),
                                            ));

                                            // Icons row
                                            parent
                                                .spawn((Node {
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::Center,
                                                    column_gap: Val::Px(20.0),
                                                    ..default()
                                                },))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        ImageNode {
                                                            image: move_icon.clone(),
                                                            image_mode: NodeImageMode::Auto,
                                                            ..default()
                                                        },
                                                        Node {
                                                            height: Val::Px(80.0),
                                                            ..default()
                                                        },
                                                    ));

                                                    parent.spawn((
                                                        ImageNode {
                                                            image: low_credits_icon.clone(),
                                                            image_mode: NodeImageMode::Auto,
                                                            ..default()
                                                        },
                                                        Node {
                                                            height: Val::Px(80.0),
                                                            ..default()
                                                        },
                                                    ));
                                                });
                                        });

                                    // Sun replenishes battery section
                                    parent
                                        .spawn((Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(15.0),
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new("The sun replenishes battery"),
                                                TextFont {
                                                    font: asset_server
                                                        .load("fonts/SpaceGrotesk-Medium.ttf"),
                                                    font_size: 32.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                                TextLayout::new_with_justify(JustifyText::Center),
                                            ));

                                            // Icons row
                                            parent
                                                .spawn((Node {
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::Center,
                                                    column_gap: Val::Px(20.0),
                                                    ..default()
                                                },))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        ImageNode {
                                                            image: sun_icon.clone(),
                                                            image_mode: NodeImageMode::Auto,
                                                            ..default()
                                                        },
                                                        Node {
                                                            height: Val::Px(80.0),
                                                            ..default()
                                                        },
                                                    ));

                                                    parent.spawn((
                                                        ImageNode {
                                                            image: high_credits_icon.clone(),
                                                            image_mode: NodeImageMode::Auto,
                                                            ..default()
                                                        },
                                                        Node {
                                                            height: Val::Px(80.0),
                                                            ..default()
                                                        },
                                                    ));
                                                });
                                        });

                                    // Close button
                                    parent
                                        .spawn((
                                            Button,
                                            CloseHelpButton,
                                            Node {
                                                width: Val::Px(200.0),
                                                height: Val::Px(60.0),
                                                border: UiRect::all(Val::Px(15.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                margin: UiRect::top(Val::Px(20.0)),
                                                ..default()
                                            },
                                            BackgroundColor::from(Color::Srgba(
                                                Srgba::hex("3a312e").unwrap(),
                                            )),
                                            BorderRadius::all(Val::Px(15.0)),
                                            BorderColor::from(Color::Srgba(
                                                Srgba::hex("3a312e").unwrap(),
                                            )),
                                            InteractiveButton::simple(
                                                Color::Srgba(Srgba::hex("3a312e").unwrap()),
                                                Color::WHITE,
                                                true,
                                            ),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new("Close"),
                                                TextFont {
                                                    font: asset_server
                                                        .load("fonts/SpaceGrotesk-Light.ttf"),
                                                    font_size: 36.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                                            ));
                                        });
                                });
                        });
                } else {
                    for entity in query_dialog.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn close_help_handler(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<CloseHelpButton>,
            With<InteractiveButton>,
        ),
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<HelpDialog>>,
    mut help_button_query: Query<&mut HelpButton>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn();
            }
            for mut help_button in help_button_query.iter_mut() {
                help_button.help_visible = false;
            }
        }
    }
}
