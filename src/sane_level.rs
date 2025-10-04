use bevy::platform::collections::HashMap;

use crate::level::{GRADVS, GRADVS1, GRADVS2, TEGVLA, TEGVLA_TYPVS};

pub type Level = GRADVS;

pub type Tile = TEGVLA;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Start,
    End,
    Path,
}

pub trait TileExt {
    fn tile_type(&self) -> TileType;
}

impl TileExt for Tile {
    fn tile_type(&self) -> TileType {
        match self.TEGVLA_TYPVS() {
            TEGVLA_TYPVS::INITIVM => TileType::Start,
            TEGVLA_TYPVS::FINIS => TileType::End,
            TEGVLA_TYPVS::SEMITA => TileType::Path,
        }
    }
}

pub trait LevelExt {
    fn tiles(&self) -> &HashMap<(i8, i8), Tile>;
}

impl LevelExt for Level {
    fn tiles(&self) -> &HashMap<(i8, i8), Tile> {
        self.TEGVLAE()
    }
}

pub fn level_1() -> Level {
    GRADVS1()
}

pub fn level_2() -> Level {
    GRADVS2()
}
