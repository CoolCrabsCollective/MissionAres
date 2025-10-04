use bevy::platform::collections::HashMap;

pub enum TileType {
    Start,
    End,
    Path,
}

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
                    tile_type: TileType::Start,
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
                    tile_type: TileType::End,
                    shadow: false,
                },
            ),
        ]),
    }
}

pub fn level2() -> Level {
    Level {
        tiles: HashMap::from([
            (
                (0, 2),
                Tile {
                    tile_type: TileType::Start,
                    shadow: false,
                },
            ),
            (
                (1, 2),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (2, 2),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (2, 1),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (2, 0),
                Tile {
                    tile_type: TileType::Path,
                    shadow: false,
                },
            ),
            (
                (3, 0),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (4, 0),
                Tile {
                    tile_type: TileType::Path,
                    shadow: false,
                },
            ),
            (
                (4, 1),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (4, 2),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (5, 2),
                Tile {
                    tile_type: TileType::Path,
                    shadow: true,
                },
            ),
            (
                (6, 2),
                Tile {
                    tile_type: TileType::End,
                    shadow: true,
                },
            ),
        ]),
    }
}
