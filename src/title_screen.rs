use crate::ui::interactive_button::InteractiveButton;
use bevy::prelude::*;

pub struct TitleScreenPlugin;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct TitleScreenUI;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    Game,
}

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::TitleScreen), on_enter);
        app.add_systems(OnExit(GameState::TitleScreen), clean);
        app.add_systems(
            Update,
            start_game_click_handler.run_if(in_state(GameState::TitleScreen)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TitleScreenUI,
            Node {
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    StartGameButton,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(15.0)),
                        top: Val::Percent(35.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor::from(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                    BorderRadius::all(Val::Px(15.0)),
                    BorderColor::from(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                    InteractiveButton::simple(
                        Color::Srgba(Srgba::hex("3a312e").unwrap()),
                        Color::WHITE,
                        false,
                    ),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Game"),
                        TextFont {
                            font: asset_server.load("fonts/SpaceGrotesk-Light.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                        Transform::default(),
                        TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                        //TextShadow::default(),
                    ));
                });

            parent.spawn((
                Text::new("Made in Rust!"),
                TextFont {
                    font: asset_server.load("fonts/SpaceGrotesk-Light.ttf"),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::Srgba(Srgba::hex("3a312e").unwrap())),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
            ));
        });

    commands.spawn((
        TitleScreenUI,
        Node {
            position_type: PositionType::Absolute,
            height: Val::Auto,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        ImageNode::new(asset_server.load("logo.png")),
    ));
}

fn start_game_click_handler(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    gamepads: Query<&Gamepad>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Game);
        }
    }

    for gamepad in &gamepads {
        if gamepad.pressed(GamepadButton::Start) {
            next_state.set(GameState::Game);
        }
    }
}

fn clean(mut commands: Commands, query: Query<Entity, With<TitleScreenUI>>) {
    for ui_element in query.iter() {
        commands.entity(ui_element).despawn();
    }
}
