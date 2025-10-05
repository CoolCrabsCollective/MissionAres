use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Default)]
pub struct Animation {
    pub animation_list: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

pub struct HentaiAnimePlugin;

impl Plugin for HentaiAnimePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, setup_hentai_anime_repeat_all_anime);
    }
}

pub fn setup_hentai_anime_repeat_all_anime(
    mut player_query: Query<(&mut AnimationPlayer, &mut Animation), Added<AnimationPlayer>>,
) {
    for (mut player, animation) in player_query.iter_mut() {
        for hentai in &animation.animation_list {
            player.play(hentai.clone()).repeat();
        }
    }
}
