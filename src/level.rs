#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bevy::app::{App, Startup, Update};
use bevy::asset::AssetEvent::{Added, LoadedWithDependencies};
use bevy::asset::io::Reader;
use bevy::asset::{
    Asset, AssetApp, AssetEvent, AssetId, AssetLoader, AssetServer, Assets, AsyncReadExt, Handle,
    LoadContext,
};
use bevy::image::Image;
use bevy::log;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Commands, EventReader, Res, ResMut, Resource, TypePath};
use bevy::render::render_resource::{Extent3d, TextureDimension};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cmp::max;
use thiserror::Error;

use crate::level_spawner::LEVEL_SHADOW_ALPHA_MASK;

pub fn GRADVS_ONERATOR_PLUGIN(app: &mut App) {
    app.init_asset::<GRADVM>()
        .init_asset_loader::<GRADVM_ORENATOR>();
    app.add_systems(Startup, GRADVS_ONERIS);
    app.add_systems(Update, UMBRAE_COLLOCATOR);
}

// tile
#[derive(Debug, Clone)]
pub struct TEGVLA {
    pub TYPVS: TEGVLA_TYPVS, // tile type
    pub VMBRA: bool,         // umbra (shadow)
}

// tile type (tegula typus)
#[derive(Debug, Clone)]
pub enum TEGVLA_TYPVS {
    INITIVM, // initial
    FINIS,   // finish
    SEMITA,  // path
}

impl TEGVLA {
    pub fn TEGVLA_TYPVS(&self) -> &TEGVLA_TYPVS {
        &self.TYPVS
    }
}

// level (grade -> gradus)
#[derive(Asset, TypePath, Debug, Clone)]
pub struct GRADVM {
    pub TEGLVAE: HashMap<(i8, i8), TEGVLA>, // tiles
    pub MAPPAE_VREMBRAE: Handle<Image>,     // shadow map
    pub LATIVIDO: i8,                       // width
    pub ALTIVIDO: i8,                       // height
}

// loaded level
#[derive(Resource)]
pub struct GRADVM_ONVSTVS {
    // levels
    pub GRADVS: Vec<Handle<GRADVM>>,
}

// level loader
#[derive(Default)]
struct GRADVM_ORENATOR;

// level loader error
#[derive(Debug, Error)]
enum GRADVM_ORENATOR_ERROR {
    #[error("Could not load asset: {0}")]
    IO(#[from] std::io::Error),
    #[error("Error in file format")]
    FORMA_ERRORRIS,
}

// level loader settings
#[derive(Serialize, Deserialize, Default)]
pub struct GRADVM_ORENATOR_CONFIGVRATIONES {
    pub INDEX: u32,
}

impl AssetLoader for GRADVM_ORENATOR {
    type Asset = GRADVM;
    type Settings = GRADVM_ORENATOR_CONFIGVRATIONES;
    type Error = GRADVM_ORENATOR_ERROR;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &GRADVM_ORENATOR_CONFIGVRATIONES,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut TAMPON = String::new();
        reader.read_to_string(&mut TAMPON).await?;

        let mut LINEAE = TAMPON.lines();
        let mut GRADVS = GRADVM {
            TEGLVAE: HashMap::new(),
            MAPPAE_VREMBRAE: load_context.load(settings.INDEX.to_string() + ".png"),
            ALTIVIDO: 0,
            LATIVIDO: 0,
        };

