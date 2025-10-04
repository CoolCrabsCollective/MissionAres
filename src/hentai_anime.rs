use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Default)]
pub struct Animation {
    pub animation_list: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

pub fn setup_hentai_anime_repeat_all_anime<'a>(
    mut player_query: Query<(Entity, &mut AnimationPlayer, &mut Animation), Added<AnimationPlayer>>,
    // target: Entity
) {
    for (entity, mut player, animation) in player_query.iter_mut() {
        // if entity == target {
        for hentai in animation.animation_list.iter() {
            player.play(hentai.clone()).repeat();
            // }
        }
    }
}