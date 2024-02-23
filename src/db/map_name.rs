pub mod maps {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;

    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum MapName {
        RoundtableHold, Limgrave, StrandedGraveyard, Stormhill, WeepingPeninsula,
        StormveilCastle, LiurniaOfTheLakes, BellumHighway, RuinStrewnPrecipice, MoonlightAltar,
        AcademyOfRayaLucaria, AltusPlateau, MtGelmir, CapitalOutskirts, VolcanoManor,
        LeyndellRoyalCapital, SubterraneanShunningGrounds, LeyndellAshenCapital, StonePlatform, Caelid,
        SwampOfAeonia, GreyollsDragonbarrow, ForbiddenLands, MountaintopsOfTheGiants, FlamePeak,
        ConsecratedSnowfield, MiquellasHaligtree, ElphaelBraceOfTheHaligtree, AinselRiver, AinselRiverMain,
        LakeOfRot, NokronEternalCity, MohgwynPalace, SiofraRiver, DeeprootDepths, CrumblingFarumAzula,
    }

    pub static MAP_NAME: Lazy<Mutex<HashMap<MapName, &str>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (MapName::RoundtableHold, "Table of Lost Grace / Roundtable Hold"),
            (MapName::Limgrave, "Limgrave"),
            (MapName::StrandedGraveyard, "Stranded Graveyard"),
            (MapName::Stormhill, "Stormhill"),
            (MapName::WeepingPeninsula, "Weeping Peninsula"),
            (MapName::StormveilCastle, "Stormveil Castle"),
            (MapName::LiurniaOfTheLakes, "Liurnia of the Lakes"),
            (MapName::BellumHighway, "Bellum Highway"),
            (MapName::RuinStrewnPrecipice, "Ruin-Strewn Precipice"),
            (MapName::MoonlightAltar, "Moonlight Altar"),
            (MapName::AcademyOfRayaLucaria, "Academy of Raya Lucaria"),
            (MapName::AltusPlateau, "Altus Plateau"),
            (MapName::MtGelmir, "Mt. Gelmir"),
            (MapName::CapitalOutskirts, "Capital Outskirts"),
            (MapName::VolcanoManor, "Volcano Manor"),
            (MapName::LeyndellRoyalCapital, "Leyndell, Royal Capital"),
            (MapName::SubterraneanShunningGrounds, "Subterranean Shunning-Grounds"),
            (MapName::LeyndellAshenCapital, "Leyndell, Ashen Capital"),
            (MapName::StonePlatform, "Stone Platform"),
            (MapName::Caelid, "Caelid"),
            (MapName::SwampOfAeonia, "Swamp of Aeonia"),
            (MapName::GreyollsDragonbarrow, "Greyoll's Dragonbarrow"),
            (MapName::ForbiddenLands, "Forbidden Lands"),
            (MapName::MountaintopsOfTheGiants, "Mountaintops of the Giants"),
            (MapName::FlamePeak, "Flame Peak"),
            (MapName::ConsecratedSnowfield, "Consecrated Snowfield"),
            (MapName::MiquellasHaligtree, "Miquella's Haligtree"),
            (MapName::ElphaelBraceOfTheHaligtree, "Elphael, Brace of the Haligtree"),
            (MapName::AinselRiver, "Ainsel River"),
            (MapName::AinselRiverMain, "Ainsel River Main"),
            (MapName::LakeOfRot, "Lake of Rot"),
            (MapName::NokronEternalCity, "Nokron, Eternal City"),
            (MapName::MohgwynPalace, "Mohgwyn Palace"),
            (MapName::SiofraRiver, "Siofra River"),
            (MapName::DeeprootDepths, "Deeproot Depths"),
            (MapName::CrumblingFarumAzula, "Crumbling Farum Azula"),
        ]))
    });
}