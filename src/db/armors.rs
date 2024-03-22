// Modified from TGA table MassItemGib
// https://github.com/The-Grand-Archives/Elden-Ring-CT-TGA
use std::collections::BTreeMap;

use once_cell::sync::OnceCell;

pub fn armor_sets() -> &'static BTreeMap<String, Vec<u32>> {
    static ARMORS: OnceCell<BTreeMap<String, Vec<u32>>> = OnceCell::new();
    ARMORS.get_or_init(|| {
        let mut armor_sets = BTreeMap::new();
         armor_sets.insert("Iron".to_string(),vec![
            0x10009C40, // Iron Helmet
            0x10009CA4, // Scale Armor
            0x10009D08, // Iron Gauntlets
            0x10009D6C, // Leather Trousers
         ]);
         armor_sets.insert("Kaiden".to_string(),vec![
            0x1000C350, // Kaiden Helm
            0x1000C3B4, // Kaiden Armor
            0x1000C418, // Kaiden Gauntlets
            0x1000C47C, // Kaiden Trousers
         ]);
         armor_sets.insert("Drake Knight".to_string(),vec![
            0x1000EA60, // Drake Knight Helm
            0x1000EAC4, // Drake Knight Armor
            0x1000EB28, // Drake Knight Gauntlets
            0x1000EB8C, // Drake Knight Greaves
            0x1000EE48, // Drake Knight Helm (Altered)
            0x1000EEAC, // Drake Knight Armor (Altered)
         ]);
         armor_sets.insert("Scaled".to_string(),vec![
            0x10013880, // Scaled Helm
            0x100138E4, // Scaled Armor
            0x10013948, // Scaled Gauntlets
            0x100139AC, // Scaled Greaves
            0x10013CCC, // Scaled Armor (Altered)
         ]);
         armor_sets.insert("Perfumer".to_string(),vec![
            0x10015F90, // Perfumer Hood
            0x10015FF4, // Perfumer Robe
            0x10016058, // Perfumer Gloves
            0x100160BC, // Perfumer Sarong
            0x100163DC, // Perfumer Robe (Altered)
         ]);
         armor_sets.insert("Traveler's Hat".to_string(),vec![
            0x100186A0, // Traveler's Hat
            0x10018704, // Perfumer's Traveling Garb
            0x10018768, // Traveler's Gloves
            0x100187CC, // Traveler's Slops
            0x10018AEC, // Perfumer's Traveling Garb (Altered)
         ]);
         armor_sets.insert("Alberich's".to_string(),vec![
            0x1001D4C0, // Alberich's Pointed Hat
            0x1001D524, // Alberich's Robe
            0x1001D588, // Alberich's Bracers
            0x1001D5EC, // Alberich's Trousers
            0x1001D8A8, // Alberich's Pointed Hat (Altered)
            0x1001D90C, // Alberich's Robe (Altered)
         ]);
         armor_sets.insert("Spellblade".to_string(),vec![
            0x1001FBD0, // Spellblade's Pointed Hat
            0x1001FC34, // Spellblade's Traveling Attire
            0x1001FC98, // Spellblade's Gloves
            0x1001FCFC, // Spellblade's Trousers
            0x1002001C, // Spellblade's Traveling Attire (Altered)
         ]);
         armor_sets.insert("Bull-Goat".to_string(),vec![
            0x100222E0, // Bull-Goat Helm
            0x10022344, // Bull-Goat Armor
            0x100223A8, // Bull-Goat Gauntlets
            0x1002240C, // Bull-Goat Greaves
         ]);
         armor_sets.insert("Ronin".to_string(),vec![
            0x10024A54, // Ronin's Armor
            0x10024AB8, // Ronin's Gauntlets
            0x10024B1C, // Ronin's Greaves
            0x10024E3C, // Ronin's Armor (Altered)
         ]);
         armor_sets.insert("Guilty".to_string(),vec![
            0x10027100, // Guilty Hood
            0x10027164, // Cloth Garb
            0x1002722C, // Cloth Trousers
         ]);
         armor_sets.insert("Blaidd".to_string(),vec![
            0x10029810, // Black Wolf Mask
            0x10029874, // Blaidd's Armor
            0x100298D8, // Blaidd's Gauntlets
            0x1002993C, // Blaidd's Greaves
            0x10029C5C, // Blaidd's Armor (Altered)
         ]);
         armor_sets.insert("Black Knife".to_string(),vec![
            0x1002BF20, // Black Knife Hood
            0x1002BF84, // Black Knife Armor
            0x1002BFE8, // Black Knife Gauntlets
            0x1002C04C, // Black Knife Greaves
            0x1002C36C, // Black Knife Armor (Altered)
         ]);
         armor_sets.insert("Exile".to_string(),vec![
            0x1002E630, // Exile Hood
            0x1002E694, // Exile Armor
            0x1002E6F8, // Exile Gauntlets
            0x1002E75C, // Exile Greaves
         ]);
         armor_sets.insert("Banished Knight".to_string(),vec![
            0x10030D40, // Banished Knight Helm
            0x10030DA4, // Banished Knight Armor
            0x10030E08, // Banished Knight Gauntlets
            0x10030E6C, // Banished Knight Greaves
            0x10031128, // Banished Knight Helm (Altered)
            0x1003118C, // Banished Knight Armor (Altered)
         ]);
         armor_sets.insert("Briar".to_string(),vec![
            0x10033450, // Briar Helm
            0x100334B4, // Briar Armor
            0x10033518, // Briar Gauntlets
            0x1003357C, // Briar Greaves
            0x1003389C, // Briar Armor (Altered)
         ]);
         armor_sets.insert("Page".to_string(),vec![
            0x10035B60, // Page Hood
            0x10035BC4, // Page Garb
            0x10035C8C, // Page Trousers
            0x10035FAC, // Page Garb (Altered)
         ]);
         armor_sets.insert("Night's Cavalry".to_string(),vec![
            0x10038270, // Night's Cavalry Helm
            0x100382D4, // Night's Cavalry Armor
            0x10038338, // Night's Cavalry Gauntlets
            0x1003839C, // Night's Cavalry Greaves
            0x10038658, // Night's Cavalry Helm (Altered)
            0x100386BC, // Night's Cavalry Armor (Altered)
         ]);
         armor_sets.insert("Blue Silver".to_string(),vec![
            0x1003A980, // Blue Silver Mail Hood
            0x1003A9E4, // Blue Silver Mail Armor
            0x1003AA48, // Blue Silver Bracelets
            0x1003AAAC, // Blue Silver Mail Skirt
            0x1003ADCC, // Blue Silver Mail Armor (Altered)
         ]);
         armor_sets.insert("Nomadic Merchant".to_string(),vec![
            0x1003D090, // Nomadic Merchant's Chapeau
            0x1003D0F4, // Nomadic Merchant's Finery
            0x1003D1BC, // Nomadic Merchant's Trousers
            0x1003D4DC, // Nomadic Merchant's Finery (Altered)
         ]);
         armor_sets.insert("Malformed Dragon".to_string(),vec![
            0x1003F7A0, // Malformed Dragon Helm
            0x1003F804, // Malformed Dragon Armor
            0x1003F868, // Malformed Dragon Gauntlets
            0x1003F8CC, // Malformed Dragon Greaves
         ]);
         armor_sets.insert("Tree Sentinel".to_string(),vec![
            0x10041EB0, // Tree Sentinel Helm
            0x10041F14, // Tree Sentinel Armor
            0x10041F78, // Tree Sentinel Gauntlets
            0x10041FDC, // Tree Sentinel Greaves
            0x100422FC, // Tree Sentinel Armor (Altered)
         ]);
         armor_sets.insert("Royal Knight".to_string(),vec![
            0x100445C0, // Royal Knight Helm
            0x10044624, // Royal Knight Armor
            0x10044688, // Royal Knight Gauntlets
            0x100446EC, // Royal Knight Greaves
            0x10044A0C, // Royal Knight Armor (Altered)
         ]);
         armor_sets.insert("Nox Monk".to_string(),vec![
            0x10046CD0, // Nox Monk Hood
            0x10046D34, // Nox Monk Armor
            0x10046D98, // Nox Monk Bracelets
            0x10046DFC, // Nox Monk Greaves
            0x100470B8, // Nox Monk Hood (Altered)
            0x1004711C, // Nox Monk Armor (Altered)
         ]);
         armor_sets.insert("Nox Swordstress Crown".to_string(),vec![
            0x100474A0, // Nox Swordstress Crown
            0x10047504, // Nox Swordstress Armor
            0x10047C70, // Nox Swordstress Crown (Altered)
            0x10047CD4, // Nox Swordstress Armor (Altered)
            0x10046D98, // Nox Monk Bracelets
            0x10046DFC, // Nox Monk Greaves
         ]);
         armor_sets.insert("Night Maiden".to_string(),vec![
            0x10047888, // Night Maiden Twin Crown
            0x100478EC, // Night Maiden Armor
         ]);
         armor_sets.insert("Fur".to_string(),vec![
            0x100493E0, // Great Horned Headband
            0x10049444, // Fur Raiment
            0x1004950C, // Fur Leggings
         ]);
         armor_sets.insert("Shaman".to_string(),vec![
            0x100497C8, // Shining Horned Headband
            0x1004982C, // Shaman Furs
            0x100498F4, // Shaman Leggings
         ]);
         armor_sets.insert("Duelist".to_string(),vec![
            0x1004BAF0, // Duelist Helm
            0x1004BB54, // Gravekeeper Cloak
            0x1004BC1C, // Duelist Greaves
            0x1004BF3C, // Gravekeeper Cloak (Altered)
         ]);
         armor_sets.insert("Sanguine Noble".to_string(),vec![
            0x1004E200, // Sanguine Noble Hood
            0x1004E264, // Sanguine Noble Robe
            0x1004E32C, // Sanguine Noble Waistcloth
         ]);
         armor_sets.insert("Guardian".to_string(),vec![
            0x10050910, // Guardian Mask
            0x10050974, // Guardian Garb (Full Bloom)
            0x100509D8, // Guardian Bracers
            0x10050A3C, // Guardian Greaves
            0x10050D5C, // Guardian Garb
         ]);
         armor_sets.insert("Cleanrot".to_string(),vec![
            0x10053020, // Cleanrot Helm
            0x10053084, // Cleanrot Armor
            0x100530E8, // Cleanrot Gauntlets
            0x1005314C, // Cleanrot Greaves
            0x10053408, // Cleanrot Helm (Altered)
            0x1005346C, // Cleanrot Armor (Altered)
         ]);
         armor_sets.insert("Fire Monk".to_string(),vec![
            0x10055730, // Fire Monk Hood
            0x10055794, // Fire Monk Armor
            0x100557F8, // Fire Monk Gauntlets
            0x1005585C, // Fire Monk Greaves
         ]);
         armor_sets.insert("Blackflame Monk".to_string(),vec![
            0x10055B18, // Blackflame Monk Hood
            0x10055B7C, // Blackflame Monk Armor
            0x10055BE0, // Blackflame Monk Gauntlets
            0x10055C44, // Blackflame Monk Greaves
         ]);
         armor_sets.insert("Fire Prelate".to_string(),vec![
            0x10057E40, // Fire Prelate Helm
            0x10057EA4, // Fire Prelate Armor
            0x10057F08, // Fire Prelate Gauntlets
            0x10057F6C, // Fire Prelate Greaves
            0x1005828C, // Fire Prelate Armor (Altered)
         ]);
         armor_sets.insert("Aristocrat".to_string(),vec![
            0x1005A550, // Aristocrat Headband
            0x1005A5B4, // Aristocrat Garb
            0x1005A67C, // Aristocrat Boots
            0x1005A99C, // Aristocrat Garb (Altered)
            0x1005CC60, // Aristocrat Hat
            0x1005CCC4, // Aristocrat Coat
         ]);
         armor_sets.insert("Old Aristocrat Cowl".to_string(),vec![
            0x1005F370, // Old Aristocrat Cowl
            0x1005F3D4, // Old Aristocrat Gown
            0x1005F49C, // Old Aristocrat Shoes
         ]);
         armor_sets.insert("Vulgar Militia".to_string(),vec![
            0x100668A0, // Vulgar Militia Helm
            0x10066904, // Vulgar Militia Armor
            0x10066968, // Vulgar Militia Gauntlets
            0x100669CC, // Vulgar Militia Greaves
         ]);
         armor_sets.insert("Sage".to_string(),vec![
            0x10068FB0, // Sage Hood
            0x10069014, // Sage Robe
            0x100690DC, // Sage Trousers
         ]);
         armor_sets.insert("Elden Lord".to_string(),vec![
            0x100704E0, // Elden Lord Crown
            0x10070544, // Elden Lord Armor
            0x100705A8, // Elden Lord Bracers
            0x1007060C, // Elden Lord Greaves
            0x1007092C, // Elden Lord Armor (Altered)
         ]);
         armor_sets.insert("General Radahn".to_string(),vec![
            0x10072BF0, // Radahn's Redmane Helm
            0x10072C54, // Radahn's Lion Armor
            0x10072CB8, // Radahn's Gauntlets
            0x10072D1C, // Radahn's Greaves
            0x1007303C, // Radahn's Lion Armor (Altered)
         ]);
         armor_sets.insert("Lord of Blood".to_string(),vec![
            0x10075364, // Lord of Blood's Robe
            0x1007574C, // Lord of Blood's Robe (Altered)
         ]);
         armor_sets.insert("Queen of the Full Moon".to_string(),vec![
            0x1007C830, // Queen's Crescent Crown
            0x1007C894, // Queen's Robe
            0x1007C8F8, // Queen's Bracelets
            0x1007C95C, // Queen's Leggings
         ]);
         armor_sets.insert("Godskin Apostle".to_string(),vec![
            0x1007EF40, // Godskin Apostle Hood
            0x1007EFA4, // Godskin Apostle Robe
            0x1007F008, // Godskin Apostle Bracelets
            0x1007F06C, // Godskin Apostle Trousers
         ]);
         armor_sets.insert("Godskin Noble".to_string(),vec![
            0x10081650, // Godskin Noble Hood
            0x100816B4, // Godskin Noble Robe
            0x10081718, // Godskin Noble Bracelets
            0x1008177C, // Godskin Noble Trousers
         ]);
         armor_sets.insert("Depraved Perfumer".to_string(),vec![
            0x10083D60, // Depraved Perfumer Headscarf
            0x10083DC4, // Depraved Perfumer Robe
            0x10083E28, // Depraved Perfumer Gloves
            0x10083E8C, // Depraved Perfumer Trousers
            0x100841AC, // Depraved Perfumer Robe (Altered)
         ]);
         armor_sets.insert("Crucible Axe".to_string(),vec![
            0x1008B290, // Crucible Axe Helm
            0x1008B2F4, // Crucible Axe Armor
            0x1008B358, // Crucible Gauntlets
            0x1008B3BC, // Crucible Greaves
         ]);
         armor_sets.insert("Crucible Tree".to_string(),vec![
            0x1008B678, // Crucible Tree Helm
            0x1008B6DC, // Crucible Tree Armor
            0x1008BAC4, // Crucible Axe Armor (Altered)
            0x1008BEAC, // Crucible Tree Armor (Altered)
         ]);
         armor_sets.insert("Lusat's".to_string(),vec![
            0x1008D9A0, // Lusat's Glintstone Crown
            0x1008DA04, // Lusat's Robe
            0x1008DA68, // Lusat's Manchettes
            0x1008DACC, // Old Sorcerer's Legwraps
         ]);
         armor_sets.insert("Azur's Glintstone".to_string(),vec![
            0x1008DD88, // Azur's Glintstone Crown
            0x1008DDEC, // Azur's Glintstone Robe
            0x1008DE50, // Azur's Manchettes
         ]);
         armor_sets.insert("All-Knowing".to_string(),vec![
            0x100900B0, // All-Knowing Helm
            0x10090114, // All-Knowing Armor
            0x10090178, // All-Knowing Gauntlets
            0x100901DC, // All-Knowing Greaves
            0x100904FC, // All-Knowing Armor (Altered)
         ]);
         armor_sets.insert("Twinned".to_string(),vec![
            0x100927C0, // Twinned Helm
            0x10092824, // Twinned Armor
            0x10092888, // Twinned Gauntlets
            0x100928EC, // Twinned Greaves
            0x10092C0C, // Twinned Armor (Altered)
         ]);
        //10094ED0, // Ragged Hat
        //10094ED0, // Ragged Hat
        //10094F34, // Ragged Armor
        //10094F98, // Ragged Gloves
        //10094FFC, // Ragged Loincloth
        //100952B8, // Ragged Hat
        //100952B8, // Ragged Hat (Altered)
        //1009531C, // Ragged Armor (Altered)
         armor_sets.insert("Prophet".to_string(),vec![
            0x100975E0, // Prophet Blindfold
            0x10097644, // Corhyn's Robe
            0x1009770C, // Prophet Trousers
            0x10097A2C, // Prophet Robe (Altered)
            0x10097E14, // Prophet Robe
         ]);
         armor_sets.insert("Astrologer".to_string(),vec![
            0x10099CF0, // Astrologer Hood
            0x10099D54, // Astrologer Robe
            0x10099DB8, // Astrologer Gloves
            0x10099E1C, // Astrologer Trousers
            0x1009A13C, // Astrologer Robe (Altered)
         ]);
         armor_sets.insert("Lionel's".to_string(),vec![
            0x1009C400, // Lionel's Helm
            0x1009C464, // Lionel's Armor
            0x1009C4C8, // Lionel's Gauntlets
            0x1009C52C, // Lionel's Greaves
            0x1009C84C, // Lionel's Armor (Altered)
         ]);
         armor_sets.insert("Hoslow's".to_string(),vec![
            0x1009EB10, // Hoslow's Helm
            0x1009EB74, // Hoslow's Armor
            0x1009EBD8, // Hoslow's Gauntlets
            0x1009EC3C, // Hoslow's Greaves
            0x1009EEF8, // Diallos's Mask
            0x1009F344, // Hoslow's Armor (Altered)
         ]);
         armor_sets.insert("Vagabond Knight".to_string(),vec![
            0x100A1220, // Vagabond Knight Helm
            0x100A1284, // Vagabond Knight Armor
            0x100A12E8, // Vagabond Knight Gauntlets
            0x100A134C, // Vagabond Knight Greaves
            0x100A166C, // Vagabond Knight Armor (Altered)
         ]);
         armor_sets.insert("Blue Cloth Cowl".to_string(),vec![
            0x100A3930, // Blue Cloth Cowl
            0x100A3994, // Blue Cloth Vest
            0x100A39F8, // Warrior Gauntlets
            0x100A3A5C, // Warrior Greaves
         ]);
         armor_sets.insert("White Mask".to_string(),vec![
            0x100A6040, // White Mask
            0x100A60A4, // War Surgeon Gown
            0x100A6108, // War Surgeon Gloves
            0x100A616C, // War Surgeon Trousers
            0x100A648C, // War Surgeon Gown (Altered)
         ]);
         armor_sets.insert("Royal Remains".to_string(),vec![
            0x100A8750, // Royal Remains Helm
            0x100A87B4, // Royal Remains Armor
            0x100A8818, // Royal Remains Gauntlets
            0x100A887C, // Royal Remains Greaves
         ]);
         armor_sets.insert("Beast Champion".to_string(),vec![
            0x100AFC80, // Beast Champion Helm
            0x100AFCE4, // Beast Champion Armor
            0x100AFD48, // Beast Champion Gauntlets
            0x100AFDAC, // Beast Champion Greaves
            0x100B00CC, // Beast Champion Armor (Altered)
         ]);
         armor_sets.insert("Champion".to_string(),vec![
            0x100B2390, // Champion Headband
            0x100B23F4, // Champion Pauldron
            0x100B2458, // Champion Bracers
            0x100B24BC, // Champion Gaiters
         ]);
         armor_sets.insert("Crimson".to_string(),vec![
            0x100B4E88, // Navy Hood
            0x100B4AA0, // Crimson Hood
            0x100B4B04, // Noble's Traveling Garb
            0x100B4B68, // Noble's Gloves
            0x100B4BCC, // Noble's Trousers
         ]);
         armor_sets.insert("Maliketh's".to_string(),vec![
            0x100B98C0, // Maliketh's Helm
            0x100B9924, // Maliketh's Armor
            0x100B9988, // Maliketh's Gauntlets
            0x100B99EC, // Maliketh's Greaves
            0x100B9D0C, // Maliketh's Armor (Altered)
         ]);
         armor_sets.insert("Malenia's".to_string(),vec![
            0x100BBFD0, // Malenia's Winged Helm
            0x100BC034, // Malenia's Armor
            0x100BC098, // Malenia's Gauntlet
            0x100BC0FC, // Malenia's Greaves
            0x100BC41C, // Malenia's Armor (Altered)
         ]);
         armor_sets.insert("Veteran's".to_string(),vec![
            0x100BE6E0, // Veteran's Helm
            0x100BE744, // Veteran's Armor
            0x100BE7A8, // Veteran's Gauntlets
            0x100BE80C, // Veteran's Greaves
            0x100BEB2C, // Veteran's Armor (Altered)
         ]);
         armor_sets.insert("Bloodhound Knight".to_string(),vec![
            0x100C0DF0, // Bloodhound Knight Helm
            0x100C0E54, // Bloodhound Knight Armor
            0x100C0EB8, // Bloodhound Knight Gauntlets
            0x100C0F1C, // Bloodhound Knight Greaves
            0x100C123C, // Bloodhound Knight Armor (Altered)
         ]);
         armor_sets.insert("Festive".to_string(),vec![
            0x100C3500, // Festive Hood
            0x100C3564, // Festive Garb
            0x100C38E8, // Festive Hood (Altered)
            0x100C394C, // Festive Garb (Altered)
         ]);
         armor_sets.insert("Blue Festive".to_string(),vec![
            0x100C3CD0, // Blue Festive Hood
            0x100C3D34, // Blue Festive Garb
         ]);
         armor_sets.insert("Commoner's".to_string(),vec![
            0x100C5C10, // Commoner's Headband
            0x100C5C74, // Commoner's Garb
            0x100C5D3C, // Commoner's Shoes
            0x100C5FF8, // Commoner's Headband (Altered)
            0x100C605C, // Commoner's Garb (Altered)
            0x100C63E0, // Commoner's Simple Garb
            0x100C6444, // Commoner's Simple Garb (Altered)
         ]);
         armor_sets.insert("Envoy Crown".to_string(),vec![
            0x100C8320, // Envoy Crown
         ]);
         armor_sets.insert("Raya Lucarian Sorcerer".to_string(),vec![
            0x100CAA30, // Twinsage Glintstone Crown
            0x100CAA94, // Raya Lucarian Robe
            0x100CAAF8, // Sorcerer Manchettes
            0x100CAB5C, // Sorcerer Leggings
         ]);
         armor_sets.insert("Glintstone Crowns".to_string(),vec![
            0x100CB200, // Lazuli Glintstone Crown
            0x100CAE18, // Olivinus Glintstone Crown
            0x100CB5E8, // Karolos Glintstone Crown
            0x100CB9D0, // Witch's Glintstone Crown
         ]);
         armor_sets.insert("Marionette Soldier".to_string(),vec![
            0x100CD140, // Marionette Soldier Helm
            0x100CD1A4, // Marionette Soldier Armor
            0x100CF850, // Marionette Soldier Birdhelm
         ]);
         armor_sets.insert("Raging Wolf".to_string(),vec![
            0x100D1F60, // Raging Wolf Helm
            0x100D1FC4, // Raging Wolf Armor
            0x100D2028, // Raging Wolf Gauntlets
            0x100D208C, // Raging Wolf Greaves
            0x100D23AC, // Raging Wolf Armor (Altered)
         ]);
         armor_sets.insert("Land of Reeds".to_string(),vec![
            0x100D4670, // Land of Reeds Helm
            0x100D46D4, // Land of Reeds Armor
            0x100D4738, // Land of Reeds Gauntlets
            0x100D479C, // Land of Reeds Greaves
            0x100D4ABC, // Land of Reeds Armor (Altered)
         ]);
         armor_sets.insert("White Reed".to_string(),vec![
            0x100D4E40, // Okina Mask
            0x100D4EA4, // White Reed Armor
            0x100D4F08, // White Reed Gauntlets
            0x100D4F6C, // White Reed Greaves
         ]);
         armor_sets.insert("Confessor".to_string(),vec![
            0x100D6D80, // Confessor Hood
            0x100D6DE4, // Confessor Armor
            0x100D6E48, // Confessor Gloves
            0x100D6EAC, // Confessor Boots
            0x100D7168, // Confessor Hood (Altered)
            0x100D71CC, // Confessor Armor (Altered)
         ]);
         armor_sets.insert("Prisoner".to_string(),vec![
            0x100D9490, // Prisoner Iron Mask
            0x100D94F4, // Prisoner Clothing
            0x100D95BC, // Prisoner Trousers
         ]);
         armor_sets.insert("Traveling Maiden".to_string(),vec![
            0x100DBBA0, // Traveling Maiden Hood
            0x100DBC04, // Traveling Maiden Robe
            0x100DBC68, // Traveling Maiden Gloves
            0x100DBCCC, // Traveling Maiden Boots
            0x100DBFEC, // Traveling Maiden Robe (Altered)
         ]);
         armor_sets.insert("Finger Maiden".to_string(),vec![
            0x100DC370, // Finger Maiden Fillet
            0x100DC3D4, // Finger Maiden Robe
            0x100DC49C, // Finger Maiden Shoes
            0x100DC7BC, // Finger Maiden Robe (Altered)
         ]);
         armor_sets.insert("Preceptor's Big Hat".to_string(),vec![
            0x100DE2B0, // Preceptor's Big Hat
            0x100DE314, // Preceptor's Long Gown
            0x100DE378, // Preceptor's Gloves
            0x100DE3DC, // Preceptor's Trousers
            0x100DE698, // Mask of Confidence
            0x100DE6FC, // Preceptor's Long Gown (Altered)
         ]);
         armor_sets.insert("Raptor".to_string(),vec![
            0x100E30D0, // Skeletal Mask
            0x100E3134, // Raptor's Black Feathers
            0x100E3198, // Bandit Manchettes
            0x100E31FC, // Bandit Boots
         ]);
         armor_sets.insert("Bandit".to_string(),vec![
            0x101560A8, // Bandit Mask
            0x100E351C, // Bandit Garb
            0x100E3198, // Bandit Manchettes
            0x100E31FC, // Bandit Boots
         ]);
         armor_sets.insert("Eccentric".to_string(),vec![
            0x100E57E0, // Eccentric's Hood
            0x100E5844, // Eccentric's Armor
            0x100E58A8, // Eccentric's Manchettes
            0x100E590C, // Eccentric's Breeches
            0x100E5BC8, // Eccentric's Hood (Altered)
         ]);
         armor_sets.insert("Fingerprint".to_string(),vec![
            0x100E7EF0, // Fingerprint Helm
            0x100E7F54, // Fingerprint Armor
            0x100E7FB8, // Fingerprint Gauntlets
            0x100E801C, // Fingerprint Greaves
            0x100E833C, // Fingerprint Armor (Altered)
         ]);
         armor_sets.insert("Consort's".to_string(),vec![
            0x100EA600, // Consort's Mask
            0x100EA664, // Consort's Robe
            0x100EA72C, // Consort's Trousers
         ]);
         armor_sets.insert("Ruler's".to_string(),vec![
            0x100EA9E8, // Ruler's Mask
            0x100EAA4C, // Ruler's Robe
            0x100EAE34, // Upper-Class Robe
         ]);
         armor_sets.insert("House Marais".to_string(),vec![
            0x100EB1B8, // Marais Mask
            0x100EB21C, // Marais Robe
         ]);
         armor_sets.insert("Bloodsoaked Mask".to_string(),vec![
             0x100EB5A0, // Bloodsoaked Mask
             0x100EB604, // Official's Attire
             0x100EB280, // Bloodsoaked Manchettes
         ]);
         armor_sets.insert("Omen".to_string(),vec![
            0x100ECD10, // Omen Helm
            0x100ECD74, // Omen Armor
            0x100ECDD8, // Omen Gauntlets
            0x100ECE3C, // Omen Greaves
         ]);
         armor_sets.insert("Carian Knight".to_string(),vec![
            0x100EF420, // Carian Knight Helm
            0x100EF484, // Carian Knight Armor
            0x100EF4E8, // Carian Knight Gauntlets
            0x100EF54C, // Carian Knight Greaves
            0x100EF86C, // Carian Knight Armor (Altered)
         ]);
         armor_sets.insert("Hierodas Glintstone Crown".to_string(),vec![
            0x100F1B30, // Hierodas Glintstone Crown
         ]);
         armor_sets.insert("Errant".to_string(),vec![
            0x100F1B94, // Errant Sorcerer Robe
            0x100F1BF8, // Errant Sorcerer Manchettes
            0x100F1C5C, // Errant Sorcerer Boots
            0x100F1F7C, // Errant Sorcerer Robe (Altered)
         ]);
         armor_sets.insert("Battlemage".to_string(),vec![
            0x100F4240, // Haima Glintstone Crown
            0x100F42A4, // Battlemage Robe
            0x100F4308, // Battlemage Manchettes
            0x100F436C, // Battlemage Legwraps
         ]);
         armor_sets.insert("Snow Witch".to_string(),vec![
            0x100F6950, // Snow Witch Hat
            0x100F69B4, // Snow Witch Robe
            0x100F6A7C, // Snow Witch Skirt
            0x100F6D9C, // Snow Witch Robe (Altered)
         ]);
         armor_sets.insert("Traveler".to_string(),vec![
            0x100F90C4, // Traveler's Clothes
            0x100F9128, // Traveler's Manchettes
            0x100F918C, // Traveler's Boots
         ]);
         armor_sets.insert("Juvenile Scholar".to_string(),vec![
            0x100FB770, // Juvenile Scholar Cap
            0x100FB7D4, // Juvenile Scholar Robe
         ]);
         armor_sets.insert("Radiant Gold Mask".to_string(),vec![
            0x100FDE80, // Radiant Gold Mask
         ]);
         armor_sets.insert("Goldmask".to_string(),vec![
            0x100FDEE4, // Goldmask's Rags
            0x100FDF48, // Gold Bracelets
            0x100FDFAC, // Gold Waistwrap
         ]);
         armor_sets.insert("Albinauric(Lilbro)".to_string(),vec![
            0x10102CA0, // Albinauric Mask
            0x10102D04, // Dirty Chainmail
         ]);
         armor_sets.insert("Zamor".to_string(),vec![
            0x101053B0, // Zamor Mask
            0x10105414, // Zamor Armor
            0x10105478, // Zamor Bracelets
            0x101054DC, // Zamor Legwraps
         ]);
         armor_sets.insert("Imp".to_string(),vec![
            0x10107AC0, // Imp Head (Cat)
            0x10107EA8, // Imp Head (Fanged)
            0x10108290, // Imp Head (Long-Tongued)
            0x10108678, // Imp Head (Corpse)
            0x10108A60, // Imp Head (Wolf)
            0x10108E48, // Imp Head (Elder)
         ]);
         armor_sets.insert("Silver Tear Mask".to_string(),vec![
            0x1010A1D0, // Silver Tear Mask
         ]);
         armor_sets.insert("Chain".to_string(),vec![
            0x1010C8E0, // Chain Coif
            0x1010C944, // Chain Armor
            0x1010C9A8, // Chain Gauntlets
            0x1010CA0C, // Chain Leggings
         ]);
         armor_sets.insert("Mushroom Head".to_string(),vec![
            0x101EAB90, // Mushroom Crown
            0x10113E10, // Mushroom Head
            0x10113E74, // Mushroom Body
            0x10113ED8, // Mushroom Arms
            0x10113F3C, // Mushroom Legs
         ]);
         armor_sets.insert("Leather".to_string(),vec![
            0x10155CC0, // Black Hood
            0x10155D24, // Leather Armor
            0x10155D88, // Leather Gloves
            0x10155DEC, // Leather Boots
         ]);
         armor_sets.insert("Knight".to_string(),vec![
            0x1016E360, // Knight Helm
            0x1016E3C4, // Knight Armor
            0x1016E428, // Knight Gauntlets
            0x1016E48C, // Knight Greaves
         ]);
         armor_sets.insert("Godrick Soldier".to_string(),vec![
            0x1019F0A0, // Godrick Soldier Helm
            0x1019F104, // Tree-and-Beast Surcoat
            0x1019F168, // Godrick Soldier Gauntlets
            0x1019F1CC, // Godrick Soldier Greaves
         ]);
         armor_sets.insert("Raya Lucarian Soldier".to_string(),vec![
            0x101A17B0, // Raya Lucarian Helm
            0x101A1814, // Cuckoo Surcoat
            0x101A1878, // Raya Lucarian Gauntlets
            0x101A18DC, // Raya Lucarian Greaves
         ]);
         armor_sets.insert("Leyndell Soldier".to_string(),vec![
            0x101A3EC0, // Leyndell Soldier Helm
            0x101A3F24, // Erdtree Surcoat
            0x101A3F88, // Leyndell Soldier Gauntlets
            0x101A3FEC, // Leyndell Soldier Greaves
         ]);
         armor_sets.insert("Radahn Soldier".to_string(),vec![
            0x101A65D0, // Radahn Soldier Helm
            0x101A6634, // Redmane Surcoat
            0x101A6698, // Radahn Soldier Gauntlets
            0x101A66FC, // Radahn Soldier Greaves
         ]);
         armor_sets.insert("Mausoleum Soldier".to_string(),vec![
            0x101A8D44, // Mausoleum Surcoat
            0x101A8DA8, // Mausoleum Gauntlets
            0x101A8E0C, // Mausoleum Greaves
         ]);
         armor_sets.insert("Haligtree Soldier".to_string(),vec![
            0x101AB3F0, // Haligtree Helm
            0x101AB454, // Haligtree Crest Surcoat
            0x101AB4B8, // Haligtree Gauntlets
            0x101AB51C, // Haligtree Greaves
         ]);
         armor_sets.insert("Gelmir Knight".to_string(),vec![
            0x101ADB00, // Gelmir Knight Helm
            0x101ADB64, // Gelmir Knight Armor
            0x101ADBC8, // Gelmir Knight Gauntlets
            0x101ADC2C, // Gelmir Knight Greaves
            0x101ADF4C, // Gelmir Knight Armor (Altered)
         ]);
         armor_sets.insert("Godrick Knight".to_string(),vec![
            0x101B0210, // Godrick Knight Helm
            0x101B0274, // Godrick Knight Armor
            0x101B02D8, // Godrick Knight Gauntlets
            0x101B033C, // Godrick Knight Greaves
            0x101B065C, // Godrick Knight Armor (Altered)
         ]);
         armor_sets.insert("Cuckoo Knight".to_string(),vec![
            0x101B2920, // Cuckoo Knight Helm
            0x101B2984, // Cuckoo Knight Armor
            0x101B29E8, // Cuckoo Knight Gauntlets
            0x101B2A4C, // Cuckoo Knight Greaves
            0x101B2D6C, // Cuckoo Knight Armor (Altered)
         ]);
         armor_sets.insert("Leyndell Knight".to_string(),vec![
            0x101B5030, // Leyndell Knight Helm
            0x101B5094, // Leyndell Knight Armor
            0x101B50F8, // Leyndell Knight Gauntlets
            0x101B515C, // Leyndell Knight Greaves
            0x101B547C, // Leyndell Knight Armor (Altered)
         ]);
         armor_sets.insert("Redmane Knight".to_string(),vec![
            0x101B7740, // Redmane Knight Helm
            0x101B77A4, // Redmane Knight Armor
            0x101B7808, // Redmane Knight Gauntlets
            0x101B786C, // Redmane Knight Greaves
            0x101B7B8C, // Redmane Knight Armor (Altered)
         ]);
         armor_sets.insert("Mausoleum Knight".to_string(),vec![
            0x101B9EB4, // Mausoleum Knight Armor
            0x101B9F18, // Mausoleum Knight Gauntlets
            0x101B9F7C, // Mausoleum Knight Greaves
            0x101BA29C, // Mausoleum Knight Armor (Altered)
         ]);
         armor_sets.insert("Haligtree Knight".to_string(),vec![
            0x101BC560, // Haligtree Knight Helm
            0x101BC5C4, // Haligtree Knight Armor
            0x101BC628, // Haligtree Knight Gauntlets
            0x101BC68C, // Haligtree Knight Greaves
            0x101BC9AC, // Haligtree Knight Armor (Altered)
         ]);
         armor_sets.insert("Godrick Foot Soldier".to_string(),vec![
            0x101BEC70, // Foot Soldier Cap
            0x101BECD4, // Chain-Draped Tabard
            0x101BED38, // Foot Soldier Gauntlets
            0x101BED9C, // Foot Soldier Greaves
         ]);
         armor_sets.insert("Leyndell Foot Soldier".to_string(),vec![
            0x101C3A90, // Gilded Foot Soldier Cap
            0x101C1380, // Foot Soldier Helmet
            0x101C13E4, // Foot Soldier Tabard
            0x101C3AF4, // Leather-Draped Tabard
         ]);
         armor_sets.insert("Mausoleum Foot Soldier".to_string(),vec![
            0x101C61A0, // Foot Soldier Helm
            0x101C8914, // Bloodsoaked Tabard
            0x101BED38, // Foot Soldier Gauntlets
            0x101BED9C, // Foot Soldier Greaves
         ]);
         armor_sets.insert("Haligtree Foot Soldier".to_string(),vec![
            0x101CAFC0, // Sacred Crown Helm
            0x101CB024, // Ivory-Draped Tabard
            0x101BED38, // Foot Soldier Gauntlets
            0x101BED9C, // Foot Soldier Greaves
         ]);
         armor_sets.insert("Omensmirk Mask".to_string(),vec![
            0x101CD6D0, // Omensmirk Mask
            0x101CD734, // Omenkiller Robe
            0x101CD798, // Omenkiller Long Gloves
            0x101CD7FC, // Omenkiller Boots
         ]);
         armor_sets.insert("Scarab".to_string(),vec![
            0x101CFDE0, // Ash-of-War Scarab
            0x101D01C8, // Incantation Scarab
            0x101D05B0, // Glintstone Scarab
            0x101D24F0, // Crimson Tear Scarab
            0x101D4C00, // Cerulean Tear Scarab
         ]);
         armor_sets.insert("Deathbed".to_string(),vec![
            0x101D7374, // Deathbed Dress
            //101D743C, // Deathbed Smalls
         ]);
         armor_sets.insert("Fia's".to_string(),vec![
            0x101D9A20, // Fia's Hood
            0x101D9A84, // Fia's Robe
            0x101D9E6C, // Fia's Robe (Altered)
         ]);
        //101DC194, // Millicent's Robe
        //101DC1F8, // Millicent's Gloves
        //101DC25C, // Millicent's Boots
        //101E0FB4, // Millicent's Tunic
        //101E1018, // Golden Prosthetic
         armor_sets.insert("Highwayman".to_string(),vec![
            0x101E3660, // Highwayman Hood
            0x101E36C4, // Highwayman Cloth Armor
            0x101E3728, // Highwayman Gauntlets
         ]);
         armor_sets.insert("High Page".to_string(),vec![
            0x101E5D70, // High Page Hood
            0x101E5DD4, // High Page Clothes
            0x101E61BC, // High Page Clothes (Altered)
         ]);
         armor_sets.insert("Rotten Duelist".to_string(),vec![
            0x101E8480, // Rotten Duelist Helm
            0x101E84E4, // Rotten Gravekeeper Cloak
            0x101E85AC, // Rotten Duelist Greaves
            0x101E88CC, // Rotten Gravekeeper Cloak (Altered)
         ]);
         armor_sets.insert("Lazuli".to_string(),vec![
            0x100CB200, // Lazuli Glintstone Crown
            0x101EF9B0, // Lazuli Robe
            0x100CAAF8, // Sorcerer Manchettes
            0x100CAB5C, // Sorcerer Leggings
         ]);
         armor_sets.insert("_Not part of a set".to_string(), vec![
            0x1010CD2C, // Eye Surcoatvec
            0x1010CCC8, // Greathelm
            0x1010D114, // Tree Surcoat
            0x1010EFF0, // Octopus Head
            0x10111700, // Jar
            0x10186A00, // Greathood
            0x1013D620, // Nox Mirrorhelm
            0x1013DA08, // Iji's Mirrorhelm
            0x101ED2A0, // Black Dumpling
            0x101005F4, // Fell Omen Cloak
            0x100249F0, // Iron Kasa
            0x1006B6C0, // Pumpkin Helm
            0x100D9878, // Blackguard's Iron Mask
         ]);
        armor_sets
    })
}