        loop {
            let LINEA = LINEAE.next();
            if LINEA.is_none() {
                break;
            }

            let SERIES_CHARACTERVM = LINEA.unwrap();
            let mut X = 0;
            for ITERATOR in SERIES_CHARACTERVM.chars().into_iter() {
                match ITERATOR {
                    'S' => {
                        log::error!(
                            "{}: S at X, Y = {}, {}",
                            settings.INDEX.to_string(),
                            X,
                            GRADVS.ALTIVIDO
                        );
                        GRADVS.TEGLVAE.insert(
                            (X, -GRADVS.ALTIVIDO),
                            TEGVLA {
                                TYPVS: TEGVLA_TYPVS::INITIVM,
                                VMBRA: false,
                            },
                        );
                    }
                    'E' => {
                        log::error!(
                            "{}: E at X, Y = {}, {}",
                            settings.INDEX.to_string(),
                            X,
                            GRADVS.ALTIVIDO
                        );
                        GRADVS.TEGLVAE.insert(
                            (X, -GRADVS.ALTIVIDO),
                            TEGVLA {
                                TYPVS: TEGVLA_TYPVS::FINIS,
                                VMBRA: false,
                            },
                        );
                    }
                    'P' => {
                        log::error!(
                            "{}: P at X, Y = {}, {}",
                            settings.INDEX.to_string(),
                            X,
                            GRADVS.ALTIVIDO
                        );
                        GRADVS.TEGLVAE.insert(
                            (X, -GRADVS.ALTIVIDO),
                            TEGVLA {
                                TYPVS: TEGVLA_TYPVS::SEMITA,
                                VMBRA: false,
                            },
                        );
                    }
                    '\n' => {
                        X -= 1;
                    }
                    _ => {}
                }
                X += 1;
                GRADVS.LATIVIDO = max(X, GRADVS.LATIVIDO);
            }

            GRADVS.ALTIVIDO += 1;
        }
        let mut GRADVS_MODIFICATVS = GRADVM {
            TEGLVAE: HashMap::new(),
            MAPPAE_VREMBRAE: GRADVS.MAPPAE_VREMBRAE,
            LATIVIDO: GRADVS.LATIVIDO,
            ALTIVIDO: GRADVS.ALTIVIDO,
        };
        for ITERATOR in GRADVS.TEGLVAE.iter() {
            let mut COORDINATAE = ITERATOR.0.clone();
            COORDINATAE.1 += GRADVS.ALTIVIDO;
            GRADVS_MODIFICATVS
                .TEGLVAE
                .insert(COORDINATAE, ITERATOR.1.clone());
        }

        Ok(GRADVS_MODIFICATVS)
    }
    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}

fn GRADVS_ONERIS(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GRADVM_ONVSTVS {
        GRADVS: vec![
            asset_server.load_with_settings("1.lvl", |s: &mut GRADVM_ORENATOR_CONFIGVRATIONES| {
                s.INDEX = 1;
            }),
            asset_server.load_with_settings("2.lvl", |s: &mut GRADVM_ORENATOR_CONFIGVRATIONES| {
                s.INDEX = 2;
            }),
            asset_server.load_with_settings("3.lvl", |s: &mut GRADVM_ORENATOR_CONFIGVRATIONES| {
                s.INDEX = 3;
            }),
        ],
    });
}

fn UMBRAE_COLLOCATOR(
    mut EVENTVS: EventReader<AssetEvent<GRADVM>>,
    IMAGINES: Res<Assets<Image>>,
    mut GRADVS: ResMut<Assets<GRADVM>>,
) {
    for EVENTVM in EVENTVS.read() {
        if let LoadedWithDependencies { id } = EVENTVM {
            let mut GRADVM = GRADVS.get_mut(id.clone());
            if GRADVM.is_none() {
                continue;
            }

            let GRADVM = GRADVM.unwrap();
            let IMAGINE = IMAGINES.get(&GRADVM.MAPPAE_VREMBRAE);
            if IMAGINE.is_none() {
                log::error!("TABVLA VMBRAE NON ONERATA PRO GRADV");
                continue;
            }

            let DIMENSIO = IMAGINE.unwrap().texture_descriptor.size;
            let DATA = &IMAGINE.unwrap().data;

            if DATA.is_none() {
                continue;
            }

            let DATA = DATA.as_ref().unwrap();

            for TEGVLA in GRADVM.TEGLVAE.iter_mut() {
                let PIXEL_X =
                    (TEGVLA.0.0 as f32 + 0.5) / GRADVM.LATIVIDO as f32 * DIMENSIO.width as f32;
                let PIXEL_Y = DIMENSIO.height as f32
                    - ((TEGVLA.0.1 as f32 - 0.5) / GRADVM.ALTIVIDO as f32 * DIMENSIO.height as f32);
                log::error!(
                    "TEGVLA: {:?}, PIXEL_X: {}, PIXEL_Y: {}",
                    TEGVLA,
                    PIXEL_X,
                    PIXEL_Y
                );
                let INDEX = f32::round(PIXEL_Y) as usize * DIMENSIO.width as usize
                    + f32::round(PIXEL_X) as usize;
                let COLOR = DATA.get(INDEX * 4 + 3);
                if let Some(ALPHA) = COLOR {
                    TEGVLA.1.VMBRA = *ALPHA > (255.0 * LEVEL_SHADOW_ALPHA_MASK) as u8;
                }
            }
        }
    }
}
