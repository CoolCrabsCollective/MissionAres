use crate::level_spawner::LevelElement;
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
        // app.add_systems(Update, debug_print_animation_playing);
    }
}

pub fn setup_hentai_anime_anime_level(
    mut commands: Commands,
    mut anime_query: Query<(Entity, &mut AnimationPlayer, &mut Animation), With<LevelElement>>,
    // mut player_query: Query<&mut AnimationPlayer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    clips_res: Res<Assets<AnimationClip>>,
) {
    for (entity, mut player, mut animation) in anime_query.iter_mut() {
        // for mut player in player_query.iter_mut() {
        if !animation.group_is_playing {
            let mut transitions = AnimationTransitions::new();

            if animation.animation_list.len() == 1 {
                // transitions
                //     .play(&mut player, animation.animation_list[0], Duration::ZERO)
                //     .repeat();

                player.start(animation.animation_list[0].clone()).repeat();

                commands
                    .entity(entity)
                    .insert(AnimationGraphHandle(animation.graph.clone()));
                // .insert(transitions);

                animation.group_is_playing = true;
            }

            // let mut transitions = AnimationTransitions::new();
            //
            // // for hentai in &animation.animation_list {
            // //     player.play(*hentai).repeat();
            // //
            // //     transitions
            // //         .play(&mut player, *hentai, Duration::ZERO)
            // //         .repeat();
            // // }
            //
            // if animation.animation_list.len() == 1 {
            //     // transitions
            //     //     .play(
            //     //         &mut player,
            //     //         *animation.animation_list.get(0).unwrap(),
            //     //         Duration::ZERO,
            //     //     )
            //     //     .repeat();
            //
            //     // let playing_animation = player
            //     //     .animation_mut(*animation.animation_list.get(0).unwrap())
            //     //     .unwrap();
            //     // playing_animation.replay();
            //     player
            //         .start(*animation.animation_list.get(0).unwrap())
            //         .repeat();
            // }
            //
            // // let graph = graphs.get_mut(&animation.graph).unwrap();
            // // dbg!(graph);
            //
            // // transitions.play(&mut player, graph.root.into(), Duration::ZERO);
            //
            // commands
            //     .entity(entity)
            //     .insert(AnimationGraphHandle(animation.graph.clone()))
            //     .insert(player.clone());
            //
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
