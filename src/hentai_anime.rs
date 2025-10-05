use crate::level_spawner::LevelElement;
use crate::mesh_loader::MeshLoader;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Reflect, Clone, Default)]
pub struct Animation {
    pub animation_list: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
    pub group_is_playing: bool,
}

pub struct HentaiAnimePlugin;

impl Plugin for HentaiAnimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_hentai_anime_anime_level);
        app.add_systems(Update, debug_print_animation_playing);
    }
}

pub fn setup_hentai_anime_anime_level(
    mut commands: Commands,
    mut anime_query: Query<(&mut Animation), With<LevelElement>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (mut animation) in anime_query.iter_mut() {
        for (entity, mut player) in &mut players {
            if !animation.group_is_playing {
                let mut transitions = AnimationTransitions::new();

                if animation.animation_list.len() == 1 {
                    transitions
                        .play(&mut player, animation.animation_list[0], Duration::ZERO)
                        .repeat();

                    // player.start(animation.animation_list[0].clone()).repeat();

                    commands
                        .entity(entity)
                        .insert(AnimationGraphHandle(animation.graph.clone()))
                        .insert(transitions);

                    animation.group_is_playing = true;
                }
            }
        }
    }
    // }
}

pub fn debug_print_animation_playing(mut player_query: Query<(&mut AnimationPlayer)>) {
    for (mut player) in player_query.iter_mut() {
        dbg!(
            &player
                .playing_animations()
                .into_iter()
                .enumerate()
                .collect::<Vec<_>>()
        );
    }
}
