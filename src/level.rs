#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::platform::collections::HashMap;
use bevy::prelude::TypePath;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum TEGVLA_TYPVS {
    INITIVM,
    FINIS,
    SEMITA,
}

#[derive(Debug, Clone)]
pub struct TEGVLA {
    TYPVS: TEGVLA_TYPVS,
    VMBRA: bool,
}

impl TEGVLA {
    pub fn TEGVLA_TYPVS(&self) -> &TEGVLA_TYPVS {
        &self.TYPVS
    }
}

#[derive(Asset, TypePath, Debug, Clone)]
pub struct GRADVS {
    TEGVLAE: HashMap<(i8, i8), TEGVLA>,
    //MAPPAE_VMBRAE: Option<Handle<Texture>>,
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

impl AssetLoader for GRADVS_ORENATOR {
    type Asset = GRADVS;
    type Settings = ();
    type Error = GRADVS_ORENATOR_ERROR;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut TAMPON = String::new();
        reader.read_to_string(&mut TAMPON).await?;

        let mut LINEAE = TAMPON.lines();
        let mut GRADVS = GRADVS {
            TEGVLAE: HashMap::new(),
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

impl GRADVS {
    pub fn TEGVLAE(&self) -> &HashMap<(i8, i8), TEGVLA> {
        &self.TEGVLAE
    }
}

pub fn GRADVS1() -> GRADVS {
    GRADVS {
        TEGVLAE: HashMap::from([
            (
                (0, 0),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::INITIVM,
                    VMBRA: false,
                },
            ),
            (
                (0, 1),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: false,
                },
            ),
            (
                (0, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: false,
                },
            ),
            (
                (1, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::FINIS,
                    VMBRA: false,
                },
            ),
        ]),
    }
}

pub fn GRADVS2() -> GRADVS {
    GRADVS {
        TEGVLAE: HashMap::from([
            (
                (0, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::INITIVM,
                    VMBRA: false,
                },
            ),
            (
                (1, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (2, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (2, 1),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (2, 0),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: false,
                },
            ),
            (
                (3, 0),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (4, 0),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: false,
                },
            ),
            (
                (4, 1),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (4, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (5, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::SEMITA,
                    VMBRA: true,
                },
            ),
            (
                (6, 2),
                TEGVLA {
                    TYPVS: TEGVLA_TYPVS::FINIS,
                    VMBRA: true,
                },
            ),
        ]),
    }
}
