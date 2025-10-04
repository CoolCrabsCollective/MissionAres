extern crate core;

mod debug_camera_controller;
mod game_control;
mod hentai_anime;
mod level;
mod level_spawner;
mod mesh_loader;
mod poop;
mod title_screen;

use crate::debug_camera_controller::DebugCameraControllerPlugin;
use crate::game_control::actions::ActionController;
use crate::game_control::control_ui::ControlUiPlugin;
use crate::level::GRADVS_ONERATOR_PLUGIN;
use crate::level_spawner::LevelSpawnerPlugin;
use crate::mesh_loader::MeshLoaderPlugin;
use crate::title_screen::{GameState, TitleScreenPlugin};
use bevy::app::{App, AppExit, PluginGroup};
use bevy::asset::AssetMetaCheck;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, FilterMode};
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy::DefaultPlugins;

fn main() {
    let mut app = App::new();

    let default_sampler = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::from(AddressMode::Repeat),
        address_mode_v: ImageAddressMode::from(AddressMode::Repeat),
        address_mode_w: ImageAddressMode::from(AddressMode::Repeat),
        mag_filter: ImageFilterMode::from(FilterMode::Linear),
        min_filter: ImageFilterMode::from(FilterMode::Linear),
        mipmap_filter: ImageFilterMode::from(FilterMode::Linear),
        ..default()
    };

    let is_web = cfg!(target_arch = "wasm32");
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    title: "LD58".to_string(),
                    cursor_options: CursorOptions {
                        visible: true,
                        // note this bug: https://github.com/bevyengine/bevy/issues/16237
                        grab_mode: CursorGrabMode::None,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin { default_sampler })
            .set(AssetPlugin {
                meta_check: if is_web {
                    AssetMetaCheck::Never
                } else {
                    Default::default()
                },
                ..default()
            }),
    );
    app.add_plugins(MeshLoaderPlugin);
    app.add_plugins(ActionController);
    app.add_plugins(ControlUiPlugin);
    app.add_plugins(TitleScreenPlugin);
    app.add_plugins(DebugCameraControllerPlugin);
    app.add_plugins(GRADVS_ONERATOR_PLUGIN);
    app.add_plugins(LevelSpawnerPlugin);
    app.insert_state(GameState::TitleScreen);
    app.add_systems(Update, quit_on_escape);

    app.run();
}

fn quit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
