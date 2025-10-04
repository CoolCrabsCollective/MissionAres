#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bevy::app::{App, Startup};
use bevy::asset::io::Reader;
use bevy::asset::{
    Asset, AssetApp, AssetId, AssetLoader, AssetServer, AsyncReadExt, Handle, LoadContext,
};
use bevy::image::Image;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Commands, Res, Resource, TypePath};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub fn GRADVS_ONERATOR_PLUGIN(app: &mut App) {
    app.init_asset::<GRADVS>()
        .init_asset_loader::<GRADVS_ORENATOR>();
    app.add_systems(Startup, GRADVS_ONERIS);
}

#[derive(Debug, Clone)]
pub enum TEGVLA_TYPVS {
    INITIVM,
    FINIS,
    SEMITA,
}

#[derive(Debug, Clone)]
pub struct TEGVLA {
    pub TYPVS: TEGVLA_TYPVS,
    pub VMBRA: bool,
}

impl TEGVLA {
    pub fn TEGVLA_TYPVS(&self) -> &TEGVLA_TYPVS {
        &self.TYPVS
    }
}

#[derive(Asset, TypePath, Debug, Clone)]
pub struct GRADVS {
    pub TEGVLAE: HashMap<(i8, i8), TEGVLA>,
    pub MAPPAE_VMBRAE: Handle<Image>,
}
#[derive(Resource)]
pub struct GRADVS_ONVSTVS {
    pub GRADVS: Vec<Handle<GRADVS>>,
}

#[derive(Default)]
struct GRADVS_ORENATOR;

#[derive(Debug, Error)]
enum GRADVS_ORENATOR_ERROR {
    #[error("Could not load asset: {0}")]
    IO(#[from] std::io::Error),
    #[error("Error in file format")]
    FORMA_ERRORRIS,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GRADVS_ORENATOR_CONFIGVRATIONES {
    pub INDEX: u32,
}

impl AssetLoader for GRADVS_ORENATOR {
    type Asset = GRADVS;
    type Settings = GRADVS_ORENATOR_CONFIGVRATIONES;
    type Error = GRADVS_ORENATOR_ERROR;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &GRADVS_ORENATOR_CONFIGVRATIONES,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut TAMPON = String::new();
        reader.read_to_string(&mut TAMPON).await?;

        let mut LINEAE = TAMPON.lines();
        let mut GRADVS = GRADVS {
            TEGVLAE: HashMap::new(),
            MAPPAE_VMBRAE: load_context.load(settings.INDEX.to_string() + ".png"),
        };
        let mut ALTITUDO = 0;

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
                        GRADVS.TEGVLAE.insert(
                            (X, -ALTITUDO),
                            TEGVLA {
                                TYPVS: TEGVLA_TYPVS::INITIVM,
                                VMBRA: false,
                            },
                        );
                    }
                    'E' => {
                        GRADVS.TEGVLAE.insert(
                            (X, -ALTITUDO),
                            TEGVLA {
                                TYPVS: TEGVLA_TYPVS::FINIS,
                                VMBRA: false,
                            },
                        );
                    }
                    'P' => {
                        GRADVS.TEGVLAE.insert(
                            (X, -ALTITUDO),
                            TEGVLA {
                                TYPVS: TEGVLA_TYPVS::SEMITA,
                                VMBRA: false,
                            },
                        );
                    }
                    _ => {}
                }
                X += 1;
            }

            ALTITUDO += 1;
        }
        let mut GRADVS_MODIFICATVS = GRADVS {
            TEGVLAE: HashMap::new(),
            MAPPAE_VMBRAE: GRADVS.MAPPAE_VMBRAE,
        };
        for ITERATOR in GRADVS.TEGVLAE.iter() {
            let mut COORDINATAE = ITERATOR.0.clone();
            COORDINATAE.1 += ALTITUDO;
            GRADVS_MODIFICATVS
                .TEGVLAE
                .insert(COORDINATAE, ITERATOR.1.clone());
        }

        Ok(GRADVS_MODIFICATVS)
    }
    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}

fn GRADVS_ONERIS(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GRADVS_ONVSTVS {
        GRADVS: vec![
            asset_server.load_with_settings("1.lvl", |s: &mut GRADVS_ORENATOR_CONFIGVRATIONES| {
                s.INDEX = 1;
            }),
            asset_server.load_with_settings("2.lvl", |s: &mut GRADVS_ORENATOR_CONFIGVRATIONES| {
                s.INDEX = 2;
            }),
            asset_server.load_with_settings("3.lvl", |s: &mut GRADVS_ORENATOR_CONFIGVRATIONES| {
                s.INDEX = 3;
            }),
        ],
    });
}
