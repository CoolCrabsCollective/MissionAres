use crate::rover::RoverEntity;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::prelude::{
    Commands, Component, Entity, ImageNode, Node, PositionType, Query, Res, Resource, With, Without,
};
use bevy::ui::Val;
use bevy::utils::default;

pub struct BatteryUIPlugin;

#[derive(Component)]
pub struct BatteryUIElement {
    rover_id: Entity,
}

#[derive(Component)]
pub struct BatteryUIAttachment;

#[derive(Resource)]
pub struct BatteryImages {
    pub images: Vec<Handle<Image>>,
}

impl Plugin for BatteryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load);
        app.add_systems(Update, (rebuild, update));
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BatteryImages {
        images: vec![
            asset_server.load("battery/battery_0.png"),
            asset_server.load("battery/battery_1.png"),
            asset_server.load("battery/battery_2.png"),
            asset_server.load("battery/battery_3.png"),
        ],
    })
}

fn rebuild(
    mut commands: Commands,
    rovers: Query<(Entity, &mut RoverEntity), Without<BatteryUIAttachment>>,
    images: Res<BatteryImages>,
    asset_server: Res<AssetServer>,
) {
    for (entity, rover) in rovers {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                ..default()
            },
            ImageNode {
                image: images.images[0].clone(),
                ..default()
            },
            BatteryUIElement { rover_id: entity },
        ));
        commands.entity(entity).insert(BatteryUIAttachment);
    }
}

fn update(
    mut commands: Commands,
    elements: Query<(Entity, &mut ImageNode, &mut Node, &mut BatteryUIElement)>,
    rovers: Query<(Entity, &mut RoverEntity), With<BatteryUIAttachment>>,
    images: Res<BatteryImages>,
) {
    for (ui_entity, mut img, node, ui_elem) in elements {
        let mut found = false;
        for (id, rover) in &rovers {
            if id == ui_elem.rover_id {
                found = true;
                img.image = images.images[rover.battery_level as usize].clone();
            }
        }

        if !found {
            commands.entity(ui_entity).despawn();
        }
    }
}
