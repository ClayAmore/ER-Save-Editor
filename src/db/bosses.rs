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
        ErdtreeAvatar,
        EdtreeBurialWatchdogImpalersCatacombs,
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

        // Caelid
        CemeteryShadeBlackKnifeCatacombs,
        CleanrotKnightStillwaterCave,
        CrystalianRayaLucariaCrystalTunnel,
        CrystalianSpearCrystalianStaffAcademyCrystalCave,
        DeathRiteBirdLiurniaNorth,
        DeathbirdLiurniaSouth,
        ErdtreeAvatarLiurniaNortheast,
        ErdtreeAvatarLiurniaSouthwest,
        ErdtreeBurialWatchdogCliffbottomCatacombs,
        GlintstoneDragonAdulaMoonlightAltar,
        GlintstoneDragonSmaragLiurnia,
        MagmaWyrmMakarRuinStrewnPrecipice,
        NightsCavalryLiurniaNorth,
        NightsCavalryLiurniaSouth,
        OmenkillerVillageoftheAlbinaurics,
        OnyxLordRoyalGraveEvergaol,
        RedWolfofRadagonRayaLucaria,
        Rennala,QueenoftheFullMoon,
        RoyalKnightLorettaCariaManor,
        RoyalRevenantKingsrealmRuins,
        SpiritcallerSnailRoadsEndCatacombs,
        CommanderONeilSwampofAeonia,
        CrucibleKnightMisbegottenWarriorRedmaneCastle,
        DeathRiteBirdCaelid,
        DecayingEkzykesCaelid,
        ElderDragonGreyollcanapparentlyspawninvisible,
        ErdtreeBurialWatchdogsMinorErdtreeCatacombs,
        FallingstarBeastSelliaCrystalTunnel,
        FrenziedDuelistGaolCave,
        MadPumpkinHeadsCaelemRuins,
        MagmaWyrmGaelTunnel,
        NightsCavalryCaelid,
        NoxSwordstressandNoxPriestSelliaTownofSorcery,
        PutridAvatarCaelid,

        // Dragonbarrow
        BattlemageHuguesSelliaEvergaol,
        BeastmenofFaramAzulaDragonbarrowCave,
        BellBearingHunterIsolatedMerchantsShack,
        BlackBladeKindredBestialSanctum,
        CleanrotKnightsStillwaterCave,
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
        GodskinApostleDominula,WindmillVillage,
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

        // NPCS
        MirielPastorofVows,
        MerchantKale,
        SorceressSellen,
        WhiteFacedVarre,
        IronFistAlexander,
        Irina,
        Edgar,
        Latenna,
        JarBairn,
        Tophs,
        KnightBernahl,
        BlackguardBigBoggart,
        Millicent,
        NepheliLoux,
        GatekeeperGostoc,
        BoctheSeamster,
        KennethHaight
    }
    pub static BOSSES: Lazy<Mutex<HashMap<Boss,(u32,&str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            // Limgrave
            (Boss::AlabasterLordEvergaol ,(0,"Alabaster Lord (Evergaol)")),
            (Boss::BeastmanofFaramAzulaGrovesideCave ,(0,"Beastman of Faram Azula (Groveside Cave)")),
            (Boss::BellBearingHunterWarmastersShack ,(0,"Bell Bearing Hunter (Warmaster's Shack)")),
            (Boss::BlackKnifeAssassinDeathtouchedCatacombs ,(0,"Black Knife Assassin (Deathtouched Catacombs)")),
            (Boss::BloodhoundKnightDarriwilForlornHoundEvergaol ,(0,"Bloodhound Knight Darriwil (Forlorn Hound Evergaol)")),
            (Boss::CrucibleKnightStormhillEvergaol ,(0,"Crucible Knight (Stormhill Evergaol)")),
            (Boss::DeathbirdLimgrave ,(0,"Deathbird (Limgrave)")),
            (Boss::DemiHumanChiefsCoastalCave ,(0,"Demi-Human Chiefs (Coastal Cave)")),
            (Boss::ErdtreeBurialWatchdogStormfrontCatacombs ,(0,"Erdtree Burial Watchdog (Stormfront Catacombs)")),
            (Boss::FlyingDragonAgheelLimgrave ,(0,"Flying Dragon Agheel (Limgrave)")),
            (Boss::GodricktheGrafted ,(0,"Godrick the Grafted")),
            (Boss::GraftedScionChapelofAnticipation ,(0,"Grafted Scion (Chapel of Anticipation)")),
            (Boss::GraveWardenDuelistMurkwaterCatacombs ,(0,"Grave Warden Duelist (Murkwater Catacombs)")),
            (Boss::GuardianGolemHighroadCave ,(0,"Guardian Golem (Highroad Cave)")),
            (Boss::MadPumpkinHeadWaypointRuins ,(0,"Mad Pumpkin Head (Waypoint Ruins)")),
            (Boss::MargittheFellOmen ,(0,"Margit the Fell Omen")),
            (Boss::NightsCavalryLimgrave ,(0,"Night's Cavalry (Limgrave)")),
            (Boss::PatchesMurkwaterCave ,(0,"Patches (Murkwater Cave)")),
            (Boss::SoldierofGodrick ,(0,"Soldier of Godrick")),
            (Boss::StonediggerTrollLimgraveTunnels ,(0,"Stonedigger Troll (Limgrave Tunnels)")),
            (Boss::TibiaMarinerSummonwaterVillage ,(0,"Tibia Mariner (Summonwater Village)")),
            (Boss::TreeSentinelLimgrave ,(0,"Tree Sentinel (Limgrave)")),

            // Weeping Weeping Peninsula
            (Boss::AncientHeroofZamorWeepingEvergaol, (0,"Ancient Hero of Zamor (Weeping Evergaol)")),
            (Boss::CemeteryShadeTombswardCatacombs, (0,"Cemetery Shade (Tombsward Catacombs)")),
            (Boss::DeathbirdWeepingPeninsula, (0,"Deathbird (Weeping Peninsula)")),
            (Boss::ErdtreeAvatar, (0,"Erdtree Avatar")),
            (Boss::EdtreeBurialWatchdogImpalersCatacombs, (0,"Edtree Burial Watchdog (Impaler's Catacombs)")),
            (Boss::LeonineMisbegottenMorneMoangrave, (0,"Leonine Misbegotten (Morne Moangrave)")),
            (Boss::MirandatheBlightedBloomTombswardCave, (0,"Miranda the Blighted Bloom (Tombsward Cave)")),
            (Boss::NightsCavalryWeepingPeninsula, (0,"Night's Cavalry (Weeping Peninsula)")),
            (Boss::RunebearEarthboreCave, (0,"Runebear (Earthbore Cave)")),

            // Liurnia of the Lakes
            (Boss::AdanThiefofFireMalefactorsEvergaol, (0, "Adan, Thief of Fire (Malefactor's Evergaol)")),
            (Boss::AlectoBlackKnifeRingleaderRingleadersEvergaol, (0, "Alecto, Black Knife Ringleader (Ringleader's Evergaol)")),
            (Boss::BellBearingHunterChurchofVows, (0, "Bell Bearing Hunter (Church of Vows)")),
            (Boss::BlackKnifeAssassinBlackKnifeCatacombs, (0, "Black Knife Assassin (Black Knife Catacombs)")),
            (Boss::BloodhoundKnightLakesideCrystalCave, (0, "Bloodhound Knight (Lakeside Crystal Cave)")),
            (Boss::BolsCarianKnightCuckoosEvergaol, (0, "Bols, Carian Knight (Cuckoo's Evergaol)")),
            
            // Caelid
            (Boss::CemeteryShadeBlackKnifeCatacombs, (0,"Cemetery Shade (Black Knife Catacombs)")),
            (Boss::CleanrotKnightStillwaterCave, (0,"Cleanrot Knight (Stillwater Cave)")),
            (Boss::CrystalianRayaLucariaCrystalTunnel, (0,"Crystalian (Raya Lucaria Crystal Tunnel)")),
            (Boss::CrystalianSpearCrystalianStaffAcademyCrystalCave, (0,"Crystalian Spear &amp; Crystalian Staff (Academy Crystal Cave)")),
            (Boss::DeathRiteBirdLiurniaNorth, (0,"Death Rite Bird (Liurnia North)")),
            (Boss::DeathbirdLiurniaSouth, (0,"Deathbird (Liurnia South)")),
            (Boss::ErdtreeAvatarLiurniaNortheast, (0,"Erdtree Avatar (Liurnia Northeast)")),
            (Boss::ErdtreeAvatarLiurniaSouthwest, (0,"Erdtree Avatar (Liurnia Southwest)")),
            (Boss::ErdtreeBurialWatchdogCliffbottomCatacombs, (0,"Erdtree Burial Watchdog (Cliffbottom Catacombs)")),
            (Boss::GlintstoneDragonAdulaMoonlightAltar, (0,"Glintstone Dragon Adula (Moonlight Altar)")),
            (Boss::GlintstoneDragonSmaragLiurnia, (0,"Glintstone Dragon Smarag (Liurnia)")),
            (Boss::MagmaWyrmMakarRuinStrewnPrecipice, (0,"Magma Wyrm Makar (Ruin-Strewn Precipice)")),
            (Boss::NightsCavalryLiurniaNorth, (0,"Night's Cavalry (Liurnia North)")),
            (Boss::NightsCavalryLiurniaSouth, (0,"Night's Cavalry (Liurnia South)")),
            (Boss::OmenkillerVillageoftheAlbinaurics, (0,"Omenkiller (Village of the Albinaurics)")),
            (Boss::OnyxLordRoyalGraveEvergaol, (0,"Onyx Lord (Royal Grave Evergaol)")),
            (Boss::RedWolfofRadagonRayaLucaria, (0,"Red Wolf of Radagon (Raya Lucaria)")),
            (Boss::Rennala, (0,"Rennala, Queen of the Full Moon")),
            (Boss::RoyalKnightLorettaCariaManor, (0,"Royal Knight Loretta (Caria Manor)")),
            (Boss::RoyalRevenantKingsrealmRuins, (0,"Royal Revenant (Kingsrealm Ruins)")),
            (Boss::SpiritcallerSnailRoadsEndCatacombs, (0,"Spiritcaller Snail (Road's End Catacombs)")),
            (Boss::CommanderONeilSwampofAeonia, (0,"Commander O'Neil (Swamp of Aeonia)")),
            (Boss::CrucibleKnightMisbegottenWarriorRedmaneCastle, (0,"Crucible Knight &amp; Misbegotten Warrior (Redmane Castle) ")),
            (Boss::DeathRiteBirdCaelid, (0,"Death Rite Bird (Caelid)")),
            (Boss::DecayingEkzykesCaelid, (0,"Decaying Ekzykes (Caelid)")),
            (Boss::ElderDragonGreyollcanapparentlyspawninvisible, (0,"Elder Dragon Greyoll (can apparently spawn invisible)")),
            (Boss::ErdtreeBurialWatchdogsMinorErdtreeCatacombs, (0,"Erdtree Burial Watchdogs (Minor Erdtree Catacombs)")),
            (Boss::FallingstarBeastSelliaCrystalTunnel, (0,"Fallingstar Beast (Sellia Crystal Tunnel)")),
            (Boss::FrenziedDuelistGaolCave, (0,"Frenzied Duelist (Gaol Cave)")),
            (Boss::MadPumpkinHeadsCaelemRuins, (0,"Mad Pumpkin Heads (Caelem Ruins)")),
            (Boss::MagmaWyrmGaelTunnel, (0,"Magma Wyrm (Gael Tunnel)")),
            (Boss::NightsCavalryCaelid, (0,"Night's Cavalry (Caelid)")),
            (Boss::NoxSwordstressandNoxPriestSelliaTownofSorcery, (0,"Nox Swordstress and Nox Priest (Sellia, Town of Sorcery)")),
            (Boss::PutridAvatarCaelid, (0,"Putrid Avatar (Caelid)")),

            // Dragonbarrow
            (Boss::BattlemageHuguesSelliaEvergaol, (0, "Battlemage Hugues (Sellia Evergaol)")),
            (Boss::BeastmenofFaramAzulaDragonbarrowCave, (0, "Beastmen of Faram Azula (Dragonbarrow Cave)")),
            (Boss::BellBearingHunterIsolatedMerchantsShack, (0, "Bell Bearing Hunter (Isolated Merchant's Shack)")),
            (Boss::BlackBladeKindredBestialSanctum, (0, "Black Blade Kindred (Bestial Sanctum)")),
            (Boss::CleanrotKnightsStillwaterCave, (0, "Cleanrot Knights (Stillwater Cave)")),
            (Boss::FlyingDragonGreyllGreyollsDragonbarrow, (0, "Flying Dragon Greyll (Greyoll's Dragonbarrow)")),
            (Boss::GodskinApostleDivineTowerofCaelid, (0, "Godskin Apostle (Divine Tower of Caelid)")),
            (Boss::NightsCavalryDragonbarrow, (0, "Night's Cavalry (Dragonbarrow)")),
            (Boss::PutridAvatarDragonbarrow, (0, "Putrid Avatar (Dragonbarrow)")),
            (Boss::PutridCrystaliansSelliaHideaway, (0, "Putrid Crystalians (Sellia Hideaway)")),

            // Altus Plateau
            (Boss::AncientDragonLansseaxAltusPlateau, (0,"Ancient Dragon Lansseax (Altus Plateau)")),
            (Boss::AncientHeroofZamorSaintedHerosGrave, (0,"Ancient Hero of Zamor (Sainted Hero's Grave)")),
            (Boss::BlackKnifeAssassinSagesCave, (0,"Black Knife Assassin (Sage's Cave)")),
            (Boss::BlackKnifeAssassinSaintedHerosGrave, (0,"Black Knife Assassin (Sainted Hero's Grave)")),
            (Boss::CrystalianSpearandCrystalianRingbladeAltusTunnel, (0,"Crystalian Spear and Crystalian Ringblade (Altus Tunnel)")),
            (Boss::DemiHumanQueenGilikaLuxRuins, (0,"Demi-Human Queen Gilika (Lux Ruins)")),
            (Boss::ElemeroftheBriarTheShadedCastle, (0,"Elemer of the Briar (The Shaded Castle)")),
            (Boss::ErdtreeBurialWatchdogWyndhamCatacombs, (0,"Erdtree Burial Watchdog (Wyndham Catacombs)")),
            (Boss::FallingstarBeastAltusPlateau, (0,"Fallingstar Beast (Altus Plateau)")),
            (Boss::GodefroytheGraftedGoldenLineageEvergaol, (0,"Godefroy the Grafted (Golden Lineage Evergaol)")),
            (Boss::GodskinApostleDominula, (0,"Godskin Apostle (Dominula, Windmill Village)")),
            (Boss::NecromancerGarrisSagesCave, (0,"Necromancer Garris (Sage's Cave)")),
            (Boss::NightsCavalryAltusPlateau, (0,"Night's Cavalry (Altus Plateau)")),
            (Boss::OmenkillerandMirandatheBlightedBloomPerfumersGrotto, (0,"Omenkiller and Miranda the Blighted Bloom (Perfumer's Grotto)")),
            (Boss::PerfumerTriciaandMisbegottenWarriorUnsightlyCatacombs, (0,"Perfumer Tricia and Misbegotten Warrior (Unsightly Catacombs)")),
            (Boss::SanguineNobleWrithebloodRuins, (0,"Sanguine Noble (Writheblood Ruins)")),
            (Boss::StonediggerTrollOldAltusTunnel, (0,"Stonedigger Troll (Old Altus Tunnel)")),
            (Boss::TibiaMarinerWyndhamRuins, (0,"Tibia Mariner (Wyndham Ruins)")),
            (Boss::TreeSentinelsAltusPlateau, (0,"Tree Sentinels (Altus Plateau)")),

            // Capital Outskirts
            (Boss::BellBearingHunterHermitMerchantsShack, (0,"Bell Bearing Hunter (Hermit Merchant's Shack)")),
            (Boss::CrucibleKnightCrucibleKnightOrdovisAurizaHeroGrave, (0,"Crucible Knight &amp; Crucible Knight Ordovis (Auriza Hero Grave)")),
            (Boss::DeathbirdWarmastersShack, (0,"Deathbird (Warmaster's Shack)")),
            (Boss::DraconicTreeSentinelCapitalOutskirts, (0,"Draconic Tree Sentinel (Capital Outskirts)")),
            (Boss::FellTwinsDivineTowerofEastAltus, (0,"Fell Twins (Divine Tower of East Altus)")),
            (Boss::GraveWardenDuelistAurizaSideTomb, (0,"Grave Warden Duelist (Auriza Side Tomb)")),

            // Leyendell, Royal Capital
            (Boss::GodfreyFirstEldenLordGoldenShade, (0,"Godfrey, First Elden Lord (Golden Shade)")),
            (Boss::MohgtheOmenCathedraloftheForsaken, (0,"Mohg, the Omen (Cathedral of the Forsaken)")),
            (Boss::MorgotttheOmenKing, (0,"Morgott, the Omen King")),

            // Mt. Gelmir
            (Boss::AbductorVirginsMtGelmir, (0,"Abductor Virgins (Mt. Gelmir)")),
            (Boss::DemiHumanQueenMaggieHermitVillage, (0,"Demi-Human Queen Maggie (Hermit Village)")),
            (Boss::DemiHumanQueenMargotVolanoCave, (0,"Demi-Human Queen Margot (Volano Cave)")),
            (Boss::FullGrownFallingstarBeastNinthMtGelmirCampsite, (0,"Full-Grown Fallingstar Beast (Ninth Mt. Gelmir Campsite)")),
            (Boss::GodDevouringSerpentRykardLordofBlasphemy, (0,"God-Devouring Serpent / Rykard, Lord of Blasphemy")),
            (Boss::GodskinNobleTempleofEiglay, (0,"Godskin Noble (Temple of Eiglay)")),
            (Boss::KindredofRotSeethewaterCave, (0,"Kindred of Rot (Seethewater Cave)")),
            (Boss::MagmaWyrmMtGelmir, (0,"Magma Wyrm (Mt. Gelmir)")),
            (Boss::RedWolfoftheChampionGelmirHerosGrave, (0,"Red Wolf of the Champion (Gelmir Hero's Grave)")),
            (Boss::UlceratedTreeSpiritMtGelmir, (0,"Ulcerated Tree Spirit (Mt. Gelmir)")),
            
            //Mountaintops of the Giants
            (Boss::AncientHeroofZamorGiantConquringHerosGrave, (0,"Ancient Hero of Zamor (Giant-Conquring Hero's Grave)")),
            (Boss::BorealistheFreezingFogFreezingLake, (0,"Borealis the Freezing Fog (Freezing Lake)")),
            (Boss::CommanderNiallCastleSol, (0,"Commander Niall (Castle Sol)")),
            (Boss::DeathRiteBirdMountaintopsoftheGiants, (0,"Death Rite Bird (Mountaintops of the Giants)")),
            (Boss::ErdtreeAvatarMountaintopsoftheGiants, (0,"Erdtree Avatar (Mountaintops of the Giants)")),
            (Boss::FireGiant, (0,"Fire Giant")),
            (Boss::GodskinApostleandGodskinNobleSpiritcallerCaveSpiritcallerSnail, (0,"Godskin Apostle and Godskin Noble  (Spiritcaller Cave) Spiritcaller Snail")),
            (Boss::UlceratedTreeSpiritGiantsMountaintopCatacombs, (0,"Ulcerated Tree Spirit (Giants' Mountaintop Catacombs)")),
            (Boss::VykeKnightoftheRoundtableLordContendersEvergaol, (0,"Vyke, Knight of the Roundtable (Lord Contender's Evergaol)")),
            
            // Crumbling Farum Azula
            (Boss::DragonlordPlacidusax, (0,"Dragonlord Placidusax")),
            (Boss::GodskinDuo, (0,"Godskin Duo")),
            (Boss::MalekiththeBlackBlade, (0,"Malekith, the Black Blade")),

            //Forbidden Lands
            (Boss::BlackBladeKindredForbiddenLands, (0,"Black Blade Kindred (Forbidden Lands)")),
            (Boss::NightsCavalryForbiddenLands, (0,"Night's Cavalry (Forbidden Lands)")),
            (Boss::StrayMimicTearHiddenPathtotheHaligtree, (0,"Stray Mimic Tear (Hidden Path to the Haligtree)")),
            
            // Consecrated Snowfields
            (Boss::AstelStarsofDarknessYeloughAnixTunnel, (0,"Astel, Stars of Darkness (Yelough Anix Tunnel)")),
            (Boss::DeathRiteBirdConsecratedSnowfield, (0,"Death Rite Bird (Consecrated Snowfield)")),
            (Boss::GreatWyrmTheodorixConsecratedSnowfield, (0,"Great Wyrm Theodorix (Consecrated Snowfield)")),
            (Boss::MisbegottenCrusaderCaveoftheForlorn, (0,"Misbegotten Crusader (Cave of the Forlorn)")),
            (Boss::NightsCavalryDuoConsecratedSnowfield, (0,"Night's Cavalry Duo (Consecrated Snowfield)")),
            (Boss::PutridAvatar, (0,"Putrid Avatar")),
            (Boss::PutridGraveWardenDuelistConsecratedSnowfieldCatacombs, (0,"Putrid Grave Warden Duelist (Consecrated Snowfield Catacombs)")),
            
            // Miquella's Haligtree
            (Boss::LorettaKnightoftheHaligtree, (0,"Loretta, Knight of the Haligtree")),
            (Boss::MaleniaGoddessofRot, (0,"Malenia, Goddess of Rot")),

            // Siofra River
            (Boss::AncestorSpiritSiofraRiverBank, (0,"Ancestor Spirit (Siofra River Bank)")),
            (Boss::DragonkinSoldierSiofraRiverBank, (0,"Dragonkin Soldier (Siofra River Bank)")),
            (Boss::MohgLordofBlood, (0,"Mohg, Lord of Blood")),
            
            // Ainsel River
            (Boss::DragonkinSoldierofNokstellaAinselRiver, (0,"Dragonkin Soldier of Nokstella (Ainsel River)")),

            // Nokron Eternal City
            (Boss::MimicTearNokronEternalCity, (0,"Mimic Tear (Nokron, Eternal City)")),
            (Boss::RegalAncestorSpiritAncestralWoods, (0,"Regal Ancestor Spirit (Ancestral Woods)")),
            (Boss::ValiantGargoylesSiofraAqueduct, (0,"Valiant Gargoyles (Siofra Aqueduct)")),

            // Deeproot Depths
            (Boss::CrucibleKnightSiluria, (0,"Crucible Knight Siluria")),
            (Boss::FiasChampions, (0,"Fia's Champions")),
            (Boss::LichdragonFortissax, (0,"Lichdragon Fortissax")),

            // Lake of Rot
            (Boss::AstelNaturalbornoftheVoid, (0,"Astel, Naturalborn of the Void")),
            (Boss::DragonkinSoldierLakeofRot, (0,"Dragonkin Soldier (Lake of Rot)")),

            // Leyendell, Ashen Capital
            (Boss::GodfreyFirstEldenLordHoarahLoux, (0,"Godfrey, First Elden Lord (Hoarah Loux)")),
            (Boss::SirGideonOfnirtheAllKnowing, (0,"Sir Gideon Ofnir, the All-Knowing")),

            // Elden Throne
            (Boss::RadagonoftheGoldenOrderEldenBeast, (0,"Radagon of the Golden Order / Elden Beast")),

            // NPC 
            (Boss::MirielPastorofVows, (0,"Miriel, Pastor of Vows")),
            (Boss::MerchantKale, (0,"Merchant Kalé")),
            (Boss::SorceressSellen, (0,"Sorceress Sellen")),
            (Boss::WhiteFacedVarre, (0,"White-Faced Varré")),
            (Boss::IronFistAlexander, (0,"Iron Fist Alexander")),
            (Boss::Irina, (0,"Irina")),
            (Boss::Edgar, (0,"Edgar")),
            (Boss::Latenna, (0,"Latenna")),
            (Boss::JarBairn, (0,"Jar Bairn")),
            (Boss::Tophs, (0,"Tophs")),
            (Boss::KnightBernahl, (0,"Knight Bernahl")),
            (Boss::BlackguardBigBoggart, (0,"Blackguard Big Boggart")),
            (Boss::Millicent, (0,"Millicent ")),
            (Boss::NepheliLoux, (0,"Nepheli Loux")),
            (Boss::GatekeeperGostoc, (0,"Gatekeeper Gostoc")),
            (Boss::BoctheSeamster, (0,"Boc the Seamster")),
            (Boss::KennethHaight, (0,"Kenneth Haight")),
        ]))
    });
}