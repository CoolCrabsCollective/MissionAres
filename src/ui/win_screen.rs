use crate::level::{GRADVM, GRADVM_ONVSTVS};
use crate::level_spawner::ActiveLevel;
use crate::puzzle_evaluation::PuzzleResponseEvent;
use crate::title_screen::GameState;
use crate::ui::interactive_button::InteractiveButton;
use crate::ui::Px_dynamic;
use bevy::color::Srgba;
use bevy::prelude::*;

pub struct WinScreenPlugin;

#[derive(Component)]
pub struct NextLevelButton;

#[derive(Component)]
pub struct WinScreenUI;

#[derive(Event)]
pub struct NextLevelRequestEvent;

impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NextLevelRequestEvent>();
        app.add_systems(
            Update,
            (
                show_win_screen.run_if(in_state(GameState::Execution)),
                next_level_click_handler,
            ),
        );
    }
}

fn show_win_screen(
    mut commands: Commands,
    mut puzzle_response_events: EventReader<PuzzleResponseEvent>,
    asset_server: Res<AssetServer>,
    existing_ui: Query<Entity, With<WinScreenUI>>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    level_handles: Res<GRADVM_ONVSTVS>,
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
            if current_level.INDEX as usize == final_level_index {
                continue;
            }
            commands
                .spawn((
                    WinScreenUI,
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
                    parent.spawn((
                        Text::new("All minerals collected!"),
                        TextFont {
                            font: asset_server.load("fonts/SpaceGrotesk-Bold.ttf"),
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Node {
                            margin: UiRect::bottom(Px_dynamic(50.0)),
                            ..default()
                        },
                    ));

                    parent
                        .spawn((
                            Button,
                            NextLevelButton,
                            Node {
                                width: Val::Px(250.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(15.0)),
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
                                Text::new("Next Level"),
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

fn next_level_click_handler(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<NextLevelButton>,
            With<InteractiveButton>,
        ),
    >,
    mut commands: Commands,
    ui_query: Query<Entity, With<WinScreenUI>>,
    mut next_level_event_writer: EventWriter<NextLevelRequestEvent>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_level_event_writer.write(NextLevelRequestEvent);
            for entity in ui_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
