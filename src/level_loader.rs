use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::{Event, EventReader, Events},
        system::Commands,
    },
    log,
};

use crate::level::{Level, level_1};

pub struct LevelLoaderPlugin;

#[derive(Event)]
pub struct LevelLoadedEvent {
    level: Level,
}

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelLoadedEvent>();
        app.add_systems(Update, debug_level_load_event);
        app.add_systems(Startup, debug_add_fake_level_load_event);
    }
}

fn debug_add_fake_level_load_event(mut commands: Commands) {
    commands.send_event(LevelLoadedEvent { level: level_1() });
}

fn debug_level_load_event(mut events: EventReader<LevelLoadedEvent>) {
    for event in events.read() {
        log::info!("Level loaded: {:?}", event.level);
    }
}
