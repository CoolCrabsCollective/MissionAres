//! English wrapper around the Latin-named level system.
//! This module provides a sane API while keeping the Latin naming contained in level.rs

use crate::level;
use bevy::app::App;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::system::Commands;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Res, Resource};

/// Type alias for the level asset storage (hides the Latin type name)
pub type LevelAssetStorage = Assets<level::GRADVS>;

/// Initializes the level loading system
pub fn level_plugin(app: &mut App) {
    use bevy::app::Startup;

    level::GRADVS_ONERATOR_PLUGIN(app);
    app.add_systems(Startup, initialize_level_assets);
}

/// Type of tile in the level
#[derive(Debug, Clone)]
pub enum TileType {
    Start,
    End,
    Path,
}

impl From<&level::TEGVLA_TYPVS> for TileType {
    fn from(value: &level::TEGVLA_TYPVS) -> Self {
        match value {
            level::TEGVLA_TYPVS::INITIVM => TileType::Start,
            level::TEGVLA_TYPVS::FINIS => TileType::End,
            level::TEGVLA_TYPVS::SEMITA => TileType::Path,
        }
    }
}

/// A single tile in the level
#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    #[allow(dead_code)]
    pub shadow: bool,
}

impl From<&level::TEGVLA> for Tile {
    fn from(value: &level::TEGVLA) -> Self {
        Tile {
            tile_type: TileType::from(&value.TYPVS),
            shadow: value.VMBRA,
        }
    }
}

/// A level containing tiles at grid positions
#[derive(Debug, Clone)]
pub struct Level {
    tiles: HashMap<(i8, i8), Tile>,
}

impl Level {
    /// Get all tiles in the level
    pub fn tiles(&self) -> &HashMap<(i8, i8), Tile> {
        &self.tiles
    }

    /// Get a specific tile at a position
    #[allow(dead_code)]
    pub fn get_tile(&self, x: i8, z: i8) -> Option<&Tile> {
        self.tiles.get(&(x, z))
    }

    /// Create a Level from the internal Latin representation
    pub fn from_internal(gradvs: &level::GRADVS) -> Self {
        let tiles = gradvs
            .TEGVLAE
            .iter()
            .map(|(pos, tile)| (*pos, Tile::from(tile)))
            .collect();

        Level { tiles }
    }
}

/// Handle to a level asset
#[derive(Clone, Debug)]
pub struct LevelHandle {
    handle: Handle<level::GRADVS>,
}

impl LevelHandle {
    /// Create a new level handle from the internal handle
    pub fn from_internal(handle: Handle<level::GRADVS>) -> Self {
        LevelHandle { handle }
    }

    /// Get the internal handle (for use with asset system)
    pub(crate) fn internal(&self) -> &Handle<level::GRADVS> {
        &self.handle
    }
}

/// Resource containing handles to all loaded levels
#[derive(Resource)]
pub struct LevelAssets {
    levels: Vec<LevelHandle>,
}

impl LevelAssets {
    /// Get a level handle by index
    pub fn get(&self, index: usize) -> Option<&LevelHandle> {
        self.levels.get(index)
    }

    /// Get all level handles
    #[allow(dead_code)]
    pub fn all(&self) -> &[LevelHandle] {
        &self.levels
    }
}

/// System to initialize level assets from the internal Latin resource
pub fn initialize_level_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LevelAssets {
        levels: vec![
            LevelHandle::from_internal(asset_server.load("1.lvl")),
            LevelHandle::from_internal(asset_server.load("2.lvl")),
            LevelHandle::from_internal(asset_server.load("3.lvl")),
        ],
    });
}

/// Helper to load a level from the asset storage
pub fn get_level(handle: &LevelHandle, assets: &LevelAssetStorage) -> Option<Level> {
    assets.get(handle.internal()).map(Level::from_internal)
}
