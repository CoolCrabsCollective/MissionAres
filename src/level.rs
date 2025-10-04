use bevy::platform::collections::HashMap;

// tile type
pub enum TileType {
    // initial tile
    Initial,
    // finish tile
    Finish,
    // semi transparent tile
    Path,
}

// tile
pub struct Tile {
    tile_type: TileType,
    shadow: bool,
}

pub struct Level {
    tiles: HashMap<(i8, i8), Tile>,
    //shadow_map: Option<Handle<Texture>>,
}

pub fn level1() -> Level {
    Level {
        tiles: HashMap::from([
            (
                (0, 0),
                Tile {
                    tile_type: TileType::Initial,
                    shadow: false,
                },
            ),
            (
                (0, 1),
                Tile {
                    tile_type: TileType::Path,
                    shadow: false,
                },
            ),
            (
                (0, 2),
                Tile {
                    tile_type: TileType::Path,
                    shadow: false,
                },
            ),
            (
                (1, 2),
                Tile {
                    tile_type: TileType::Finish,
                    shadow: false,
                },
            ),
        ]),
    }
}
