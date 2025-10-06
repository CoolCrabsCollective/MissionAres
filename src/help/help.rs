use crate::title_screen::GameState;
use crate::ui::interactive_button::InteractiveButton;
use bevy::prelude::*;
use GameState::TitleScreen;

pub struct HelpPlugin;

#[derive(Component)]
pub struct HelpButton {
    pub help_visible: bool,
}

#[derive(Component)]
pub struct HelpDialog;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(TitleScreen), (add_player_help));
        app.add_systems(Update, toggle_help_visible.after(add_player_help));
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
                    Color::WHITE,
                    Color::srgba(0.25, 1.0, 1.0, 1.0),
                    Color::srgba(0.25, 1.0, 1.0, 1.0),
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
        (Entity, &mut HelpButton, &Interaction),
        (Changed<Interaction>, With<InteractiveButton>),
    >,
    mut query_dialog: Query<(Entity, &mut HelpDialog)>,
    asset_server: Res<AssetServer>,
) {
    for (mut entity, mut help, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                help.help_visible = !help.help_visible;

                if help.help_visible {
                    let image_dialog = asset_server.load("tutorial.png");

                    commands
                        .spawn((Node {
                            height: Val::Percent(20.0),
                            width: Val::Percent(100.0),
                            position_type: PositionType::Absolute,

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            let move_node_for_img = Node {
                                height: Val::Px(100.0),
                                width: Val::Px(200.0),
                                ..default()
                            };

                            let img_move_node = ImageNode {
                                image: image_dialog.clone(),
                                image_mode: NodeImageMode::Auto,
                                ..default()
                            };

                            parent.spawn((
                                HelpDialog,
                                img_move_node.clone(),
                                move_node_for_img.clone(),
                            ));
                        });
                } else {
                    query_dialog.iter_mut().for_each(|(entity, help)| {
                        commands.entity(entity).despawn();
                    });
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
