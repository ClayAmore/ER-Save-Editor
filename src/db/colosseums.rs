pub mod colosseums {
    use once_cell::sync::Lazy;
    use std::{collections::HashMap, sync::Mutex};

    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Colosseum {
        Royal,
        Caelid,
        Limgrave,
    }
    pub static COLOSSEUMS: Lazy<Mutex<HashMap<Colosseum, (u32, &str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (Colosseum::Royal, (60370, "Royal Colosseum")),
            (Colosseum::Caelid, (60350, "Caelid Colosseum")),
            (Colosseum::Limgrave, (60360, "Limgrave Colosseum")),
        ]))
    });
}
