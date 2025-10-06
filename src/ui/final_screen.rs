use crate::help::help::{HelpButton, HelpDialog};
use crate::level::{GRADVM, GRADVM_ONVSTVS};
use crate::level_spawner::{ActiveLevel, LevelElement};
use crate::particle::particle::Particle;
use crate::puzzle_evaluation::PuzzleResponseEvent;
use crate::title_screen::GameState;
use crate::ui::battery_ui::BatteryUIElement;
use crate::ui::control_ui::ControlUI;
use crate::ui::interactive_button::InteractiveButton;
use bevy::color::Srgba;
use bevy::prelude::*;

pub struct FinalScreenPlugin;

#[derive(Component)]
pub struct ReturnToTitleButton;

#[derive(Component)]
pub struct FinalScreenUI;

impl Plugin for FinalScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                show_final_screen.run_if(in_state(GameState::Execution)),
                return_to_title_click_handler,
                adjust_camera_for_final_screen,
            ),
        );
    }
}

fn show_final_screen(
    mut commands: Commands,
    mut puzzle_response_events: EventReader<PuzzleResponseEvent>,
    asset_server: Res<AssetServer>,
    existing_ui: Query<Entity, With<FinalScreenUI>>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    level_handles: Res<GRADVM_ONVSTVS>,
    level_elements: Query<Entity, With<LevelElement>>,
    particles: Query<Entity, (With<Particle>, Without<LevelElement>)>,
    control_ui: Query<Entity, With<ControlUI>>,
    battery_ui: Query<Entity, With<BatteryUIElement>>,
    help_button: Query<Entity, With<HelpButton>>,
    help_dialog: Query<Entity, With<HelpDialog>>,
) {
    for event in puzzle_response_events.read() {
        if *event == PuzzleResponseEvent::Solved && existing_ui.is_empty() {
            let Some(active_level_handle) = &active_level.0 else {
                continue;
            };

            let Some(current_level) = levels.get(active_level_handle) else {
                continue;
            };

            let final_level_index = level_handles.GRADVS.len() - 1;
            if current_level.INDEX as usize != final_level_index {
                continue;
            }

            // Clean up level elements and in-game UI before showing the final screen
            for level_element in level_elements.iter() {
                commands.entity(level_element).despawn();
            }
            for particle in particles.iter() {
                commands.entity(particle).despawn();
            }
            for ui_element in control_ui.iter() {
                commands.entity(ui_element).despawn();
            }
            for ui_element in battery_ui.iter() {
                commands.entity(ui_element).despawn();
            }
            for ui_element in help_button.iter() {
                commands.entity(ui_element).despawn();
            }
            for ui_element in help_dialog.iter() {
                commands.entity(ui_element).despawn();
            }

            commands
                .spawn((
                    FinalScreenUI,
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
                    ZIndex(2000),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Mission Completed!"),
                        TextFont {
                            font: asset_server.load("fonts/SpaceGrotesk-Bold.ttf"),
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Node {
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new("All minerals have been successfully collected.\nThe mission was a success!"),
                        TextFont {
                            font: asset_server.load("fonts/SpaceGrotesk-Regular.ttf"),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Node {
                            margin: UiRect::bottom(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    parent
                        .spawn((
                            Button,
                            ReturnToTitleButton,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor::from(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                            BorderRadius::all(Val::Px(15.0)),
                            BorderColor::from(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                            InteractiveButton::simple(
                                Color::Srgba(Srgba::hex("3a312e").unwrap()),
                                Color::WHITE,
                                true,
                            ),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Play Again"),
                                TextFont {
                                    font: asset_server.load("fonts/SpaceGrotesk-Light.ttf"),
                                    font_size: 40.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                            ));
                        });
                });
        }
    }
}

fn return_to_title_click_handler(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<ReturnToTitleButton>,
            With<InteractiveButton>,
        ),
    >,
    mut commands: Commands,
    ui_query: Query<Entity, With<FinalScreenUI>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut active_level: ResMut<ActiveLevel>,
    level_handles: Res<GRADVM_ONVSTVS>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for entity in ui_query.iter() {
                commands.entity(entity).despawn();
            }

            active_level.0 = Some(level_handles.GRADVS[0].clone());
            next_state.set(GameState::TitleScreen);
        }
    }
}

fn adjust_camera_for_final_screen(
    final_screen_query: Query<&FinalScreenUI, Added<FinalScreenUI>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    if final_screen_query.is_empty() {
        return;
    }

    for mut transform in camera_query.iter_mut() {
        // Position camera to look at the skybox
        transform.translation = Vec3::new(0.0, 5.0, 10.0);
        transform.look_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
    }
}
