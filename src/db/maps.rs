pub mod maps {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;
    
    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Map {
        SE,
        NW,
        SW,
        Center,
        LimgraveEast,
        WeepingPeninsula,
        LimgraveWest,
        N,
        NE,
        LiurniaWest,
        LiurniaNorth,
        LiurniaEast,
        LeyndellRoyalCapital,
        AltusPlateur,
        MtGelmir,
        Dragonbarrow,
        Caelid,
        MountaintopsoftheGiantsNorth,
        MountaintopsoftheGiantsEast,
        MountaintopsoftheGiantsWest,
        SiofraRiver,
        MohgwynPalace,
        LakeofRot,
        AinselRiver,
        DeeproothDepths,
        StormfootCatacombs,
        FringefolkHeroCave,
        ShowUnderground,
    }

    pub static MAPS: Lazy<Mutex<HashMap<Map,(u32, &str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (Map::SE,(62007,"SE")),
            (Map::NW,(62006,"NW")),
            (Map::SW,(62005,"SW")),
            (Map::Center,(62004,"Center")),
            (Map::LimgraveEast,(62012,"Limgrave, East")),
            (Map::WeepingPeninsula,(62011,"Weeping Peninsula")),
            (Map::LimgraveWest,(62010,"Limgrave, West")),
            (Map::N,(62009,"N")),
            (Map::NE,(62008,"NE")),
            (Map::LiurniaWest,(62022,"Liurnia, West")),
            (Map::LiurniaNorth,(62021,"Liurnia, North")),
            (Map::LiurniaEast,(62020,"Liurnia, East")),
            (Map::LeyndellRoyalCapital,(62031,"Leyndell, Royal Capital")),
            (Map::AltusPlateur,(62030,"Altus Plateur")),
            (Map::MtGelmir,(62032,"Mt. Gelmir")),
            (Map::Dragonbarrow,(62041,"Dragonbarrow")),
            (Map::Caelid,(62040,"Caelid")),
            (Map::MountaintopsoftheGiantsNorth,(62052,"Mountaintops of the Giants, North")),
            (Map::MountaintopsoftheGiantsEast,(62051,"Mountaintops of the Giants, East")),
            (Map::MountaintopsoftheGiantsWest,(62050,"Mountaintops of the Giants, West")),
            (Map::SiofraRiver,(62063,"Siofra River")),
            (Map::MohgwynPalace,(62062,"Mohgwyn Palace")),
            (Map::LakeofRot,(62061,"Lake of Rot")),
            (Map::AinselRiver,(62060,"Ainsel River")),
            (Map::DeeproothDepths,(62064,"Deeprooth Depths")),
            (Map::StormfootCatacombs,(62103,"Stormfoot Catacombs")),
            (Map::FringefolkHeroCave,(62102,"Fringefolk Hero's Cave")),
            (Map::ShowUnderground,(82001,"Show underground")),
        ]))
    });
}