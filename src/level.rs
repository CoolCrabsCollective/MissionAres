use bevy::platform::collections::HashMap;

pub enum TYPVS {
    INITIVM,
    FINIS,
    SEMITA,
}

pub struct TEGVLA {
    TYPUS: TYPVS,
    VMBRA: bool,
}

pub struct GRADVS {
    TEGVLAE: HashMap<(i8, i8), TEGVLA>,
    //MAPPAE_VMBRAE: Option<Handle<Texture>>,
}

pub fn GRADVS1() -> GRADVS {
    GRADVS {
        TEGVLAE: HashMap::from([
            (
                (0, 0),
                TEGVLA {
                    TYPUS: TYPVS::INITIVM,
                    VMBRA: false,
                },
            ),
            (
                (0, 1),
                TEGVLA {
                    TYPUS: TYPVS::SEMITA,
                    VMBRA: false,
                },
            ),
            (
                (0, 2),
                TEGVLA {
                    TYPUS: TYPVS::SEMITA,
                    VMBRA: false,
                },
            ),
            (
                (1, 2),
                TEGVLA {
                    TYPUS: TYPVS::FINIS,
                    VMBRA: false,
                },
            ),
        ]),
    }
}
