mod debug_camera_controller;
mod mesh_loader;
mod object_spawner;
mod scene_loader;
mod title_screen;
mod level;

use crate::debug_camera_controller::DebugCameraControllerPlugin;
use crate::mesh_loader::MeshLoaderPlugin;
use crate::object_spawner::ObjectSpawnerPlugin;
use crate::scene_loader::SceneLoaderPlugin;
use crate::title_screen::{GameState, TitleScreenPlugin};
use bevy::app::{App, PluginGroup};
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
                    title: "Bevy Quickstart Game".to_string(),
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
    app.add_plugins(SceneLoaderPlugin);
    app.add_plugins(TitleScreenPlugin);
    app.add_plugins(DebugCameraControllerPlugin);
    app.add_plugins(ObjectSpawnerPlugin);
    app.insert_state(GameState::TitleScreen);

    app.run();
}
