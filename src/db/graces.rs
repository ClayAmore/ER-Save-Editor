pub mod maps {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;
    use crate::db::map_name::map_name::MapName;

    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Grace {
        // Table of Lost Grace / Roundtable Hold
        RoundtableHold,

        //Limgrave
        AgheelLakeNorth,
        AgheelLakeSouth,
        ChurchofDragonCommunion,
        ChurchofElleh,
        CoastalCave,
        FortHaightWest,
        Gatefront,
        GrovesideCave,
        HighroadCave,
        LimgraveArtistShack,
        LimgraveTunnels,
        MistwoodOutskirts,
        MurkwaterCatacombs,
        MurkwaterCave,
        MurkwaterCoast,
        SeasideRuins,
        StormfootCatacombs,
        SummonwaterVillageOutskirts,
        TheFirstStep,
        ThirdChurchofMarika,
        WaypointRuinsCellar,
        
        // Stranded Graveyard
        CaveofKnowledge,
        StrandedGraveyard,

        // Stormhill
        CastlewardTunnel,
        DeathtouchedCatacombs,
        DivineTowerofLimgrave,
        LimgraveTowerBridge,
        MargittheFellOmen,
        Saintsbridge,
        StormhillShack,
        WarmastersShack,

        // WeepingPeninsula
        AilingVillageOutskirts,
        BehindTheCastle,
        BesidetheCraterPockedGlade,
        BesidetheRampartGaol,
        BridgeofSacrifice,
        CastleMorneLift,
        CastleMorneRampart,
        ChurchofPilgrimage,
        EarthboreCave,
        FourthChurchofMarika,
        ImpalersCatacombs,
        LimgraveIsolatedMerchantsShack,
        MorneMoangrave,
        MorneTunnel,
        SouthoftheLookoutTower,
        Tombsward,
        TombswardCatacombs,
        TombswardCave,

        // StormveilCastle
        GatesideChamber,
        GodricktheGrafted,
        LiftsideChamber,
        RampartTower,
        SecludedCell,
        StormveilCliffside,
        StormveilMainGate,

        // LiurniaoftheLakes
        AcademyCrystalCave,
        AcademyGateTown,
        BehindCariaManor,
        BlackKnifeCatacombs,
        BoilprawnShack,
        ChurchofVows,
        CliffbottomCatacombs,
        ConvertedTower,
        CrystallineWoods,
        DivineTowerofLiurnia,
        EasternLiurniaLakeShore,
        EasternTableland,
        EastGateBridgeTrestle,
        FallenRuinsoftheLake,
        FollyontheLake,
        FootoftheFourBelfries,
        GateTownBridge,
        GateTownNorth,
        Jarburg,
        LakeFacingCliffs,
        LakesideCrystalCave,
        LaskyarRuins,
        LiurniaHighwayNorth,
        LiurniaHighwaySouth,
        LiurniaLakeShore,
        LiurniaoftheLakesArtistsShack,
        LiurniaTowerBridge,
        MainAcademyGate,
        MainCariaManorGate,
        ManorLowerLevel,
        ManorUpperLevel,
        MausoleumCompound,
        NorthernLiurniaLakeShore,
        RannisChamber,
        RannisRise,
        RavineVeiledVillage,
        RayaLucariaCrystalTunnel,
        RevengersShack,
        RoadsEndCatacombs,
        RoadtotheManor,
        RoyalMoongazingGrounds,
        RuinedLabyrinth,
        ScenicIsle,
        SlumberingWolfsShack,
        SorcerersIsle,
        SouthRayaLucariaGate,
        StillwaterCave,
        StudyHallEntrance,
        TempleQuarter,
        TheFourBelfries,
        TheRavine,
        VillageoftheAlbinaurics,

        // BellumHighway
        BellumChurch,
        ChurchofInhibition,
        EastRayaLucariaGate,
        FrenziedFlameVillageOutskirts,
        GrandLiftofDectus,

        // RuinStrewnPrecipice
        MagmaWyrm,
        RuinStrewnPrecipice,
        RuinStrewnPrecipiceOverlook,

        // MoonlightAltar
        AltarSouth,
        CathedralofManusCeles,
        MoonlightAltar,

        // AcademyofRayaLucaria
        ChurchoftheCuckoo,
        DebateParlour,
        RayaLucariaGrandLibrary,
        SchoolhouseClassroom,

        // AltusPlateau
        AbandonedCoffin,
        AltusHighwayJunction,
        AltusPlateau,
        AltusTunnel,
        BowerofBounty,
        CastellansHall,
        ErdtreeGazingHill,
        ForestSpanningGreatbridge,
        OldAltusTunnel,
        PerfumersGrotto,
        RampartsidePath,
        RoadofIniquitySidePath,
        SagesCave,
        SaintedHerosGrave,
        ShadedCastleInnerGate,
        ShadedCastleRamparts,
        UnsightlyCatacombs,
        WindmillHeights,
        WindmillVillage,

        // MtGelmir
        BridgeofIniquity,
        CraftsmansShack,
        FirstMtGelmirCampsite,
        GelmirHerosGrave,
        NinthMtGelmirCampsite,
        PrimevalSorcererAzur,
        RoadofIniquity,
        SeethewaterCave,
        SeethewaterRiver,
        SeethewaterTerminus,
        VolcanoCave,
        WyndhamCatacombs,

        // CapitalOutskirts
        AurizaSideTomb,
        AuziraHerosGrave,
        CapitalRampart,
        DivineTowerofWestAltus,
        DivineTowerofWestAltusGate,
        HermitMerchantsShack,
        MinorEerdtreeChurch,
        OuterWallBattleground,
        OuterWallPhantomTree,
        SealedTunnel,

        // VolcanoManor
        AbductorVirgin,
        AudiencePathway,
        GuestHall,
        PrisonTownChurch,
        RykardLordofBlasphemy,
        SubterraneanInquisitionChamber,
        TempleofEiglay,
        VolcanoManor,

        // LeyndellRoyalCapital
        AvenueBalcony,
        DivineBridge,
        EastCapitalRampart,
        EldenThrone,
        ErdtreeSanctuary,
        FortifiedManorFirstFloor,
        LowerCapitalChurch,
        QueensBedchamber,
        WestCapitalRampart,

        // SubterraneanShunningGrounds
        CathedraloftheForsaken,
        ForsakenDepths,
        FrenziedFlameProscription,
        LeyndellCatacombs,
        UndergroundRoadside,

        // LeyndellAshenCapital
        LeyendellCapitalofAsh,
        LeyndellAshenCapitalDivineBridge,
        LeyndellAshenCapitalEastCapitalRampart,
        LeyndellAshenCapitalEldenThrone,
        LeyndellAshenCapitalErdtreeSanctuary,
        LeyndellAshenCapitalQueensBedchamber,

        // StonePlatfrom
        FracturedMarika,

        // Caelid
        AbandonedCave,
        CaelemRuins,
        CaelidCatacombs,
        CaelidHighwaySouth,
        CathedralofDragonCommunion,
        ChairCryptofSellia,
        ChamberOutsidethePlaza,
        ChurchofthePlague,
        DeepSiofraWell,
        FortGaelNorth,
        GaelTunnel,
        GaolCave,
        ImpassableGreatbridge,
        MinorEerdtreeCatacombs,
        RearGaelTunnelEntrance,
        RedmaneCastlePlaza,
        RotviewBalcony,
        SelliaBackstreets,
        SelliaCrystalTunnel,
        SelliaUnderStair,
        SmolderingChurch,
        SmolderingWall,
        SouthernAeoniaSwampBank,
        StarscourgeRadahn,
        WarDeadCatacombs,

        // SwampofAeonia
        AeoniaSwampShore,
        AstrayfromCaelidHighwayNorth,
        HeartofAeonia,
        InnerAeonia,

        // GreyollsDragonbarrow
        BestialSanctum,
        DivineTowerofCaelidBasement,
        DivineTowerofCaelidCenter,
        DragonbarrowCave,
        DragonbarrowFork,
        DragonbarrowWest,
        FarumGreatbridge,
        FortFaroth,
        GreyollsDragonbarrowIsolatedMerchantsShack,
        IsolatedDivineTower,
        LennesRise,
        SelliaHideaway,

        // ForbidenLands
        DivineToweroftheEastAltus,
        DivineToweroftheEastAltusGate,
        ForbiddenLands,
        GrandLiftofRold,
        HiddenPathtotheHaligtree,

        // MountaintopsoftheGiants
        AncientSnowValleyRuins,
        CastleSolMainGate,
        CastleSolRooftop,
        ChurchoftheEclipse,
        FirstChurchofMarika,
        FreezingLake,
        SnowValleyRuinsOverlook,
        SpiritcallersCave,
        WhiteridgeRoad,
        ZamorRuins,

        // FlamePeak
        ChurchofRepose,
        FireGiant,
        FootoftheForge,
        ForgeoftheGiants,
        GiantConqueringHerosGrave,
        GiantsGravepost,
        GiantsMountaintopCatacombs,

        // ConsecratedSnowfield
        ApostateDerelict,
        CaveoftheForlorn,
        ConsecratedSnowfield,
        ConsecratedSnowfieldCatacombs,
        InnerConsecratedSnowfield,
        OrdinaLiturgicalTown,
        YeloughAnixTunnel,

        // MiquellasHaligtree
        HaligtreeCanopy,
        HaligtreePromenade,
        HaligtreeTown,
        HaligtreeTownPlaza,

        // ElphaelBraceoftheHaligtree
        DrainageChannel,
        ElphaelInnerWall,
        HaligtreeRoots,
        MaleniaGodessofRot,
        PrayerRoom,

        // CrumblingFarumAzula
        BesidethegreatBridge,
        CrumblingBeastGrave,
        CrumblingBeastGraveDepths,
        DragonlordPlacidusax,
        DragonTemple,
        DragonTempleAltar,
        DragonTempleLift,
        DragonTempleRooftop,
        DragonTempleTransept,
        MalikeththeBlackBlade,
        TempestFacingBalcony,

        // AinselRiver
        AinselRiverDownstream,
        AinselRiverSluiceGate,
        AinselRiverWellDepths,
        AstelNaturalbornoftheVoid,
        DragonkinSoldierofNokstella,

        // AinselRiverMain
        AinselRiverMain,
        NokstellaEternalCity,
        NokstellaWaterfallBasin,

        // LakeOfRot
        GrandCloister,
        LakeofRotShoreside,

        // NokronEternalCity
        AncestralWoods,
        AqueductFacingCliffs,
        GreatWaterfallBasin,
        MimicTear,
        NightsSacredGround,
        NokronEternalCity,

        // SiofraRiver
        BelowtheWell,
        SiofraRiverBank,
        SiofraRiverWellDepths,
        WorshippersWoods,

        // MohgwynPalace
        CocoonoftheEmpyrean,
        DynastyMausoleumEntrance,
        DynastyMausoleumMidpoint,
        PalaceApproachLedgeRoad,

        // DeeprootDepths
        AcrosstheRoots,
        DeeprootDepths,
        GreatWaterfallCrest,
        PrinceofDeathsThrone,
        RootFacingCliffs,
        TheNamelessEternalCity,
    }
    
    pub static GRACES: Lazy<Mutex<HashMap<Grace, (MapName, u32, &str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            // Table of Lost Grace / Roundtable Hold
            (Grace::RoundtableHold, (MapName::RoundtableHold, 71190,"Table of Lost Grace / Roundtable Hold")),
        
            // Limgrave
            (Grace::AgheelLakeNorth, (MapName::Limgrave, 76108,"Agheel Lake North")),
            (Grace::AgheelLakeSouth, (MapName::Limgrave, 76106,"Agheel Lake South")),
            (Grace::ChurchofDragonCommunion, (MapName::Limgrave, 76110,"Church of Dragon Communion")),
            (Grace::ChurchofElleh, (MapName::Limgrave, 76100,"Church of Elleh")),
            (Grace::CoastalCave, (MapName::Limgrave, 73115,"Coastal Cave")),
            (Grace::FortHaightWest, (MapName::Limgrave, 76105,"Fort Haight West")),
            (Grace::Gatefront, (MapName::Limgrave, 76111,"Gatefront")),
            (Grace::GrovesideCave, (MapName::Limgrave, 73103,"Groveside Cave")),
            (Grace::HighroadCave, (MapName::Limgrave, 73117,"Highroad Cave")),
            (Grace::LimgraveArtistShack, (MapName::Limgrave, 76103,"Artist's Shack (Limgrave)")),
            (Grace::LimgraveTunnels, (MapName::Limgrave, 73201,"Limgrave Tunnels")),
            (Grace::MistwoodOutskirts, (MapName::Limgrave, 76114,"Mistwood Outskirts")),
            (Grace::MurkwaterCatacombs, (MapName::Limgrave, 73004,"Murkwater Catacombs")),
            (Grace::MurkwaterCave, (MapName::Limgrave, 73100,"Murkwater Cave")),
            (Grace::MurkwaterCoast, (MapName::Limgrave, 76116,"Murkwater Coast")),
            (Grace::SeasideRuins, (MapName::Limgrave, 76113,"Seaside Ruins")),
            (Grace::StormfootCatacombs, (MapName::Limgrave, 73002,"Stormfoot Catacombs")),
            (Grace::SummonwaterVillageOutskirts, (MapName::Limgrave, 76119,"Summonwater Village Outskirts")),
            (Grace::TheFirstStep, (MapName::Limgrave, 76101,"The First Step")),
            (Grace::ThirdChurchofMarika, (MapName::Limgrave, 76104,"Third Church of Marika")),
            (Grace::WaypointRuinsCellar, (MapName::Limgrave, 76120,"Waypoint Ruins Cellar")),

            // Stranded Graveyard
            (Grace::CaveofKnowledge, (MapName::StrandedGraveyard, 71800,"Cave of Knowledge")),
            (Grace::StrandedGraveyard, (MapName::StrandedGraveyard, 71801,"Stranded Graveyard")),

            // Stormhill
            (Grace::CastlewardTunnel, (MapName::Stormhill, 71002,"Castleward Tunnel")),
            (Grace::DeathtouchedCatacombs, (MapName::Stormhill, 73011,"Deathtouched Catacombs")),
            (Grace::DivineTowerofLimgrave, (MapName::Stormhill, 73412,"Divine Tower of Limgrave")),
            (Grace::LimgraveTowerBridge, (MapName::Stormhill, 73410,"Limgrave Tower Bridge")),
            (Grace::MargittheFellOmen, (MapName::Stormhill, 71001,"Margit, the Fell Omen")),
            (Grace::Saintsbridge, (MapName::Stormhill, 76117,"Saintsbridge")),
            (Grace::StormhillShack, (MapName::Stormhill, 76102,"Stormhill Shack")),
            (Grace::WarmastersShack, (MapName::Stormhill, 76118,"Warmaster's Shack")),

            // Weeping Peninsula
            (Grace::AilingVillageOutskirts, (MapName::WeepingPeninsula, 76154,"Ailing Village Outskirts")),
            (Grace::BehindTheCastle, (MapName::WeepingPeninsula, 76159,"Behind The Castle")),
            (Grace::BesidetheCraterPockedGlade, (MapName::WeepingPeninsula, 76155,"Beside the Crater-Pocked Glade")),
            (Grace::BesidetheRampartGaol, (MapName::WeepingPeninsula, 76160,"Beside the Rampart Gaol")),
            (Grace::BridgeofSacrifice, (MapName::WeepingPeninsula, 76157,"Bridge of Sacrifice")),
            (Grace::CastleMorneLift, (MapName::WeepingPeninsula, 76158,"Castle Morne Lift")),
            (Grace::CastleMorneRampart, (MapName::WeepingPeninsula, 76151,"Castle Morne Rampart")),
            (Grace::ChurchofPilgrimage, (MapName::WeepingPeninsula, 76150,"Church of Pilgrimage")),
            (Grace::EarthboreCave, (MapName::WeepingPeninsula, 73101,"Earthbore Cave")),
            (Grace::FourthChurchofMarika, (MapName::WeepingPeninsula, 76162,"Fourth Church of Marika")),
            (Grace::ImpalersCatacombs, (MapName::WeepingPeninsula, 73001,"Impaler's Catacombs")),
            (Grace::LimgraveIsolatedMerchantsShack, (MapName::WeepingPeninsula, 76156,"Isolated Merchant's Shack (Limgrave)")),
            (Grace::MorneMoangrave, (MapName::WeepingPeninsula, 76161,"Morne Moangrave")),
            (Grace::MorneTunnel, (MapName::WeepingPeninsula, 73200,"Morne Tunnel")),
            (Grace::SouthoftheLookoutTower, (MapName::WeepingPeninsula, 76153,"South of the Lookout Tower")),
            (Grace::Tombsward, (MapName::WeepingPeninsula, 76152,"Tombsward")),
            (Grace::TombswardCatacombs, (MapName::WeepingPeninsula, 73000,"Tombsward Catacombs")),
            (Grace::TombswardCave, (MapName::WeepingPeninsula, 73102,"Tombsward Cave")),

            // Stormveil Castle
            (Grace::GatesideChamber, (MapName::StormveilCastle, 71003,"Gateside Chamber")),
            (Grace::GodricktheGrafted, (MapName::StormveilCastle, 71000,"Godrick the Grafted")),
            (Grace::LiftsideChamber, (MapName::StormveilCastle, 71006,"Liftside Chamber")),
            (Grace::RampartTower, (MapName::StormveilCastle, 71005,"Rampart Tower")),
            (Grace::SecludedCell, (MapName::StormveilCastle, 71007,"Secluded Cell")),
            (Grace::StormveilCliffside, (MapName::StormveilCastle, 71004,"Stormveil Cliffside")),
            (Grace::StormveilMainGate, (MapName::StormveilCastle, 71008,"Stormveil Main Gate")),

            // Liurnia of the Lakes
            (Grace::AcademyCrystalCave, (MapName::LiurniaOfTheLakes, 73106,"Academy Crystal Cave")),
            (Grace::AcademyGateTown, (MapName::LiurniaOfTheLakes, 76204,"Academy Gate Town")),
            (Grace::BehindCariaManor, (MapName::LiurniaOfTheLakes, 76238,"Behind Caria Manor")),
            (Grace::BlackKnifeCatacombs, (MapName::LiurniaOfTheLakes, 73005,"Black Knife Catacombs")),
            (Grace::BoilprawnShack, (MapName::LiurniaOfTheLakes, 76216,"Boilprawn Shack")),
            (Grace::ChurchofVows, (MapName::LiurniaOfTheLakes, 76224,"Church of Vows")),
            (Grace::CliffbottomCatacombs, (MapName::LiurniaOfTheLakes, 73006,"Cliffbottom Catacombs")),
            (Grace::ConvertedTower, (MapName::LiurniaOfTheLakes, 76237,"Converted Tower")),
            (Grace::CrystallineWoods, (MapName::LiurniaOfTheLakes, 76243,"Crystalline Woods")),
            (Grace::DivineTowerofLiurnia, (MapName::LiurniaOfTheLakes, 73422,"Divine Tower of Liurnia")),
            (Grace::EasternLiurniaLakeShore, (MapName::LiurniaOfTheLakes, 76223,"Eastern Liurnia Lake Shore")),
            (Grace::EasternTableland, (MapName::LiurniaOfTheLakes, 76234,"Eastern Tableland")),
            (Grace::EastGateBridgeTrestle, (MapName::LiurniaOfTheLakes, 76242,"East Gate Bridge Trestle")),
            (Grace::FallenRuinsoftheLake, (MapName::LiurniaOfTheLakes, 76236,"Fallen Ruins of the Lake")),
            (Grace::FollyontheLake, (MapName::LiurniaOfTheLakes, 76219,"Folly on the Lake")),
            (Grace::FootoftheFourBelfries, (MapName::LiurniaOfTheLakes, 76210,"Foot of the Four Belfries")),
            (Grace::GateTownBridge, (MapName::LiurniaOfTheLakes, 76222,"Gate Town Bridge")),
            (Grace::GateTownNorth, (MapName::LiurniaOfTheLakes, 76233,"Gate Town North")),
            (Grace::Jarburg, (MapName::LiurniaOfTheLakes, 76245,"Jarburg")),
            (Grace::LakeFacingCliffs, (MapName::LiurniaOfTheLakes, 76200,"Lake-Facing Cliffs")),
            (Grace::LakesideCrystalCave, (MapName::LiurniaOfTheLakes, 73105,"Lakeside Crystal Cave")),
            (Grace::LaskyarRuins, (MapName::LiurniaOfTheLakes, 76202,"Laskyar Ruins")),
            (Grace::LiurniaHighwayNorth, (MapName::LiurniaOfTheLakes, 76221,"Liurnia Highway North")),
            (Grace::LiurniaHighwaySouth, (MapName::LiurniaOfTheLakes, 76244,"Liurnia Highway South")),
            (Grace::LiurniaLakeShore, (MapName::LiurniaOfTheLakes, 76201,"Liurnia Lake Shore")),
            (Grace::LiurniaoftheLakesArtistsShack, (MapName::LiurniaOfTheLakes, 76217,"Artist's Shack (Liurnia of the Lakes)")),
            (Grace::LiurniaTowerBridge, (MapName::LiurniaOfTheLakes, 73421,"Liurnia Tower Bridge")),
            (Grace::MainAcademyGate, (MapName::LiurniaOfTheLakes, 76206,"Main Academy Gate")),
            (Grace::MainCariaManorGate, (MapName::LiurniaOfTheLakes, 76214,"Main Caria Manor Gate")),
            (Grace::ManorLowerLevel, (MapName::LiurniaOfTheLakes, 76231,"Manor Lower Level")),
            (Grace::ManorUpperLevel, (MapName::LiurniaOfTheLakes, 76230,"Manor Upper Level")),
            (Grace::MausoleumCompound, (MapName::LiurniaOfTheLakes, 76226,"Mausoleum Compound")),
            (Grace::NorthernLiurniaLakeShore, (MapName::LiurniaOfTheLakes, 76212,"Northern Liurnia Lake Shore")),
            (Grace::RannisChamber, (MapName::LiurniaOfTheLakes, 76247,"Ranni's Chamber")),
            (Grace::RannisRise, (MapName::LiurniaOfTheLakes, 76228,"Ranni's Rise")),
            (Grace::RavineVeiledVillage, (MapName::LiurniaOfTheLakes, 76229,"Ravine-Veiled Village")),
            (Grace::RayaLucariaCrystalTunnel, (MapName::LiurniaOfTheLakes, 73202,"Raya Lucaria Crystal Tunnel")),
            (Grace::RevengersShack, (MapName::LiurniaOfTheLakes, 76218,"Revenger's Shack")),
            (Grace::RoadsEndCatacombs, (MapName::LiurniaOfTheLakes, 73003,"Road's End Catacombs")),
            (Grace::RoadtotheManor, (MapName::LiurniaOfTheLakes, 76213,"Road to the Manor")),
            (Grace::RoyalMoongazingGrounds, (MapName::LiurniaOfTheLakes, 76232,"Royal Moongazing Grounds")),
            (Grace::RuinedLabyrinth, (MapName::LiurniaOfTheLakes, 76225,"Ruined Labyrinth")),
            (Grace::ScenicIsle, (MapName::LiurniaOfTheLakes, 76203,"Scenic Isle")),
            (Grace::SlumberingWolfsShack, (MapName::LiurniaOfTheLakes, 76215,"Slumbering Wolf's Shack")),
            (Grace::SorcerersIsle, (MapName::LiurniaOfTheLakes, 76211,"Sorcerer's Isle")),
            (Grace::SouthRayaLucariaGate, (MapName::LiurniaOfTheLakes, 76205,"South Raya Lucaria Gate")),
            (Grace::StillwaterCave, (MapName::LiurniaOfTheLakes, 73104,"Stillwater Cave")),
            (Grace::StudyHallEntrance, (MapName::LiurniaOfTheLakes, 73420,"Study Hall Entrance")),
            (Grace::TempleQuarter, (MapName::LiurniaOfTheLakes, 76241,"Temple Quarter")),
            (Grace::TheFourBelfries, (MapName::LiurniaOfTheLakes, 76227,"The Four Belfries")),
            (Grace::TheRavine, (MapName::LiurniaOfTheLakes, 76235,"The Ravine")),
            (Grace::VillageoftheAlbinaurics, (MapName::LiurniaOfTheLakes, 76220,"Village of the Albinaurics")),
            
            // Bellum Highway
            (Grace::BellumChurch, (MapName::BellumHighway, 76208, "Bellum Church")),
            (Grace::ChurchofInhibition, (MapName::BellumHighway, 76240, "Church of Inhibition")),
            (Grace::EastRayaLucariaGate, (MapName::BellumHighway, 76207, "East Raya Lucaria Gate")),
            (Grace::FrenziedFlameVillageOutskirts, (MapName::BellumHighway, 76239, "Frenzied Flame Village Outskirts")),
            (Grace::GrandLiftofDectus, (MapName::BellumHighway, 76209, "Grand Lift of Dectus")),

            // Ruin-Strewn Precipice
            (Grace::MagmaWyrm, (MapName::RuinStrewnPrecipice, 73900, "Magma Wyrm")),
            (Grace::RuinStrewnPrecipice, (MapName::RuinStrewnPrecipice, 73901, "Ruin-Strewn Precipice")),
            (Grace::RuinStrewnPrecipiceOverlook, (MapName::RuinStrewnPrecipice, 73902, "Ruin-Strewn Precipice Overlook")),

            // Moonlight Altar
            (Grace::AltarSouth, (MapName::MoonlightAltar, 76252, "Altar South")),
            (Grace::CathedralofManusCeles, (MapName::MoonlightAltar, 76251, "Cathedral of Manus Celes")),
            (Grace::MoonlightAltar, (MapName::MoonlightAltar, 76250, "Moonlight Altar")),

            // Academy of Raya Lucaria
            (Grace::ChurchoftheCuckoo, (MapName::AcademyOfRayaLucaria, 71402, "Church of the Cuckoo")),
            (Grace::DebateParlour, (MapName::AcademyOfRayaLucaria, 71401, "Debate Parlour")),
            (Grace::RayaLucariaGrandLibrary, (MapName::AcademyOfRayaLucaria, 71400, "Raya Lucaria Grand Library")),
            (Grace::SchoolhouseClassroom, (MapName::AcademyOfRayaLucaria, 71403, "Schoolhouse Classroom")),

            // Altus Plateau
            (Grace::AbandonedCoffin, (MapName::AltusPlateau, 76300, "Abandoned Coffin")),
            (Grace::AltusHighwayJunction, (MapName::AltusPlateau, 76303, "Altus Highway Junction")),
            (Grace::AltusPlateau, (MapName::AltusPlateau, 76301, "Altus Plateau")),
            (Grace::AltusTunnel, (MapName::AltusPlateau, 73205, "Altus Tunnel")),
            (Grace::BowerofBounty, (MapName::AltusPlateau, 76306, "Bower of Bounty")),
            (Grace::CastellansHall, (MapName::AltusPlateau, 76322, "Castellan's Hall")),
            (Grace::ErdtreeGazingHill, (MapName::AltusPlateau, 76302, "Erdtree-Gazing Hill")),
            (Grace::ForestSpanningGreatbridge, (MapName::AltusPlateau, 76304, "Forest-Spanning Greatbridge")),
            (Grace::OldAltusTunnel, (MapName::AltusPlateau, 73204, "Old Altus Tunnel")),
            (Grace::PerfumersGrotto, (MapName::AltusPlateau, 73118, "Perfumer's Grotto")),
            (Grace::RampartsidePath, (MapName::AltusPlateau, 76305, "Rampartside Path")),
            (Grace::RoadofIniquitySidePath, (MapName::AltusPlateau, 76307, "Road of Iniquity Side Path")),
            (Grace::SagesCave, (MapName::AltusPlateau, 73119, "Sage's Cave")),
            (Grace::SaintedHerosGrave, (MapName::AltusPlateau, 73008, "Sainted Hero's Grave")),
            (Grace::ShadedCastleInnerGate, (MapName::AltusPlateau, 76321, "Shaded Castle Inner Gate")),
            (Grace::ShadedCastleRamparts, (MapName::AltusPlateau, 76320, "Shaded Castle Ramparts")),
            (Grace::UnsightlyCatacombs, (MapName::AltusPlateau, 73012, "Unsightly Catacombs")),
            (Grace::WindmillHeights, (MapName::AltusPlateau, 76313, "Windmill Heights")),
            (Grace::WindmillVillage, (MapName::AltusPlateau, 76308, "Windmill Village")),

            // Mt. Gelmir
            (Grace::BridgeofIniquity, (MapName::MtGelmir, 76350, "Bridge of Iniquity")),
            (Grace::CraftsmansShack, (MapName::MtGelmir, 76356, "Craftsman's Shack")),
            (Grace::FirstMtGelmirCampsite, (MapName::MtGelmir, 76351, "First Mt. Gelmir Campsite")),
            (Grace::GelmirHerosGrave, (MapName::MtGelmir, 73009, "Gelmir Hero's Grave")),
            (Grace::NinthMtGelmirCampsite, (MapName::MtGelmir, 76352, "Ninth Mt. Gelmir Campsite")),
            (Grace::PrimevalSorcererAzur, (MapName::MtGelmir, 76357, "Primeval Sorcerer Azur")),
            (Grace::RoadofIniquity, (MapName::MtGelmir, 76353, "Road of Iniquity")),
            (Grace::SeethewaterCave, (MapName::MtGelmir, 73107, "Seethewater Cave")),
            (Grace::SeethewaterRiver, (MapName::MtGelmir, 76354, "Seethewater River")),
            (Grace::SeethewaterTerminus, (MapName::MtGelmir, 76355, "Seethewater Terminus")),
            (Grace::VolcanoCave, (MapName::MtGelmir, 73109, "Volcano Cave")),
            (Grace::WyndhamCatacombs, (MapName::MtGelmir, 73007, "Wyndham Catacombs")),

            // Capital Outskirts
            (Grace::AurizaSideTomb, (MapName::CapitalOutskirts, 73013, "Auriza Side Tomb")),
            (Grace::AuziraHerosGrave, (MapName::CapitalOutskirts, 73010, "Auzira Hero's Grave")),
            (Grace::CapitalRampart, (MapName::CapitalOutskirts, 76314, "Capital Rampart")),
            (Grace::DivineTowerofWestAltus, (MapName::CapitalOutskirts, 73430, "Divine Tower of West Altus")),
            (Grace::DivineTowerofWestAltusGate, (MapName::CapitalOutskirts, 73432, "Divine Tower of West Altus: Gate")),
            (Grace::HermitMerchantsShack, (MapName::CapitalOutskirts, 76311, "Hermit Merchant's Shack")),
            (Grace::MinorEerdtreeChurch, (MapName::CapitalOutskirts, 76310, "Minor Eerdtree Church")),
            (Grace::OuterWallBattleground, (MapName::CapitalOutskirts, 76312, "Outer Wall Battleground")),
            (Grace::OuterWallPhantomTree, (MapName::CapitalOutskirts, 76309, "Outer Wall Phantom Tree")),
            (Grace::SealedTunnel, (MapName::CapitalOutskirts, 73431, "Sealed Tunnel")),

            // Volcano Manor
            (Grace::AbductorVirgin, (MapName::VolcanoManor, 71606, "Abductor Virgin")),
            (Grace::AudiencePathway, (MapName::VolcanoManor, 71605, "Audience Pathway")),
            (Grace::GuestHall, (MapName::VolcanoManor, 71604, "Guest Hall")),
            (Grace::PrisonTownChurch, (MapName::VolcanoManor, 71603, "Prison Town Church")),
            (Grace::RykardLordofBlasphemy, (MapName::VolcanoManor, 71600, "Rykard, Lord of Blasphemy")),
            (Grace::SubterraneanInquisitionChamber, (MapName::VolcanoManor, 71607, "Subterranean Inquisition Chamber")),
            (Grace::TempleofEiglay, (MapName::VolcanoManor, 71601, "Temple of Eiglay")),
            (Grace::VolcanoManor, (MapName::VolcanoManor, 71602, "Volcano Manor")),

            // Leyndell, Royal Capital
            (Grace::AvenueBalcony, (MapName::LeyndellRoyalCapital, 71104, "Avenue Balcony")),
            (Grace::DivineBridge, (MapName::LeyndellRoyalCapital, 71109, "Divine Bridge")),
            (Grace::EastCapitalRampart, (MapName::LeyndellRoyalCapital, 71102, "East Capital Rampart")),
            (Grace::EldenThrone, (MapName::LeyndellRoyalCapital, 71100, "Elden Throne")),
            (Grace::ErdtreeSanctuary, (MapName::LeyndellRoyalCapital, 71101, "Erdtree Sanctuary")),
            (Grace::FortifiedManorFirstFloor, (MapName::LeyndellRoyalCapital, 71108, "Fortified Manor, First Floor")),
            (Grace::LowerCapitalChurch, (MapName::LeyndellRoyalCapital, 71103, "Lower Capital Church")),
            (Grace::QueensBedchamber, (MapName::LeyndellRoyalCapital, 71107, "Queen's Bedchamber")),
            (Grace::WestCapitalRampart, (MapName::LeyndellRoyalCapital, 71105, "West Capital Rampart")),

            // Subterranean Shunning-Grounds
            (Grace::CathedraloftheForsaken, (MapName::SubterraneanShunningGrounds, 73500, "Cathedral of the Forsaken")),
            (Grace::ForsakenDepths, (MapName::SubterraneanShunningGrounds, 73502, "Forsaken Depths")),
            (Grace::FrenziedFlameProscription, (MapName::SubterraneanShunningGrounds, 73504, "Frenzied Flame Proscription")),
            (Grace::LeyndellCatacombs, (MapName::SubterraneanShunningGrounds, 73503, "Leyndell Catacombs")),
            (Grace::UndergroundRoadside, (MapName::SubterraneanShunningGrounds, 73501, "Underground Roadside")),

            // Leyndell, Ashen Capital
            (Grace::LeyendellCapitalofAsh, (MapName::LeyndellAshenCapital, 71123, "Leyendell, Capital of Ash")),
            (Grace::LeyndellAshenCapitalDivineBridge, (MapName::LeyndellAshenCapital, 71125, "Divine Bridge")),
            (Grace::LeyndellAshenCapitalEastCapitalRampart, (MapName::LeyndellAshenCapital, 71122, "East Capital Rampart")),
            (Grace::LeyndellAshenCapitalEldenThrone, (MapName::LeyndellAshenCapital, 71120, "Elden Throne")),
            (Grace::LeyndellAshenCapitalErdtreeSanctuary, (MapName::LeyndellAshenCapital, 71121, "Erdtree Sanctuary")),
            (Grace::LeyndellAshenCapitalQueensBedchamber, (MapName::LeyndellAshenCapital, 71124, "Queen's Bedchamber")),

            // Stone Platfrom
            (Grace::FracturedMarika, (MapName::StonePlatform, 71900, "Fractured Marika")),

            // Caelid
            (Grace::AbandonedCave, (MapName::Caelid, 73120, "Abandoned Cave")),
            (Grace::CaelemRuins, (MapName::Caelid, 76403, "Caelem Ruins")),
            (Grace::CaelidCatacombs, (MapName::Caelid, 73015, "Caelid Catacombs")),
            (Grace::CaelidHighwaySouth, (MapName::Caelid, 76405, "Caelid Highway South")),
            (Grace::CathedralofDragonCommunion, (MapName::Caelid, 76404, "Cathedral of Dragon Communion")),
            (Grace::ChairCryptofSellia, (MapName::Caelid, 76415, "Chair-Crypt of Sellia")),
            (Grace::ChamberOutsidethePlaza, (MapName::Caelid, 76420, "Chamber Outside the Plaza")),
            (Grace::ChurchofthePlague, (MapName::Caelid, 76418, "Church of the Plague")),
            (Grace::DeepSiofraWell, (MapName::Caelid, 76410, "Deep Siofra Well")),
            (Grace::FortGaelNorth, (MapName::Caelid, 76402, "Fort Gael North")),
            (Grace::GaelTunnel, (MapName::Caelid, 73207, "Gael Tunnel")),
            (Grace::GaolCave, (MapName::Caelid, 73121, "Gaol Cave")),
            (Grace::ImpassableGreatbridge, (MapName::Caelid, 76417, "Impassable Greatbridge")),
            (Grace::MinorEerdtreeCatacombs, (MapName::Caelid, 73014, "Minor Eerdtree Catacombs")),
            (Grace::RearGaelTunnelEntrance, (MapName::Caelid, 73257, "Rear Gael Tunnel Entrance")),
            (Grace::RedmaneCastlePlaza, (MapName::Caelid, 76419, "Redmane Castle Plaza")),
            (Grace::RotviewBalcony, (MapName::Caelid, 76401, "Rotview Balcony")),
            (Grace::SelliaBackstreets, (MapName::Caelid, 76414, "Sellia Backstreets")),
            (Grace::SelliaCrystalTunnel, (MapName::Caelid, 73208, "Sellia Crystal Tunnel")),
            (Grace::SelliaUnderStair, (MapName::Caelid, 76416, "Sellia Under-Stair")),
            (Grace::SmolderingChurch, (MapName::Caelid, 76400, "Smoldering Church")),
            (Grace::SmolderingWall, (MapName::Caelid, 76409, "Smoldering Wall")),
            (Grace::SouthernAeoniaSwampBank, (MapName::Caelid, 76411, "Southern Aeonia Swamp Bank")),
            (Grace::StarscourgeRadahn, (MapName::Caelid, 76422, "Starscourge Radahn")),
            (Grace::WarDeadCatacombs, (MapName::Caelid, 73016, "War-Dead Catacombs")),

            // Swamp of Aeonia
            (Grace::AeoniaSwampShore, (MapName::SwampOfAeonia, 76406, "Aeonia Swamp Shore")),
            (Grace::AstrayfromCaelidHighwayNorth, (MapName::SwampOfAeonia, 76407, "Astray from Caelid Highway North")),
            (Grace::HeartofAeonia, (MapName::SwampOfAeonia, 76412, "Heart of Aeonia")),
            (Grace::InnerAeonia, (MapName::SwampOfAeonia, 76413, "Inner Aeonia")),

            // Greyoll's Dragonbarrow
            (Grace::BestialSanctum, (MapName::GreyollsDragonbarrow, 76454, "Bestial Sanctum")),
            (Grace::DivineTowerofCaelidBasement, (MapName::GreyollsDragonbarrow, 73440, "Divine Tower of Caelid: Basement")),
            (Grace::DivineTowerofCaelidCenter, (MapName::GreyollsDragonbarrow, 73441, "Divine Tower of Caelid: Center")),
            (Grace::DragonbarrowCave, (MapName::GreyollsDragonbarrow, 73110, "Dragonbarrow Cave")),
            (Grace::DragonbarrowFork, (MapName::GreyollsDragonbarrow, 76452, "Dragonbarrow Fork")),
            (Grace::DragonbarrowWest, (MapName::GreyollsDragonbarrow, 76450, "Dragonbarrow West")),
            (Grace::FarumGreatbridge, (MapName::GreyollsDragonbarrow, 76456, "Farum Greatbridge")),
            (Grace::FortFaroth, (MapName::GreyollsDragonbarrow, 76453, "Fort Faroth")),
            (Grace::GreyollsDragonbarrowIsolatedMerchantsShack, (MapName::GreyollsDragonbarrow, 76451, "Isolated Merchant's Shack (Greyoll's Dragonbarrow)")),
            (Grace::IsolatedDivineTower, (MapName::GreyollsDragonbarrow, 73460, "Isolated Divine Tower")),
            (Grace::LennesRise, (MapName::GreyollsDragonbarrow, 76455, "Lenne's Rise")),
            (Grace::SelliaHideaway, (MapName::GreyollsDragonbarrow, 73111, "Sellia Hideaway")),

            // Forbiden Lands
            (Grace::DivineToweroftheEastAltus, (MapName::ForbiddenLands, 73451, "Divine Tower of the East Altus")),
            (Grace::DivineToweroftheEastAltusGate, (MapName::ForbiddenLands, 73450, "Divine Tower of the East Altus: Gate")),
            (Grace::ForbiddenLands, (MapName::ForbiddenLands, 76500, "Forbidden Lands")),
            (Grace::GrandLiftofRold, (MapName::ForbiddenLands, 76502, "Grand Lift of Rold")),
            (Grace::HiddenPathtotheHaligtree, (MapName::ForbiddenLands, 73020, "Hidden Path to the Haligtree")),

            // Mountaintops of the Giants
            (Grace::AncientSnowValleyRuins, (MapName::MountaintopsOfTheGiants, 76503, "Ancient Snow Valley Ruins")),
            (Grace::CastleSolMainGate, (MapName::MountaintopsOfTheGiants, 76522, "Castle Sol Main Gate")),
            (Grace::CastleSolRooftop, (MapName::MountaintopsOfTheGiants, 76524, "Castle Sol Rooftop")),
            (Grace::ChurchoftheEclipse, (MapName::MountaintopsOfTheGiants, 76523, "Church of the Eclipse")),
            (Grace::FirstChurchofMarika, (MapName::MountaintopsOfTheGiants, 76505, "First Church of Marika")),
            (Grace::FreezingLake, (MapName::MountaintopsOfTheGiants, 76504, "Freezing Lake")),
            (Grace::SnowValleyRuinsOverlook, (MapName::MountaintopsOfTheGiants, 76521, "Snow Valley Ruins Overlook")),
            (Grace::SpiritcallersCave, (MapName::MountaintopsOfTheGiants, 73122, "Spiritcaller's Cave")),
            (Grace::WhiteridgeRoad, (MapName::MountaintopsOfTheGiants, 76520, "Whiteridge Road")),
            (Grace::ZamorRuins, (MapName::MountaintopsOfTheGiants, 76501, "Zamor Ruins")),

            // Flame Peak
            (Grace::ChurchofRepose, (MapName::FlamePeak, 76507, "Church of Repose")),
            (Grace::FireGiant, (MapName::FlamePeak, 76509, "Fire Giant")),
            (Grace::FootoftheForge, (MapName::FlamePeak, 76508, "Foot of the Forge")),
            (Grace::ForgeoftheGiants, (MapName::FlamePeak, 76510, "Forge of the Giants")),
            (Grace::GiantConqueringHerosGrave, (MapName::FlamePeak, 73017, "Giant-Conquering Hero's Grave")),
            (Grace::GiantsGravepost, (MapName::FlamePeak, 76506, "Giant's Gravepost")),
            (Grace::GiantsMountaintopCatacombs, (MapName::FlamePeak, 73018, "Giant's Mountaintop Catacombs")),

            // Consecrated Snowfield
            (Grace::ApostateDerelict, (MapName::ConsecratedSnowfield, 76653, "Apostate Derelict")),
            (Grace::CaveoftheForlorn, (MapName::ConsecratedSnowfield, 73112, "Cave of the Forlorn")),
            (Grace::ConsecratedSnowfield, (MapName::ConsecratedSnowfield, 76550, "Consecrated Snowfield")),
            (Grace::ConsecratedSnowfieldCatacombs, (MapName::ConsecratedSnowfield, 73019, "Consecrated Snowfield Catacombs")),
            (Grace::InnerConsecratedSnowfield, (MapName::ConsecratedSnowfield, 76551, "Inner Consecrated Snowfield")),
            (Grace::OrdinaLiturgicalTown, (MapName::ConsecratedSnowfield, 76652, "Ordina, Liturgical Town")),
            (Grace::YeloughAnixTunnel, (MapName::ConsecratedSnowfield, 73211, "Yelough Anix Tunnel")),

            // Miquella's Haligtree
            (Grace::HaligtreeCanopy, (MapName::MiquellasHaligtree, 71506, "Haligtree Canopy")),
            (Grace::HaligtreePromenade, (MapName::MiquellasHaligtree, 71505, "Haligtree Promenade")),
            (Grace::HaligtreeTown, (MapName::MiquellasHaligtree, 71507, "Haligtree Town")),
            (Grace::HaligtreeTownPlaza, (MapName::MiquellasHaligtree, 71508, "Haligtree Town Plaza")),

            // Elphael, Brace of the Haligtree
            (Grace::DrainageChannel, (MapName::ElphaelBraceOfTheHaligtree, 71503, "Drainage Channel")),
            (Grace::ElphaelInnerWall, (MapName::ElphaelBraceOfTheHaligtree, 71502, "Elphael Inner Wall")),
            (Grace::HaligtreeRoots, (MapName::ElphaelBraceOfTheHaligtree, 71504, "Haligtree Roots")),
            (Grace::MaleniaGodessofRot, (MapName::ElphaelBraceOfTheHaligtree, 71500, "Malenia, Godess of Rot")),
            (Grace::PrayerRoom, (MapName::ElphaelBraceOfTheHaligtree, 71501, "Prayer Room")),

            // Crumbling Farum Azula
            (Grace::BesidethegreatBridge, (MapName::CrumblingFarumAzula, 71310, "Beside the great Bridge")),
            (Grace::CrumblingBeastGrave, (MapName::CrumblingFarumAzula, 71303, "Crumbling Beast Grave")),
            (Grace::CrumblingBeastGraveDepths, (MapName::CrumblingFarumAzula, 71304, "Crumbling Beast Grave Depths")),
            (Grace::DragonlordPlacidusax, (MapName::CrumblingFarumAzula, 71301, "Dragonlord Placidusax")),
            (Grace::DragonTemple, (MapName::CrumblingFarumAzula, 71306, "Dragon Temple")),
            (Grace::DragonTempleAltar, (MapName::CrumblingFarumAzula, 71302, "Dragon Temple Altar")),
            (Grace::DragonTempleLift, (MapName::CrumblingFarumAzula, 71308, "Dragon Temple Lift")),
            (Grace::DragonTempleRooftop, (MapName::CrumblingFarumAzula, 71309, "Dragon Temple Rooftop")),
            (Grace::DragonTempleTransept, (MapName::CrumblingFarumAzula, 71307, "Dragon Temple Transept")),
            (Grace::MalikeththeBlackBlade, (MapName::CrumblingFarumAzula, 71300, "Maliketh, the Black Blade")),
            (Grace::TempestFacingBalcony, (MapName::CrumblingFarumAzula, 71305, "Tempest-Facing Balcony")),

            // Ainsel River
            (Grace::AinselRiverDownstream, (MapName::AinselRiver, 71213, "Ainsel River Downstream")),
            (Grace::AinselRiverSluiceGate, (MapName::AinselRiver, 71212, "Ainsel River Sluice Gate")),
            (Grace::AinselRiverWellDepths, (MapName::AinselRiver, 71211, "Ainsel River Well Depths")),
            (Grace::AstelNaturalbornoftheVoid, (MapName::AinselRiver, 71240, "Astel, Naturalborn of the Void")),
            (Grace::DragonkinSoldierofNokstella, (MapName::AinselRiver, 71210, "Dragonkin Soldier of Nokstella")),

            // Ainsel River Main
            (Grace::AinselRiverMain, (MapName::AinselRiverMain, 71214, "Ainsel River Main")),
            (Grace::NokstellaEternalCity, (MapName::AinselRiverMain, 71215, "Nokstella, Eternal City")),
            (Grace::NokstellaWaterfallBasin, (MapName::AinselRiverMain, 71219, "Nokstella Waterfall Basin")),

            // Lake Of Rot
            (Grace::GrandCloister, (MapName::LakeOfRot, 71218, "Grand Cloister")),
            (Grace::LakeofRotShoreside, (MapName::LakeOfRot, 71216, "Lake of Rot Shoreside")),

            // Nokron, Eternal City
            (Grace::AncestralWoods, (MapName::NokronEternalCity, 71224, "Ancestral Woods")),
            (Grace::AqueductFacingCliffs, (MapName::NokronEternalCity, 71225, "Aqueduct-Facing Cliffs")),
            (Grace::GreatWaterfallBasin, (MapName::NokronEternalCity, 71220, "Great Waterfall Basin")),
            (Grace::MimicTear, (MapName::NokronEternalCity, 71221, "Mimic Tear")),
            (Grace::NightsSacredGround, (MapName::NokronEternalCity, 71226, "Night's Sacred Ground")),
            (Grace::NokronEternalCity, (MapName::NokronEternalCity, 71271, "Nokron, Eternal City")),

            // Siofra River
            (Grace::BelowtheWell, (MapName::SiofraRiver, 71227, "Below the Well")),
            (Grace::SiofraRiverBank, (MapName::SiofraRiver, 71222, "Siofra River Bank")),
            (Grace::SiofraRiverWellDepths, (MapName::SiofraRiver, 71270, "Siofra River Well Depths")),
            (Grace::WorshippersWoods, (MapName::SiofraRiver, 71223, "Worshippers' Woods")),

            // Mohgwyn Palace
            (Grace::CocoonoftheEmpyrean, (MapName::MohgwynPalace, 71250, "Cocoon of the Empyrean")),
            (Grace::DynastyMausoleumEntrance, (MapName::MohgwynPalace, 71252, "Dynasty Mausoleum Entrance")),
            (Grace::DynastyMausoleumMidpoint, (MapName::MohgwynPalace, 71253, "Dynasty Mausoleum Midpoint")),
            (Grace::PalaceApproachLedgeRoad, (MapName::MohgwynPalace, 71251, "Palace Approach Ledge-Road")),

            // Deeproot Depths
            (Grace::AcrosstheRoots, (MapName::DeeprootDepths, 71235, "Across the Roots")),
            (Grace::DeeprootDepths, (MapName::DeeprootDepths, 71233, "Deeproot Depths")),
            (Grace::GreatWaterfallCrest, (MapName::DeeprootDepths, 71232, "Great Waterfall Crest")),
            (Grace::PrinceofDeathsThrone, (MapName::DeeprootDepths, 71230, "Prince of Death's Throne")),
            (Grace::RootFacingCliffs, (MapName::DeeprootDepths, 71231, "Root-Facing Cliffs")),
            (Grace::TheNamelessEternalCity, (MapName::DeeprootDepths, 71234, "The Nameless Eternal City")),
        ]))
    });
}