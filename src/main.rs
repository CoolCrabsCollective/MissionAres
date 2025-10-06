extern crate core;

mod debug_camera_controller;
mod game_control;
mod hentai_anime;
mod level;
mod level_spawner;
mod mesh_loader;
mod particle;
mod puzzle_evaluation;
mod rover;
mod scene_hook;
mod title_screen;
mod ui;

use crate::debug_camera_controller::DebugCameraControllerPlugin;
use crate::game_control::actions::ActionController;
use crate::level::GRADVS_ONERATOR_PLUGIN;
use crate::level_spawner::LevelSpawnerPlugin;
use crate::mesh_loader::MeshLoaderPlugin;
use crate::particle::dust::DustPlugin;
use crate::particle::fail_particle::FailParticlePlugin;
use crate::particle::particle::ParticlePlugin;
use crate::puzzle_evaluation::PuzzleEvaluationPlugin;
use crate::title_screen::{GameState, TitleScreenPlugin};
use crate::ui::battery_ui::BatteryUIPlugin;
use crate::ui::interactive_button::{InteractiveButton, InteractiveButtonPlugin};
use bevy::app::{App, AppExit, PluginGroup};
use bevy::asset::AssetMetaCheck;
use bevy::ecs::entity::EntityDoesNotExistError;
use bevy::ecs::error::{BevyError, ErrorContext, GLOBAL_ERROR_HANDLER};
use bevy::ecs::query::QueryEntityError;
use bevy::ecs::world::error::EntityMutableFetchError;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, FilterMode};
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy::DefaultPlugins;
use ui::control_ui::ControlUIPlugin;

fn main() {
    let mut app = App::new();

    GLOBAL_ERROR_HANDLER
        .set(global_error_handler)
        .expect("The error handler can only be set once.");

    let default_sampler = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::from(AddressMode::ClampToEdge),
        address_mode_v: ImageAddressMode::from(AddressMode::ClampToEdge),
        address_mode_w: ImageAddressMode::from(AddressMode::ClampToEdge),
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
                    title: "Mission Ares".to_string(),
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
    app.add_plugins(InteractiveButtonPlugin);
    app.add_plugins(MeshLoaderPlugin);
    app.add_plugins(ActionController);
    app.add_plugins(ControlUIPlugin);
    app.add_plugins(TitleScreenPlugin);
    app.add_plugins(DebugCameraControllerPlugin);
    app.add_plugins(GRADVS_ONERATOR_PLUGIN);
    app.add_plugins(LevelSpawnerPlugin);
    app.add_plugins(PuzzleEvaluationPlugin);
    app.add_plugins(BatteryUIPlugin);
    app.add_plugins(DustPlugin);
    app.add_plugins(ParticlePlugin);
    app.add_plugins(FailParticlePlugin);
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

fn global_error_handler(error: BevyError, ctx: ErrorContext) {
    // ignore the bullshit issue where modifying an entity causes a crash in dev
    // if the entity is also de-spawned by another system in the same frame
    if let Some(entity_fetch_error) = error.downcast_ref::<EntityMutableFetchError>() {
        if matches!(
            entity_fetch_error,
            EntityMutableFetchError::EntityDoesNotExist(_)
        ) {
            trace!("EntityDoesNotExist, ignoring.");
            return;
        }
    }

    dbg!(&error, &ctx);
    bevy::ecs::error::error(error, ctx);
}
