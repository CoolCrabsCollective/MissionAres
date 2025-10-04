use crate::level_spawner::AfterLevelSpawnEvent;
use crate::rover::RoverEntity;
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Component, Entity, EventReader, ImageNode, Node, Query, Res, With};
use bevy::ui::Val;
use bevy::utils::default;

pub struct BatteryUIPlugin;

#[derive(Component)]
pub struct BatteryUIElement {
    rover_id: Entity,
}

impl Plugin for BatteryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rebuild_ui, update_batteries));
    }
}

fn rebuild_ui(
    mut commands: Commands,
    events: EventReader<AfterLevelSpawnEvent>,
    rovers: Query<Entity, With<RoverEntity>>,
    elements: Query<Entity, With<BatteryUIElement>>,
    asset_server: Res<AssetServer>,
) {
    if events.is_empty() {
        return;
    }

    for level_element in elements.iter() {
        commands.entity(level_element).despawn();
    }

    for rover in rovers {
        commands.spawn((
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                ..default()
            },
            ImageNode {
                image: asset_server.load("battery/battery_3.png"),
                ..default()
            },
            BatteryUIElement { rover_id: rover },
        ));
    }
}

fn update_batteries(mut commands: Commands) {}
