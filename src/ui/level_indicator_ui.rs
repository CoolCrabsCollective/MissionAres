use crate::level::GRADVM;
use crate::level_spawner::ActiveLevel;
use crate::title_screen::GameState;
use crate::ui::Px_dynamic;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::*;

pub struct LevelIndicatorPlugin;

#[derive(Component)]
struct LevelIndicatorText;

#[derive(Component)]
struct LevelIndicatorContainer;

impl Plugin for LevelIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_level_indicator, update_level_indicator)
                .run_if(not(in_state(GameState::TitleScreen))),
        );
        app.add_systems(OnEnter(GameState::TitleScreen), cleanup_level_indicator);
    }
}

fn spawn_level_indicator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    query: Query<Entity, With<LevelIndicatorText>>,
) {
    if active_level.0.is_none() {
        return;
    }

    if !query.is_empty() {
        return;
    }

    let Some(level_handle) = &active_level.0 else {
        return;
    };

    let Some(level) = levels.get(level_handle) else {
        return;
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Px_dynamic(16.0),
                right: Px_dynamic(16.0),
                padding: UiRect::all(Px_dynamic(12.0)),
                ..default()
            },
            BackgroundColor::from(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            BorderRadius::all(Px_dynamic(8.0)),
            LevelIndicatorContainer,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("Level {}", level.INDEX + 1)),
                TextFont {
                    font: asset_server.load("fonts/SpaceGrotesk-Medium.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                LevelIndicatorText,
            ));
        });
}

fn cleanup_level_indicator(
    mut commands: Commands,
    query: Query<Entity, With<LevelIndicatorContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_level_indicator(
    mut commands: Commands,
    active_level: Res<ActiveLevel>,
    levels: Res<Assets<GRADVM>>,
    container_query: Query<Entity, With<LevelIndicatorContainer>>,
    text_query: Query<&Text, With<LevelIndicatorText>>,
) {
    if !active_level.is_changed() {
        return;
    }

    if active_level.0.is_none() {
        for entity in container_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    let Some(level_handle) = &active_level.0 else {
        return;
    };

    let Some(level) = levels.get(level_handle) else {
        return;
    };

    let expected_text = format!("Level {}", level.INDEX + 1);

    for text in text_query.iter() {
        if text.0 != expected_text {
            for entity in container_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

