use bevy::platform::collections::HashMap;

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

#[derive(Debug)]
pub struct GRADVS {
    TEGVLAE: HashMap<(i8, i8), TEGVLA>,
    //MAPPAE_VMBRAE: Option<Handle<Texture>>,
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
