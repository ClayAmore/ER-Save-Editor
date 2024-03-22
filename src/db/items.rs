// Modified from TGA table MassItemGib 
// https://github.com/The-Grand-Archives/Elden-Ring-CT-TGA
use std::collections::BTreeMap;

use once_cell::sync::OnceCell;

pub fn items() -> &'static BTreeMap<String, Vec<u32>> {
    static ITEMS: OnceCell<BTreeMap<String, Vec<u32>>> = OnceCell::new();
    ITEMS.get_or_init(|| {
        let mut item_groups = BTreeMap::new();
        item_groups.insert("Consumables".to_string(), vec![
            0x400003AC, // Preserving Boluses
            0x400003B6, // Rejuvenating Boluses
            0x400003C0, // Clarifying Boluses
            0x400003A2, // Stimulating Boluses
            0x40000398, // Thawfrost Boluses
            0x4000038E, // Stanching Boluses
            0x40000384, // Neutralizing Boluses
            0x400000BE, // Rune Arc
            0x40000096, // Furlcalling Finger Remedy
            0x40000334, // Boiled Crab
            0x40000D21, // Radiant Baldachin's Blessing
            0x4000032C, // Frozen Raisin
            0x4000050A, // Starlight Shards
            0x40000CEE, // Warming Stone
            0x40000CEF, // Frenzyflame Stone
            0x40000CF8, // Scriptstone
            0x40000D16, // Bewitching Branch
            0x400004A6, // Silver-Pickled Fowl Foot
            0x400004B0, // Gold-Pickled Fowl Foot
            0x400006E0, // Ruin Fragment
            0x40000B67, // Lord's Rune
            0x40001F40, // Stonesword Key
            0x40000834, // Soft Cotton
            0x40000802, // Grace Mimic
        ]);
        item_groups.insert("Meats".to_string(), vec![
            0x4000044C, // Pickled Turtle Neck
            0x40000460, // Invigorating Cured Meat
            0x40000456, // Immunizing Cured Meat
            0x4000046A, // Clarifying Cured Meat
            0x40000474, // Dappled Cured Meat
            0x400004BA, // Exalted Flesh
            //0x400004C4, // Deathsbane Jerky
            0x400004D3, // Raw Meat Dumpling
            0x4000051E, // Immunizing White Cured Meat
            0x40000528, // Invigorating White Cured Meat
            0x40000532, // Clarifying White Cured Meat
            0x4000053C, // Dappled White Cured Meat
            //0x40000546, // Deathsbane White Jerky
            0x4000047E, // Spellproof Dried Liver
            0x40000488, // Fireproof Dried Liver
            0x40000492, // Lightningproof Dried Liver
            0x4000049C, // Holyproof Dried Liver
        ]);
        item_groups.insert("Throwables".to_string(), vec![
            0x400006A4, // Throwing Dagger
            0x400006AE, // Bone Dart
            0x400006B8, // Poisonbone Dart
            0x400006C2, // Kukri
            0x400006CC, // Crystal Dart
            0x400006D6, // Fan Daggers
            0x40000726, // Explosive Stone
            0x40000727, // Explosive Stone Clump
            0x40000730, // Poisoned Stone
            0x40000731, // Poisoned Stone Clump
            0x400007E4, // Rainbow Stone
            0x400007EE, // Glowstone
            //0x40000BCC, // Miranda's Prayer
            0x40000BD6, // Cuckoo Glintstone
            0x40000BEA, // Glintstone Scrap
            0x40000BEB, // Large Glintstone Scrap
            0x40000BF4, // Gravity Stone Fan
            0x40000BFE, // Gravity Stone Chunk
        ]);
        item_groups.insert("Grease".to_string(), vec![
            0x40000578, // Fire Grease
            0x40000582, // Lightning Grease
            0x4000058C, // Magic Grease
            0x40000596, // Holy Grease
            0x400005A0, // Blood Grease
            0x400005AA, // Soporific Grease
            0x400005B4, // Poison Grease
            0x400005BE, // Freezing Grease
            0x400005C8, // Dragonwound Grease
            0x400005D2, // Rot Grease
            0x400005DC, // Drawstring Fire Grease
            0x400005E6, // Drawstring Lightning Grease
            0x400005F0, // Drawstring Magic Grease
            0x400005FA, // Drawstring Holy Grease
            0x40000604, // Drawstring Blood Grease
            0x4000060E, // Drawstring Soporific Grease
            0x40000618, // Drawstring Poison Grease
            //0x40000622, // Drawstring Freezing Grease
            0x40000636, // Drawstring Rot Grease
            0x4000069A, // Shield Grease
            //0x40000CE4, // Holy Water Grease
        ]);
        item_groups.insert("Crafting: Animal".to_string(), vec![
            0x40003A98, // Sliver of Meat
            0x40003AA2, // Beast Liver
            0x40003AAC, // Lump of Flesh
            0x40003AB6, // Beast Blood
            0x40003AC0, // Old Fang
            0x40003ACA, // Budding Horn
            0x40003AD4, // Flight Pinion
            0x40003AE8, // Four-Toed Fowl Foot
            0x40003AF2, // Turtle Neck Meat
            0x40003AFC, // Human Bone Shard
            0x40003B06, // Great Dragonfly Head
            0x40003B10, // Slumbering Egg
            0x40003B1A, // Crab Eggs
            0x40003B24, // Land Octopus Ovary
            0x40003B38, // Strip of White Flesh
            0x40003BEC, // Thin Beast Bones
            0x40003BED, // Hefty Beast Bone
            0x40003C32, // Living Jar Shard
            0x40003C3C, // Albinauric Bloodclot
            0x40003C46, // Stormhawk Feather
            0x40005140, // Nascent Butterfly
            0x40005141, // Aeonian Butterfly
            0x40005142, // Smoldering Butterfly
            0x4000514A, // Silver Firefly
            0x4000514B, // Gold Firefly
            0x4000514C, // Glintstone Firefly
            0x40005154, // Golden Centipede
            0x40005159, // Silver Tear Husk
            0x4000515E, // Gold-Tinged Excrement
            0x4000515F, // Blood-Tainted Excrement
            0x4000516D, // Yellow Ember
        ]);
        item_groups.insert("Crafting: Plant".to_string(), vec![
            0x40003B2E, // Miranda Powder
            0x400050AA, // Poisonbloom
            0x400050AB, // Trina's Lily
            0x400050AC, // Fulgurbloom
            0x400050AD, // Miquella's Lily
            0x400050AE, // Grave Violet
            0x400050B4, // Faded Erdleaf Flower
            0x400050C8, // Erdleaf Flower
            0x400050C9, // Altus Bloom
            0x400050CA, // Fire Blossom
            0x400050CB, // Golden Sunflower
            0x400050CD, // Tarnished Golden Sunflower
            0x400050D2, // Herba
            0x400050D3, // Arteria Leaf
            0x400050E6, // Dewkissed Herba
            0x400050F0, // Rowa Fruit
            0x400050F1, // Golden Rowa
            0x400050F2, // Rimed Rowa
            0x400050F3, // Bloodrose
            0x40005104, // Eye of Yelough
            0x4000510E, // Crystal Bud
            0x4000510F, // Rimed Crystal Bud
            0x40005111, // Sacramental Bud
            0x40005118, // Mushroom
            0x40005119, // Melted Mushroom
            0x40005122, // Toxic Mushroom
            0x40005127, // Root Resin
            0x40005168, // Cave Moss
            0x40005169, // Budding Cave Moss
            0x4000516A, // Crystal Cave Moss
        ]);
        item_groups.insert("Crafting: Inorganic".to_string(), vec![
            0x400006E0, // Ruin Fragment
            0x40003C28, // String
            0x4000512C, // Cracked Crystal
            0x4000513B, // Sanctuary Stone
            0x40005172, // Volcanic Stone
            0x40005174, // Formic Rock
            0x40005177, // Gravel Stone
        ]);
        item_groups.insert("Smithing Stone".to_string(), vec![
            0x40002774, // Smithing Stone [1]
            0x40002775, // Smithing Stone [2]
            0x40002776, // Smithing Stone [3]
            0x40002777, // Smithing Stone [4]
            0x40002778, // Smithing Stone [5]
            0x40002779, // Smithing Stone [6]
            0x4000277A, // Smithing Stone [7]
            0x4000277B, // Smithing Stone [8]
            0x4000279C, // Ancient Dragon Smithing Stone
        ]);
        item_groups.insert("Somber Smithing Stone".to_string(), vec![
            0x400027B0, // Somber Smithing Stone [1]
            0x400027B1, // Somber Smithing Stone [2]
            0x400027B2, // Somber Smithing Stone [3]
            0x400027B3, // Somber Smithing Stone [4]
            0x400027B4, // Somber Smithing Stone [5]
            0x400027B5, // Somber Smithing Stone [6]
            0x400027B6, // Somber Smithing Stone [7]
            0x400027B7, // Somber Smithing Stone [8]
            0x400027B8, // Somber Ancient Dragon Smithing Stone
            0x400027D8, // Somber Smithing Stone [9]
        ]);
        item_groups.insert("Grave Glovewort".to_string(), vec![
            0x40002A94, // Grave Glovewort [1]
            0x40002A95, // Grave Glovewort [2]
            0x40002A96, // Grave Glovewort [3]
            0x40002A97, // Grave Glovewort [4]
            0x40002A98, // Grave Glovewort [5]
            0x40002A99, // Grave Glovewort [6]
            0x40002A9A, // Grave Glovewort [7]
            0x40002A9B, // Grave Glovewort [8]
            0x40002A9C, // Grave Glovewort [9]
            0x40002A9D, // Great Grave Glovewort
        ]);
        item_groups.insert("Ghost Glovewort".to_string(), vec![
            0x40002A9E, // Ghost Glovewort [1]
            0x40002A9F, // Ghost Glovewort [2]
            0x40002AA0, // Ghost Glovewort [3]
            0x40002AA1, // Ghost Glovewort [4]
            0x40002AA2, // Ghost Glovewort [5]
            0x40002AA3, // Ghost Glovewort [6]
            0x40002AA4, // Ghost Glovewort [7]
            0x40002AA5, // Ghost Glovewort [8]
            0x40002AA6, // Ghost Glovewort [9]
            0x40002AA7, // Great Ghost Glovewort
        ]);
        item_groups.insert("Perfumes".to_string(), vec![
            0x40000DAC, // Uplifting Aromatic
            0x40000DB6, // Spark Aromatic
            0x40000DC0, // Ironjar Aromatic
            0x40000DDE, // Bloodboil Aromatic
            0x40000DFC, // Poison Spraymist
            0x40000E1A, // Acid Spraymist
            0x40000DC0, // Ironjar Aromatic
        ]);
        item_groups.insert("Pots".to_string(), vec![
            0x4000012C, // Fire Pot
            0x4000012D, // Redmane Fire Pot
            0x4000012E, // Giantsflame Fire Pot
            0x40000140, // Lightning Pot
            0x40000141, // Ancient Dragonbolt Pot
            0x4000014A, // Fetid Pot
            0x4000015E, // Holy Water Pot
            0x4000015F, // Sacred Order Pot
            0x40000168, // Freezing Pot
            0x40000172, // Poison Pot
            0x4000017C, // Oil Pot
            0x40000186, // Alluring Pot
            0x40000187, // Beastlure Pot
            0x40000190, // Roped Fire Pot
            0x40000154, // Swarm Pot
            0x40000280, // Sleep Pot
            0x400001A4, // Roped Lightning Pot
            0x400001AE, // Roped Fetid Pot
            0x400001B8, // Roped Poison Pot
            0x400001C2, // Roped Oil Pot
            0x400001CC, // Roped Magic Pot
            0x400001D6, // Roped Fly Pot
            0x400001EA, // Roped Volcano Pot
            0x400001FE, // Roped Holy Water Pot
            0x40000258, // Volcano Pot
            0x40000262, // Albinauric Pot
            0x40000276, // Cursed-Blood Pot
            0x40000294, // Magic Pot
            0x40000295, // Academy Magic Pot
        ]);
        item_groups.insert("Crystal tears".to_string(), vec![
            0x40002AF8, // Crimsonspill Crystal Tear
            0x40002AF9, // Greenspill Crystal Tear
            0x40002AFA, // Crimson Crystal Tear
            0x40002AFB, // Crimson Crystal Tear
            0x40002AFC, // Cerulean Crystal Tear
            0x40002AFD, // Cerulean Crystal Tear
            0x40002AFE, // Speckled Hardtear
            0x40002AFF, // Crimson Bubbletear
            0x40002B00, // Opaline Bubbletear
            0x40002B01, // Crimsonburst Crystal Tear
            0x40002B02, // Greenburst Crystal Tear
            0x40002B03, // Opaline Hardtear
            0x40002B04, // Winged Crystal Tear
            0x40002B05, // Thorny Cracked Tear
            0x40002B06, // Spiked Cracked Tear
            0x40002B07, // Windy Crystal Tear
            0x40002B08, // Ruptured Crystal Tear
            0x40002B09, // Ruptured Crystal Tear
            0x40002B0A, // Leaden Hardtear
            0x40002B0B, // Twiggy Cracked Tear
            0x40002B0C, // Crimsonwhorl Bubbletear
            0x40002B0D, // Strength-knot Crystal Tear
            0x40002B0E, // Dexterity-knot Crystal Tear
            0x40002B0F, // Intelligence-knot Crystal Tear
            0x40002B10, // Faith-knot Crystal Tear
            0x40002B11, // Cerulean Hidden Tear
            0x40002B12, // Stonebarb Cracked Tear
            0x40002B13, // Purifying Crystal Tear
            0x40002B14, // Flame-Shrouding Cracked Tear
            0x40002B15, // Magic-Shrouding Cracked Tear
            0x40002B16, // Lightning-Shrouding Cracked Tear
            0x40002B17, // Holy-Shrouding Cracked Tear
        ]);
        item_groups.insert("Prattling Pates".to_string(), vec![
            0x40000898, // Prattling Pate "Hello"
            0x40000899, // Prattling Pate "Thank you"
            0x4000089A, // Prattling Pate "Apologies"
            0x4000089B, // Prattling Pate "Wonderful"
            0x4000089C, // Prattling Pate "Please help"
            0x4000089D, // Prattling Pate "My beloved"
            0x4000089E, // Prattling Pate "Let's get to it"
            0x4000089F, // Prattling Pate "You're beautiful"
        ]);
        item_groups.insert("Bell Bearings".to_string(), vec![
            0x400022CE, // Pidia's Bell Bearing
            0x400022CF, // Seluvis's Bell Bearing
            0x400022D0, // Patches' Bell Bearing
            0x400022D1, // Sellen's Bell Bearing
            0x400022D3, // D's Bell Bearing
            0x400022D4, // Bernahl's Bell Bearing
            0x400022D5, // Miriel's Bell Bearing
            0x400022D6, // Gostoc's Bell Bearing
            0x400022D7, // Thops's Bell Bearing
            0x400022D8, // Kalé's Bell Bearing
            0x400022D9, // Nomadic Merchant's Bell Bearing [1]
            0x400022DA, // Nomadic Merchant's Bell Bearing [2]
            0x400022DB, // Nomadic Merchant's Bell Bearing [3]
            0x400022DC, // Nomadic Merchant's Bell Bearing [4]
            0x400022DD, // Nomadic Merchant's Bell Bearing [5]
            0x400022DE, // Isolated Merchant's Bell Bearing [1]
            0x400022DF, // Isolated Merchant's Bell Bearing [2]
            0x400022E0, // Nomadic Merchant's Bell Bearing [6]
            0x400022E1, // Hermit Merchant's Bell Bearing [1]
            0x400022E2, // Nomadic Merchant's Bell Bearing [7]
            0x400022E3, // Nomadic Merchant's Bell Bearing [8]
            0x400022E4, // Nomadic Merchant's Bell Bearing [9]
            0x400022E5, // Nomadic Merchant's Bell Bearing [10]
            //dd 400022E6 00000001 00000000 FFFFFFFF // Nomadic Merchant's Bell Bearing [11]
            0x400022E7, // Isolated Merchant's Bell Bearing [3]
            0x400022E8, // Hermit Merchant's Bell Bearing [2]
            0x400022E9, // Abandoned Merchant's Bell Bearing
            0x400022EA, // Hermit Merchant's Bell Bearing [3]
            0x400022EB, // Imprisoned Merchant's Bell Bearing
            0x400022EC, // Iji's Bell Bearing
            0x400022ED, // Rogier's Bell Bearing
            0x400022EE, // Blackguard's Bell Bearing
            0x400022EF, // Corhyn's Bell Bearing
            0x400022F0, // Gowry's Bell Bearing
            0x400022F1, // Bone Peddler's Bell Bearing
            0x400022F2, // Meat Peddler's Bell Bearing
            0x400022F3, // Medicine Peddler's Bell Bearing
            0x400022F4, // Gravity Stone Peddler's Bell Bearing
            0x400022F7, // Smithing-Stone Miner's Bell Bearing [1]
            0x400022F8, // Smithing-Stone Miner's Bell Bearing [2]
            0x400022F9, // Smithing-Stone Miner's Bell Bearing [3]
            0x400022FA, // Smithing-Stone Miner's Bell Bearing [4]
            0x400022FB, // Somberstone Miner's Bell Bearing [1]
            0x400022FC, // Somberstone Miner's Bell Bearing [2]
            0x400022FD, // Somberstone Miner's Bell Bearing [3]
            0x400022FE, // Somberstone Miner's Bell Bearing [4]
            0x400022FF, // Somberstone Miner's Bell Bearing [5]
            0x40002300, // Glovewort Picker's Bell Bearing [1]
            0x40002301, // Glovewort Picker's Bell Bearing [2]
            0x40002302, // Glovewort Picker's Bell Bearing [3]
            0x40002303, // Ghost-Glovewort Picker's Bell Bearing [1]
            0x40002304, // Ghost-Glovewort Picker's Bell Bearing [2]
            0x40002305, // Ghost-Glovewort Picker's Bell Bearing [3]
        ]);
        item_groups.insert("Spirit Ashes".to_string(), vec![
            0x40030D40, // Black Knife Tiche
            0x40031128, // Banished Knight Oleg
            0x40031510, // Banished Knight Engvall
            0x400318F8, // Fanged Imp Ashes
            0x40031CE0, // Latenna the Albinauric
            0x400320C8, // Nomad Ashes
            0x400324B0, // Nightmaiden & Swordstress Puppets
            0x40032898, // Mimic Tear Ashes
            0x40032C80, // Crystalian Ashes
            0x40033068, // Ancestral Follower Ashes
            0x40033450, // Winged Misbegotten Ashes
            0x40033838, // Albinauric Ashes
            0x40033C20, // Skeletal Militiaman Ashes
            0x40034008, // Skeletal Bandit Ashes
            0x400343F0, // Oracle Envoy Ashes
            0x400347D8, // Putrid Corpse Ashes
            0x40034BC0, // Depraved Perfumer Carmaan
            0x40034FA8, // Perfumer Tricia
            0x40035390, // Glintstone Sorcerer Ashes
            0x40035778, // Twinsage Sorcerer Ashes
            0x40035B60, // Page Ashes
            0x40035F48, // Battlemage Hugues
            0x40036330, // Clayman Ashes
            0x40036718, // Cleanrot Knight Finlay
            0x40036B00, // Kindred of Rot Ashes
            0x40036EE8, // Marionette Soldier Ashes
            0x400372D0, // Avionette Soldier Ashes
            0x400376B8, // Fire Monk Ashes
            0x40037AA0, // Blackflame Monk Amon
            0x40037E88, // Man-Serpent Ashes
            0x40038270, // Azula Beastman Ashes
            0x40038658, // Kaiden Sellsword Ashes
            0x40038A40, // Lone Wolf Ashes
            0x40038E28, // Giant Rat Ashes
            0x40039210, // Demi-Human Ashes
            0x400395F8, // Rotten Stray Ashes
            0x400399E0, // Spirit Jellyfish Ashes
            0x40039DC8, // Warhawk Ashes
            0x4003A1B0, // Stormhawk Deenh
            0x4003A598, // Bloodhound Knight Floh
            0x4003A980, // Wandering Noble Ashes
            0x4003AD68, // Noble Sorcerer Ashes
            0x4003B150, // Vulgar Militia Ashes
            0x4003B538, // Mad Pumpkin Head Ashes
            0x4003B920, // Land Squirt Ashes
            0x4003BD08, // Miranda Sprout Ashes
            0x4003C0F0, // Soldjars of Fortune Ashes
            0x4003C4D8, // Omenkiller Rollo
            0x4003C8C0, // Greatshield Soldier Ashes
            0x4003CCA8, // Archer Ashes
            0x4003D090, // Godrick Soldier Ashes
            0x4003D478, // Raya Lucaria Soldier Ashes
            0x4003D860, // Leyndell Soldier Ashes
            0x4003DC48, // Radahn Soldier Ashes
            0x4003E030, // Mausoleum Soldier Ashes
            0x4003E418, // Haligtree Soldier Ashes
            0x4003E800, // Ancient Dragon Knight Kristoff
            0x4003EBE8, // Redmane Knight Ogha
            0x4003EFD0, // Lhutel the Headless
            0x4003F3B8, // Nepheli Loux Puppet
            0x4003F7A0, // Dung Eater Puppet
            0x4003FB88, // Finger Maiden Therolina Puppet
            0x4003FF70, // Dolores the Sleeping Arrow Puppet
            0x40040358, // Jarwight Puppet
        ]);
        item_groups.insert("Sorceries".to_string(), vec![
            0x40000FA0, // Glintstone Pebble
            0x40000FA1, // Great Glintstone Shard
            0x40000FAA, // Swift Glintstone Shard
            0x40000FB4, // Glintstone Cometshard
            0x40000FB5, // Comet
            0x40000FBE, // Shard Spiral
            0x40000FC8, // Glintstone Stars
            0x40000FD2, // Star Shower
            0x40000FDC, // Crystal Barrage
            0x40000FE6, // Glintstone Arc
            0x40000FF0, // Cannon of Haima
            0x40000FFA, // Crystal Burst
            0x40001004, // Shatter Earth
            0x4000100E, // Rock Blaster
            0x40001018, // Gavel of Haima
            0x40001022, // Terra Magicus
            0x4000102C, // Starlight
            0x40001068, // Comet Azur
            0x40001072, // Founding Rain of Stars
            0x4000107C, // Stars of Ruin
            0x400010CC, // Glintblade Phalanx
            0x400010CD, // Carian Phalanx
            0x400010CE, // Greatblade Phalanx
            0x40001108, // Rennala's Full Moon
            0x40001109, // Ranni's Dark Moon
            0x40001112, // Magic Downpour
            0x4000111C, // Loretta's Greatbow
            0x4000111D, // Loretta's Mastery
            0x40001126, // Magic Glintblade
            0x40001130, // Glintstone Icecrag
            0x4000113A, // Zamor Ice Storm
            0x40001144, // Freezing Mist
            0x4000114E, // Carian Greatsword
            0x4000114F, // Adula's Moonblade
            0x40001158, // Carian Slicer
            0x40001162, // Carian Piercer
            0x4000116C, // Scholar's Armament
            0x40001176, // Scholar's Shield
            0x40001180, // Lucidity
            0x4000118A, // Frozen Armament
            0x40001194, // Shattering Crystal
            0x4000119E, // Crystal Release
            0x400011A8, // Crystal Torrent
            0x400011F8, // Ambush Shard
            0x40001202, // Night Shard
            0x4000120C, // Night Comet
            0x40001216, // Thops's Barrier
            0x40001220, // Carian Retaliation
            0x4000122A, // Eternal Darkness
            0x40001234, // Unseen Blade
            0x4000123E, // Unseen Form
            0x4000125C, // Meteorite
            0x4000125D, // Meteorite of Astel
            0x40001266, // Rock Sling
            0x40001270, // Gravity Well
            0x40001271, // Collapsing Stars
            0x400012C0, // Magma Shot
            0x400012CA, // Gelmir's Fury
            0x400012D4, // Roiling Magma
            0x400012DE, // Rykard's Rancor
            0x40001324, // Briars of Sin
            0x4000132E, // Briars of Punishment
            0x40001388, // Rancorcall
            0x40001389, // Ancient Death Rancor
            0x40001392, // Explosive Ghostflame
            0x4000139C, // Fia's Mist
            0x400013A6, // Tibia's Summons
            0x400013B0, // Death Lightning
            0x400013EC, // Oracle Bubbles
            0x400013F6, // Great Oracular Bubble
            0x40001964, // Night Maiden's Mist
        ]);
        item_groups.insert("Incantations".to_string(), vec![
            0x40001770, // Catch Flame
            0x40001771, // O, Flame!
            0x4000177A, // Flame Sling
            0x40001784, // Flame, Fall Upon Them
            0x4000178E, // Whirl, O Flame!
            0x40001798, // Flame, Cleanse Me
            0x400017A2, // Flame, Grant Me Strength
            0x400017AC, // Flame, Protect Me
            0x400017D4, // Giantsflame Take Thee
            0x400017DE, // Flame of the Fell God
            0x400017E8, // Burn, O Flame!
            0x40001842, // Black Flame
            0x4000184C, // Surge, O Flame!
            0x40001856, // Scouring Black Flame
            0x40001860, // Black Flame Ritual
            0x4000186A, // Black Flame Blade
            0x40001874, // Black Flame's Protection
            0x4000187E, // Noble Presence
            0x4000189C, // Bloodflame Talons
            0x400018A6, // Bloodboon
            0x400018B0, // Bloodflame Blade
            0x400018BA, // Barrier of Gold
            0x400018C4, // Protection of the Erdtree
            0x40001900, // Rejection
            0x4000190A, // Wrath of Gold
            0x40001914, // Urgent Heal
            0x40001915, // Heal
            0x40001916, // Great Heal
            0x40001917, // Lord's Heal
            0x40001918, // Erdtree Heal
            0x4000191E, // Blessing's Boon
            0x4000191F, // Blessing of the Erdtree
            0x40001928, // Cure Poison
            0x40001929, // Lord's Aid
            0x40001932, // Flame Fortification
            0x4000193C, // Magic Fortification
            0x40001946, // Lightning Fortification
            0x40001950, // Divine Fortification
            0x4000195A, // Lord's Divine Fortification
            0x4000196E, // Assassin's Approach
            0x40001978, // Shadow Bait
            0x40001982, // Darkness
            0x400019C8, // Golden Vow
            0x40001A2C, // Discus of Light
            0x40001A2D, // Triple Rings of Light
            0x40001A36, // Radagon's Rings of Light
            0x40001A40, // Elden Stars
            0x40001A4A, // Law of Regression
            0x40001A54, // Immutable Shield
            0x40001A5E, // Litany of Proper Death
            0x40001A68, // Law of Causality
            0x40001A72, // Order's Blade
            0x40001A7C, // Order Healing
            0x40001A90, // Bestial Sling
            0x40001A9A, // Stone of Gurranq
            0x40001AA4, // Beast Claw
            0x40001AAE, // Gurranq's Beast Claw
            0x40001AB8, // Bestial Vitality
            0x40001AC2, // Bestial Constitution
            0x40001AF4, // Lightning Spear
            0x40001AFE, // Ancient Dragons' Lightning Strike
            0x40001B08, // Lightning Strike
            0x40001B09, // Frozen Lightning Spear
            0x40001B12, // Honed Bolt
            0x40001B1C, // Ancient Dragons' Lightning Spear
            0x40001B1D, // Fortissax's Lightning Spear
            0x40001B26, // Lansseax's Glaive
            0x40001B30, // Electrify Armament
            0x40001B3A, // Vyke's Dragonbolt
            0x40001B3B, // Dragonbolt Blessing
            0x40001B58, // Dragonfire
            0x40001B59, // Agheel's Flame
            0x40001B62, // Magma Breath
            0x40001B63, // Theodorix's Magma
            0x40001B6C, // Dragonice
            0x40001B6D, // Borealis's Mist
            0x40001B76, // Rotten Breath
            0x40001B77, // Ekzykes's Decay
            0x40001B80, // Glintstone Breath
            0x40001B81, // Smarag's Glintstone Breath
            0x40001B8A, // Placidusax's Ruin
            0x40001B94, // Dragonclaw
            0x40001BA8, // Dragonmaw
            0x40001BB2, // Greyoll's Roar
            0x40001C20, // Pest Threads
            0x40001C2A, // Swarm of Flies
            0x40001C34, // Poison Mist
            0x40001C3E, // Poison Armament
            0x40001C48, // Scarlet Aeonia
            0x40001C84, // Inescapable Frenzy
            0x40001C8E, // The Flame of Frenzy
            0x40001C8F, // Unendurable Frenzy
            0x40001C98, // Frenzied Burst
            0x40001CA2, // Howl of Shabriri
            0x40001D4C, // Aspects of the Crucible 00000001 00000000 FFFFFFFF //  Tail
            0x40001D56, // Aspects of the Crucible 00000001 00000000 FFFFFFFF //  Horns
            0x40001D60, // Aspects of the Crucible 00000001 00000000 FFFFFFFF //  Breath
            0x40001D6A, // Black Blade
            0x40001EDC, // Fire's Deadly Sin
            0x40001EDF, // Golden Lightning Fortification
        ]);
        item_groups
    })
}