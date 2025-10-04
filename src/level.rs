use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::platform::collections::HashMap;
use bevy::prelude::TypePath;
use thiserror::Error;

#[derive(Debug)]
pub enum TEGVLA_TYPVS {
    INITIVM,
    FINIS,
    SEMITA,
}

#[derive(Debug)]
pub struct TEGVLA {
    TYPVS: TEGVLA_TYPVS,
    VMBRA: bool,
}

impl TEGVLA {
    pub fn TEGVLA_TYPVS(&self) -> &TEGVLA_TYPVS {
        &self.TYPVS
    }
}

#[derive(Asset, TypePath, Debug)]
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

        while true {
            let LINEA = LINEAE.next();
            //if LINEA.
        }

        Ok(GRADVS)
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
