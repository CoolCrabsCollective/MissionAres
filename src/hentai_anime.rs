use crate::level_spawner::LevelElement;
use crate::mesh_loader::MeshLoader;
use bevy::prelude::*;
use bevy::scene::{SceneInstance, SceneInstanceReady};
use std::time::Duration;

#[derive(Component, Reflect, Clone, Default)]
pub struct Animation {
    pub animation_list: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
    pub player_entity: Option<Entity>,
}

pub fn setup_anime_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    mut animations_to_play: Query<&mut Animation>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut animation) = animations_to_play.get_mut(trigger.target()) {
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(player) = players.get_mut(child) {
                // Player found, add to entity for use later
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.clone().graph));

                animation.player_entity = Option::from(child);
            }
        }
    }
}

pub fn play_all_animations_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&Animation>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation) = animations_to_play.get(trigger.target()) {
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {
                for hentai in animation.animation_list.iter() {
                    player.play(*hentai).repeat();
                }

                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()))
                    .insert(player.clone());
            }
        }
    }
}
