use std::{collections::HashMap, sync::OnceLock};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Colosseum {
    Royal,
    Caelid,
    Limgrave,
}

impl Colosseum {
    #[rustfmt::skip]
    pub fn colusseums() -> &'static HashMap<Colosseum, (u32, &'static str)> {
        static COLUSSEUMS: OnceLock<HashMap<Colosseum, (u32, &'static str)>> = OnceLock::new();

        COLUSSEUMS.get_or_init(||
            HashMap::from([
                (Colosseum::Royal,(60370,"Royal Colosseum")),
                (Colosseum::Caelid,(60350,"Caelid Colosseum")),
                (Colosseum::Limgrave,(60360,"Limgrave Colosseum")),
            ])
        )
    }
}
