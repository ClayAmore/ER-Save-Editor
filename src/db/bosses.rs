pub mod bosses {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;
    
    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Boss {
        // Limgrave,
        AlabasterLordEvergaol,
        BeastmanofFaramAzulaGrovesideCave,
        BellBearingHunterWarmastersShack,
        BlackKnifeAssassinDeathtouchedCatacombs,
        BloodhoundKnightDarriwilForlornHoundEvergaol,
        CrucibleKnightStormhillEvergaol,
        DeathbirdLimgrave,
        DemiHumanChiefsCoastalCave,
        ErdtreeBurialWatchdogStormfrontCatacombs,
        FlyingDragonAgheelLimgrave,
        GodricktheGrafted,
        GraftedScionChapelofAnticipation,
        GraveWardenDuelistMurkwaterCatacombs,
        GuardianGolemHighroadCave,
        MadPumpkinHeadWaypointRuins,
        MargittheFellOmen,
        NightsCavalryLimgrave,
        PatchesMurkwaterCave,
        SoldierofGodrick,
        StonediggerTrollLimgraveTunnels,
        TibiaMarinerSummonwaterVillage,
        TreeSentinelLimgrave,

        // Weeping Weeping Peninsula
        AncientHeroofZamorWeepingEvergaol,
        CemeteryShadeTombswardCatacombs,
        DeathbirdWeepingPeninsula,
        EdtreeBurialWatchdogImpalersCatacombs,
        ErdtreeAvatar,
        LeonineMisbegottenMorneMoangrave,
        MirandatheBlightedBloomTombswardCave,
        NightsCavalryWeepingPeninsula,
        RunebearEarthboreCave,

        // Liurnia of the Lakes
        AdanThiefofFireMalefactorsEvergaol,
        AlectoBlackKnifeRingleaderRingleadersEvergaol,
        BellBearingHunterChurchofVows,
        BlackKnifeAssassinBlackKnifeCatacombs,
        BloodhoundKnightLakesideCrystalCave,
        BolsCarianKnightCuckoosEvergaol,
        CemeteryShadeBlackKnifeCatacombs,
        RennalaQueenoftheFullMoon,

        // Caelid
        CleanrotKnightStillwaterCave,
        CommanderONeilSwampofAeonia,
        CrucibleKnightMisbegottenWarriorRedmaneCastle,
        CrystalianRayaLucariaCrystalTunnel,
        CrystalianSpearCrystalianStaffAcademyCrystalCave,
        DeathbirdLiurniaSouth,
        DeathRiteBirdCaelid,
        DeathRiteBirdLiurniaNorth,
        DecayingEkzykesCaelid,
        ElderDragonGreyollcanapparentlyspawninvisible,
        ErdtreeAvatarLiurniaNortheast,
        ErdtreeAvatarLiurniaSouthwest,
        ErdtreeBurialWatchdogCliffbottomCatacombs,
        ErdtreeBurialWatchdogsMinorErdtreeCatacombs,
        FallingstarBeastSelliaCrystalTunnel,
        FrenziedDuelistGaolCave,
        GlintstoneDragonAdulaMoonlightAltar,
        GlintstoneDragonSmaragLiurnia,
        MadPumpkinHeadsCaelemRuins,
        MagmaWyrmGaelTunnel,
        MagmaWyrmMakarRuinStrewnPrecipice,
        NightsCavalryCaelid,
        NightsCavalryLiurniaNorth,
        NightsCavalryLiurniaSouth,
        NoxSwordstressandNoxPriestSelliaTownofSorcery,
        OmenkillerVillageoftheAlbinaurics,
        OnyxLordRoyalGraveEvergaol,
        PutridAvatarCaelid,
        RedWolfofRadagonRayaLucaria,
        RoyalKnightLorettaCariaManor,
        RoyalRevenantKingsrealmRuins,
        SpiritcallerSnailRoadsEndCatacombs,

        // Dragonbarrow
        _CleanrotKnightsStillwaterCave, // TODO:: Why is this not used?
        BattlemageHuguesSelliaEvergaol,
        BeastmenofFaramAzulaDragonbarrowCave,
        BellBearingHunterIsolatedMerchantsShack,
        BlackBladeKindredBestialSanctum,
        FlyingDragonGreyllGreyollsDragonbarrow,
        GodskinApostleDivineTowerofCaelid,
        NightsCavalryDragonbarrow,
        PutridAvatarDragonbarrow,
        PutridCrystaliansSelliaHideaway,

        // Altus Plateau
        AncientDragonLansseaxAltusPlateau,
        AncientHeroofZamorSaintedHerosGrave,
        BlackKnifeAssassinSagesCave,
        BlackKnifeAssassinSaintedHerosGrave,
        CrystalianSpearandCrystalianRingbladeAltusTunnel,
        DemiHumanQueenGilikaLuxRuins,
        ElemeroftheBriarTheShadedCastle,
        ErdtreeBurialWatchdogWyndhamCatacombs,
        FallingstarBeastAltusPlateau,
        GodefroytheGraftedGoldenLineageEvergaol,
        GodskinApostleDominulaWindmillVillage,
        NecromancerGarrisSagesCave,
        NightsCavalryAltusPlateau,
        OmenkillerandMirandatheBlightedBloomPerfumersGrotto,
        PerfumerTriciaandMisbegottenWarriorUnsightlyCatacombs,
        SanguineNobleWrithebloodRuins,
        StonediggerTrollOldAltusTunnel,
        TibiaMarinerWyndhamRuins,
        TreeSentinelsAltusPlateau,

        // Capital Outskirts 
        BellBearingHunterHermitMerchantsShack,
        CrucibleKnightCrucibleKnightOrdovisAurizaHeroGrave,
        DeathbirdWarmastersShack,
        DraconicTreeSentinelCapitalOutskirts,
        FellTwinsDivineTowerofEastAltus,
        GraveWardenDuelistAurizaSideTomb,
        
        // Leyendell, Royal Capital
        GodfreyFirstEldenLordGoldenShade,
        MohgtheOmenCathedraloftheForsaken,
        MorgotttheOmenKing,

        // Mt. Gelmir
        AbductorVirginsMtGelmir,
        DemiHumanQueenMaggieHermitVillage,
        DemiHumanQueenMargotVolanoCave,
        FullGrownFallingstarBeastNinthMtGelmirCampsite,
        GodDevouringSerpentRykardLordofBlasphemy,
        GodskinNobleTempleofEiglay,
        KindredofRotSeethewaterCave,
        MagmaWyrmMtGelmir,
        RedWolfoftheChampionGelmirHerosGrave,
        UlceratedTreeSpiritMtGelmir,

        // Mountaintops of the Giants
        AncientHeroofZamorGiantConquringHerosGrave,
        BorealistheFreezingFogFreezingLake,
        CommanderNiallCastleSol,
        DeathRiteBirdMountaintopsoftheGiants,
        ErdtreeAvatarMountaintopsoftheGiants,
        FireGiant,
        GodskinApostleandGodskinNobleSpiritcallerCaveSpiritcallerSnail,
        UlceratedTreeSpiritGiantsMountaintopCatacombs,
        VykeKnightoftheRoundtableLordContendersEvergaol,
        
        // Crumbling Farum Azula
        DragonlordPlacidusax,
        GodskinDuo,
        MalekiththeBlackBlade,

        // Forbidden Lands
        BlackBladeKindredForbiddenLands,
        NightsCavalryForbiddenLands,
        StrayMimicTearHiddenPathtotheHaligtree,

        // Consecrated Snowfields
        AstelStarsofDarknessYeloughAnixTunnel,
        DeathRiteBirdConsecratedSnowfield,
        GreatWyrmTheodorixConsecratedSnowfield,
        MisbegottenCrusaderCaveoftheForlorn,
        NightsCavalryDuoConsecratedSnowfield,
        PutridAvatar,
        PutridGraveWardenDuelistConsecratedSnowfieldCatacombs,
    
        // Miquella's Haligtree
        LorettaKnightoftheHaligtree,
        MaleniaGoddessofRot,

        // Siofra River
        AncestorSpiritSiofraRiverBank,
        DragonkinSoldierSiofraRiverBank,
        MohgLordofBlood,

        // Ainsel River
        DragonkinSoldierofNokstellaAinselRiver,

        // Nokron Eternal City
        MimicTearNokronEternalCity,
        RegalAncestorSpiritAncestralWoods,
        ValiantGargoylesSiofraAqueduct,

        // Deeproot Depths
        CrucibleKnightSiluria,
        FiasChampions,
        LichdragonFortissax,

        // Lake of Rot
        AstelNaturalbornoftheVoid,
        DragonkinSoldierLakeofRot,

        // Leyendell, Ashen Capital
        GodfreyFirstEldenLordHoarahLoux,
        SirGideonOfnirtheAllKnowing,

        // Elden Throne
        RadagonoftheGoldenOrderEldenBeast,
    }
    pub static BOSSES: Lazy<Mutex<HashMap<Boss, (u32,&str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            // Limgrave
            (Boss::AlabasterLordEvergaol, (1036500800,"Alabaster Lord (Evergaol)")),
            (Boss::BeastmanofFaramAzulaGrovesideCave, (31030800,"Beastman of Faram Azula (Groveside Cave)")),
            (Boss::BellBearingHunterWarmastersShack, (1042380850,"Bell Bearing Hunter (Warmaster's Shack)")),
            (Boss::BlackKnifeAssassinDeathtouchedCatacombs, (30110800,"Black Knife Assassin (Deathtouched Catacombs)")),
            (Boss::BloodhoundKnightDarriwilForlornHoundEvergaol, (1044350800,"Bloodhound Knight Darriwil (Forlorn Hound Evergaol)")),
            (Boss::CrucibleKnightStormhillEvergaol, (1042370800,"Crucible Knight (Stormhill Evergaol)")),
            (Boss::DeathbirdLimgrave, (1042380800,"Deathbird (Limgrave)")),
            (Boss::DemiHumanChiefsCoastalCave, (31150800,"Demi-Human Chiefs (Coastal Cave)")),
            (Boss::ErdtreeBurialWatchdogStormfrontCatacombs, (30020800,"Erdtree Burial Watchdog (Stormfront Catacombs)")),
            (Boss::FlyingDragonAgheelLimgrave, (1043360800,"Flying Dragon Agheel (Limgrave)")),
            (Boss::GodricktheGrafted, (10000800,"Godrick the Grafted")),
            (Boss::GraftedScionChapelofAnticipation, (10010800,"Grafted Scion (Chapel of Anticipation)")),
            (Boss::GraveWardenDuelistMurkwaterCatacombs, (30040800,"Grave Warden Duelist (Murkwater Catacombs)")),
            (Boss::GuardianGolemHighroadCave, (31170800,"Guardian Golem (Highroad Cave)")),
            (Boss::MadPumpkinHeadWaypointRuins, (1044360800,"Mad Pumpkin Head (Waypoint Ruins)")),
            (Boss::MargittheFellOmen, (10000850,"Margit the Fell Omen")),
            (Boss::NightsCavalryLimgrave, (1043370800,"Night's Cavalry (Limgrave)")),
            (Boss::PatchesMurkwaterCave, (31000800,"Patches (Murkwater Cave)")),
            (Boss::SoldierofGodrick, (18000850,"Soldier of Godrick")),
            (Boss::StonediggerTrollLimgraveTunnels, (32010800,"Stonedigger Troll (Limgrave Tunnels)")),
            (Boss::TibiaMarinerSummonwaterVillage, (1045390800,"Tibia Mariner (Summonwater Village)")),
            (Boss::TreeSentinelLimgrave, (1042360800,"Tree Sentinel (Limgrave)")),

            // Weeping Weeping Peninsula
            (Boss::AncientHeroofZamorWeepingEvergaol, (1042330800,"Ancient Hero of Zamor (Weeping Evergaol)")),
            (Boss::CemeteryShadeTombswardCatacombs, (30000800,"Cemetery Shade (Tombsward Catacombs)")),
            (Boss::DeathbirdWeepingPeninsula, (1044320800,"Deathbird (Weeping Peninsula)")),
            (Boss::EdtreeBurialWatchdogImpalersCatacombs, (30010800,"Edtree Burial Watchdog (Impaler's Catacombs)")),
            (Boss::ErdtreeAvatar, (1043330800,"Erdtree Avatar")),
            (Boss::LeonineMisbegottenMorneMoangrave, (1043300800,"Leonine Misbegotten (Morne Moangrave)")),
            (Boss::MirandatheBlightedBloomTombswardCave, (31020800,"Miranda the Blighted Bloom (Tombsward Cave)")),
            (Boss::NightsCavalryWeepingPeninsula, (1044320850,"Night's Cavalry (Weeping Peninsula)")),
            (Boss::RunebearEarthboreCave, (31010800,"Runebear (Earthbore Cave)")),

            // Liurnia of the Lakes
            (Boss::AdanThiefofFireMalefactorsEvergaol, (1038410800, "Adan, Thief of Fire (Malefactor's Evergaol)")),
            (Boss::AlectoBlackKnifeRingleaderRingleadersEvergaol, (1033420800, "Alecto, Black Knife Ringleader (Ringleader's Evergaol)")),
            (Boss::BellBearingHunterChurchofVows, (1037460800, "Bell Bearing Hunter (Church of Vows)")),
            (Boss::BlackKnifeAssassinBlackKnifeCatacombs, (30050850, "Black Knife Assassin (Black Knife Catacombs)")),
            (Boss::BloodhoundKnightLakesideCrystalCave, (31050800, "Bloodhound Knight (Lakeside Crystal Cave)")),
            (Boss::BolsCarianKnightCuckoosEvergaol, (1033450800, "Bols, Carian Knight (Cuckoo's Evergaol)")),
            (Boss::CemeteryShadeBlackKnifeCatacombs, (30050800,"Cemetery Shade (Black Knife Catacombs)")),
            (Boss::CleanrotKnightStillwaterCave, (31040800,"Cleanrot Knight (Stillwater Cave)")),
            (Boss::CrystalianRayaLucariaCrystalTunnel, (32020800,"Crystalian (Raya Lucaria Crystal Tunnel)")),
            (Boss::CrystalianSpearCrystalianStaffAcademyCrystalCave, (31060800,"Crystalian Spear Crystalian Staff (Academy Crystal Cave)")),
            (Boss::DeathbirdLiurniaSouth, (1037420800,"Deathbird (Liurnia South)")),
            (Boss::DeathRiteBirdLiurniaNorth, (1036450800,"Death Rite Bird (Liurnia North)")),
            (Boss::ErdtreeAvatarLiurniaNortheast, (1038480800,"Erdtree Avatar (Liurnia Northeast)")),
            (Boss::ErdtreeAvatarLiurniaSouthwest, (1033430800,"Erdtree Avatar (Liurnia Southwest)")),
            (Boss::ErdtreeBurialWatchdogCliffbottomCatacombs, (30060800,"Erdtree Burial Watchdog (Cliffbottom Catacombs)")),
            (Boss::GlintstoneDragonAdulaMoonlightAltar, (1034500800,"Glintstone Dragon Adula (Moonlight Altar)")),
            (Boss::GlintstoneDragonSmaragLiurnia, (1034450800,"Glintstone Dragon Smarag (Liurnia)")),
            (Boss::NightsCavalryLiurniaNorth, (1036480800,"Night's Cavalry (Liurnia North)")),
            (Boss::NightsCavalryLiurniaSouth, (1039430800,"Night's Cavalry (Liurnia South)")),
            (Boss::OmenkillerVillageoftheAlbinaurics, (1035420800,"Omenkiller (Village of the Albinaurics)")),
            (Boss::OnyxLordRoyalGraveEvergaol, (1036500800,"Onyx Lord (Royal Grave Evergaol)")),
            (Boss::RoyalKnightLorettaCariaManor, (1035500800,"Royal Knight Loretta (Caria Manor)")),
            (Boss::RoyalRevenantKingsrealmRuins, (1034480800,"Royal Revenant (Kingsrealm Ruins)")),
            (Boss::SpiritcallerSnailRoadsEndCatacombs, (30030800,"Spiritcaller Snail (Road's End Catacombs)")),
            
            // Academya of Raya Lucaria
            (Boss::RennalaQueenoftheFullMoon, (14000800,"Rennala, Queen of the Full Moon")),
            (Boss::RedWolfofRadagonRayaLucaria, (14000850,"Red Wolf of Radagon (Raya Lucaria)")),

            // Ruin-Strewn Precipice
            (Boss::MagmaWyrmMakarRuinStrewnPrecipice, (39200800,"Magma Wyrm Makar (Ruin-Strewn Precipice)")),

            // Caelid
            (Boss::CommanderONeilSwampofAeonia, (1049380800,"Commander O'Neil (Swamp of Aeonia)")),
            (Boss::CrucibleKnightMisbegottenWarriorRedmaneCastle, (1051360800,"Crucible Knight. Misbegotten Warrior (Redmane Castle) ")),
            (Boss::DeathRiteBirdCaelid, (1049370850,"Death Rite Bird (Caelid)")),
            (Boss::DecayingEkzykesCaelid, (1048370800,"Decaying Ekzykes (Caelid)")),
            (Boss::ElderDragonGreyollcanapparentlyspawninvisible, (1050400800,"Elder Dragon Greyoll")),
            (Boss::ErdtreeBurialWatchdogsMinorErdtreeCatacombs, (30140800,"Erdtree Burial Watchdogs (Minor Erdtree Catacombs)")),
            (Boss::FallingstarBeastSelliaCrystalTunnel, (32080800,"Fallingstar Beast (Sellia Crystal Tunnel)")),
            (Boss::FrenziedDuelistGaolCave, (31210800,"Frenzied Duelist (Gaol Cave)")),
            (Boss::MadPumpkinHeadsCaelemRuins, (1048400800,"Mad Pumpkin Heads (Caelem Ruins)")),
            (Boss::MagmaWyrmGaelTunnel, (32070800,"Magma Wyrm (Gael Tunnel)")),
            (Boss::NightsCavalryCaelid, (1049370800,"Night's Cavalry (Caelid)")),
            (Boss::NoxSwordstressandNoxPriestSelliaTownofSorcery, (1049390800,"Nox Swordstress and Nox Priest (Sellia, Town of Sorcery)")),
            (Boss::PutridAvatarCaelid, (1047400800,"Putrid Avatar (Caelid)")),

            // Dragonbarrow
            (Boss::BattlemageHuguesSelliaEvergaol, (1049390850, "Battlemage Hugues (Sellia Evergaol)")),
            (Boss::BeastmenofFaramAzulaDragonbarrowCave, (31100800, "Beastmen of Faram Azula (Dragonbarrow Cave)")),
            (Boss::BellBearingHunterIsolatedMerchantsShack, (1048410800, "Bell Bearing Hunter (Isolated Merchant's Shack)")),
            (Boss::BlackBladeKindredBestialSanctum, (1051430800, "Black Blade Kindred (Bestial Sanctum)")),
            (Boss::FlyingDragonGreyllGreyollsDragonbarrow, (1052410800, "Flying Dragon Greyll (Greyoll's Dragonbarrow)")),
            (Boss::GodskinApostleDivineTowerofCaelid, (34130800, "Godskin Apostle (Divine Tower of Caelid)")),
            (Boss::NightsCavalryDragonbarrow, (1052410850, "Night's Cavalry (Dragonbarrow)")),
            (Boss::PutridAvatarDragonbarrow, (1052560800, "Putrid Avatar (Dragonbarrow)")),
            (Boss::PutridCrystaliansSelliaHideaway, (31110800, "Putrid Crystalians (Sellia Hideaway)")),

            // Altus Plateau
            (Boss::AncientDragonLansseaxAltusPlateau, (1041520800,"Ancient Dragon Lansseax (Altus Plateau)")),
            (Boss::AncientHeroofZamorSaintedHerosGrave, (30080800,"Ancient Hero of Zamor (Sainted Hero's Grave)")),
            (Boss::BlackKnifeAssassinSagesCave, (31190800,"Black Knife Assassin (Sage's Cave)")),
            (Boss::BlackKnifeAssassinSaintedHerosGrave, (1040520800,"Black Knife Assassin (Sainted Hero's Grave)")),
            (Boss::CrystalianSpearandCrystalianRingbladeAltusTunnel, (32050800,"Crystalian Spear and Crystalian Ringblade (Altus Tunnel)")),
            (Boss::DemiHumanQueenGilikaLuxRuins, (1038510800,"Demi-Human Queen Gilika (Lux Ruins)")),
            (Boss::ElemeroftheBriarTheShadedCastle, (1039540800,"Elemer of the Briar (The Shaded Castle)")),
            (Boss::ErdtreeBurialWatchdogWyndhamCatacombs, (30070800,"Erdtree Burial Watchdog (Wyndham Catacombs)")),
            (Boss::FallingstarBeastAltusPlateau, (1041500800,"Fallingstar Beast (Altus Plateau)")),
            (Boss::GodefroytheGraftedGoldenLineageEvergaol, (1039500800,"Godefroy the Grafted (Golden Lineage Evergaol)")),
            (Boss::GodskinApostleDominulaWindmillVillage, (1042550800,"Godskin Apostle (Dominula, Windmill Village)")),
            (Boss::NecromancerGarrisSagesCave, (31190850,"Necromancer Garris (Sage's Cave)")),
            (Boss::NightsCavalryAltusPlateau, (1039510800,"Night's Cavalry (Altus Plateau)")),
            (Boss::OmenkillerandMirandatheBlightedBloomPerfumersGrotto, (31180800,"Omenkiller and Miranda the Blighted Bloom (Perfumer's Grotto)")),
            (Boss::PerfumerTriciaandMisbegottenWarriorUnsightlyCatacombs, (30120800,"Perfumer Tricia and Misbegotten Warrior (Unsightly Catacombs)")),
            (Boss::SanguineNobleWrithebloodRuins, (1040530800,"Sanguine Noble (Writheblood Ruins)")),
            (Boss::StonediggerTrollOldAltusTunnel, (32040800,"Stonedigger Troll (Old Altus Tunnel)")),
            (Boss::TibiaMarinerWyndhamRuins, (1038520800,"Tibia Mariner (Wyndham Ruins)")),
            (Boss::TreeSentinelsAltusPlateau, (1041510800,"Tree Sentinels (Altus Plateau)")),

            // Capital Outskirts
            (Boss::BellBearingHunterHermitMerchantsShack, (1043530800,"Bell Bearing Hunter (Hermit Merchant's Shack)")),
            (Boss::CrucibleKnightCrucibleKnightOrdovisAurizaHeroGrave, (30100800,"Crucible Knight, Crucible Knight Ordovis (Auriza Hero Grave)")),
            (Boss::DeathbirdWarmastersShack, (1042380850,"Deathbird (Warmaster's Shack)")),
            (Boss::DraconicTreeSentinelCapitalOutskirts, (1045520800,"Draconic Tree Sentinel (Capital Outskirts)")),
            (Boss::FellTwinsDivineTowerofEastAltus, (34140850,"Fell Twins (Divine Tower of East Altus)")),
            (Boss::GraveWardenDuelistAurizaSideTomb, (30130800,"Grave Warden Duelist (Auriza Side Tomb)")),

            // Leyendell, Royal Capital
            (Boss::GodfreyFirstEldenLordGoldenShade, (11000850,"Godfrey, First Elden Lord (Golden Shade)")),
            (Boss::MohgtheOmenCathedraloftheForsaken, (35000800,"Mohg, the Omen (Cathedral of the Forsaken)")),
            (Boss::MorgotttheOmenKing, (11000800,"Morgott, the Omen King")),

            // Mt. Gelmir
            (Boss::AbductorVirginsMtGelmir, (16000860,"Abductor Virgins (Mt. Gelmir)")),
            (Boss::DemiHumanQueenMaggieHermitVillage, (1037530800,"Demi-Human Queen Maggie (Hermit Village)")),
            (Boss::DemiHumanQueenMargotVolanoCave, (31090800,"Demi-Human Queen Margot (Volano Cave)")),
            (Boss::FullGrownFallingstarBeastNinthMtGelmirCampsite, (1036540800,"Full-Grown Fallingstar Beast (Ninth Mt. Gelmir Campsite)")),
            (Boss::GodDevouringSerpentRykardLordofBlasphemy, (16000800,"God-Devouring Serpent / Rykard, Lord of Blasphemy")),
            (Boss::GodskinNobleTempleofEiglay, (16000850,"Godskin Noble (Temple of Eiglay)")),
            (Boss::KindredofRotSeethewaterCave, (31070800,"Kindred of Rot (Seethewater Cave)")),
            (Boss::MagmaWyrmMtGelmir, (1035530800,"Magma Wyrm (Mt. Gelmir)")),
            (Boss::RedWolfoftheChampionGelmirHerosGrave, (30090800,"Red Wolf of the Champion (Gelmir Hero's Grave)")),
            (Boss::UlceratedTreeSpiritMtGelmir, (1037540810,"Ulcerated Tree Spirit (Mt. Gelmir)")),
            
            //Mountaintops of the Giants
            (Boss::AncientHeroofZamorGiantConquringHerosGrave, (30170800,"Ancient Hero of Zamor (Giant-Conquring Hero's Grave)")),
            (Boss::BorealistheFreezingFogFreezingLake, (1054560800,"Borealis the Freezing Fog (Freezing Lake)")),
            (Boss::CommanderNiallCastleSol, (1051570800,"Commander Niall (Castle Sol)")),
            (Boss::DeathRiteBirdMountaintopsoftheGiants, (1050570800,"Death Rite Bird (Mountaintops of the Giants)")),
            (Boss::ErdtreeAvatarMountaintopsoftheGiants, (1050570800,"Erdtree Avatar (Mountaintops of the Giants)")),
            (Boss::FireGiant, (1052520800,"Fire Giant")),
            (Boss::GodskinApostleandGodskinNobleSpiritcallerCaveSpiritcallerSnail, (31220800,"Godskin Apostle and Godskin Noble  (Spiritcaller Cave) Spiritcaller Snail")),
            (Boss::UlceratedTreeSpiritGiantsMountaintopCatacombs, (30180800,"Ulcerated Tree Spirit (Giants' Mountaintop Catacombs)")),
            (Boss::VykeKnightoftheRoundtableLordContendersEvergaol, (1053560800,"Vyke, Knight of the Roundtable (Lord Contender's Evergaol)")),
            
            // Crumbling Farum Azula
            (Boss::DragonlordPlacidusax, (13000830,"Dragonlord Placidusax")),
            (Boss::GodskinDuo, (13000850,"Godskin Duo")),
            (Boss::MalekiththeBlackBlade, (13000800,"Malekith, the Black Blade")),

            //Forbidden Lands
            (Boss::BlackBladeKindredForbiddenLands, (1049520800,"Black Blade Kindred (Forbidden Lands)")),
            (Boss::NightsCavalryForbiddenLands, (1048510800,"Night's Cavalry (Forbidden Lands)")),
            (Boss::StrayMimicTearHiddenPathtotheHaligtree, (30200800,"Stray Mimic Tear (Hidden Path to the Haligtree)")),
            
            // Consecrated Snowfields
            (Boss::AstelStarsofDarknessYeloughAnixTunnel, (32110800,"Astel, Stars of Darkness (Yelough Anix Tunnel)")),
            (Boss::DeathRiteBirdConsecratedSnowfield, (1048570800,"Death Rite Bird (Consecrated Snowfield)")),
            (Boss::GreatWyrmTheodorixConsecratedSnowfield, (1050560800,"Great Wyrm Theodorix (Consecrated Snowfield)")),
            (Boss::MisbegottenCrusaderCaveoftheForlorn, (31120800,"Misbegotten Crusader (Cave of the Forlorn)")),
            (Boss::NightsCavalryDuoConsecratedSnowfield, (1248550800,"Night's Cavalry Duo (Consecrated Snowfield)")),
            (Boss::PutridAvatar, (1050570850,"Putrid Avatar")),
            (Boss::PutridGraveWardenDuelistConsecratedSnowfieldCatacombs, (30190800,"Putrid Grave Warden Duelist (Consecrated Snowfield Catacombs)")),
            
            // Miquella's Haligtree
            (Boss::LorettaKnightoftheHaligtree, (15000850,"Loretta, Knight of the Haligtree")),
            (Boss::MaleniaGoddessofRot, (15000800,"Malenia, Goddess of Rot")),

            // Siofra River
            (Boss::AncestorSpiritSiofraRiverBank, (12080800,"Ancestor Spirit (Siofra River Bank)")),
            (Boss::DragonkinSoldierSiofraRiverBank, (12020830,"Dragonkin Soldier (Siofra River Bank)")),
            (Boss::MohgLordofBlood, (12050800,"Mohg, Lord of Blood")),
            
            // Ainsel River
            (Boss::DragonkinSoldierofNokstellaAinselRiver, (12010800,"Dragonkin Soldier of Nokstella (Ainsel River)")),

            // Nokron Eternal City
            (Boss::MimicTearNokronEternalCity, (12020850,"Mimic Tear (Nokron, Eternal City)")),
            (Boss::RegalAncestorSpiritAncestralWoods, (12090800,"Regal Ancestor Spirit (Ancestral Woods)")),
            (Boss::ValiantGargoylesSiofraAqueduct, (12020800,"Valiant Gargoyles (Siofra Aqueduct)")),

            // Deeproot Depths
            (Boss::CrucibleKnightSiluria, (12030390,"Crucible Knight Siluria")),
            (Boss::FiasChampions, (12030800,"Fia's Champions")),
            (Boss::LichdragonFortissax, (12030850,"Lichdragon Fortissax")),

            // Lake of Rot
            (Boss::AstelNaturalbornoftheVoid, (12040800,"Astel, Naturalborn of the Void")),
            (Boss::DragonkinSoldierLakeofRot, (12010850,"Dragonkin Soldier (Lake of Rot)")),

            // Leyendell, Ashen Capital
            (Boss::GodfreyFirstEldenLordHoarahLoux, (11050800,"Godfrey, First Elden Lord (Hoarah Loux)")),
            (Boss::SirGideonOfnirtheAllKnowing, (11050850,"Sir Gideon Ofnir, the All-Knowing")),

            // Elden Throne
            (Boss::RadagonoftheGoldenOrderEldenBeast, (19000810,"Radagon of the Golden Order / Elden Beast")),
        ]))
    });
}