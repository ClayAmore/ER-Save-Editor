// Modified from TGA table MassItemGib
// https://github.com/The-Grand-Archives/Elden-Ring-CT-TGA
use std::collections::BTreeMap;

use once_cell::sync::OnceCell;

pub fn weapons() -> &'static BTreeMap<String, Vec<u32>> {
    static WEAPONS: OnceCell<BTreeMap<String, Vec<u32>>> = OnceCell::new();
    WEAPONS.get_or_init(|| {
        let mut weapon_groups = BTreeMap::new();
        weapon_groups.insert("Daggers".to_string(),vec![
            0x000F4240, // Dagger
            0x000F6950, // Black Knife
            0x000F9060, // Parrying Dagger
            0x000FB770, // Miséricorde
            0x000FDE80, // Reduvia
            0x00100590, // Crystal Knife
            0x00102CA0, // Celebrant's Sickle
            0x001053B0, // Glintstone Kris
            0x00107AC0, // Scorpion's Stinger
            0x0010A1D0, // Great Knife
            0x0010C8E0, // Wakizashi
            0x0010EFF0, // Cinquedea
            0x00113E10, // Ivory Sickle
            0x00116520, // Bloodstained Dagger
            0x00118C30, // Erdsteel Dagger
            0x0011B340, // Blade of Calling
        ]);
        weapon_groups.insert("Straight Swords".to_string(),vec![
            0x001E8480, // Longsword
            0x001EAB90, // Short Sword
            0x001ED2A0, // Broadsword
            0x001F20C0, // Lordsworn's Straight Sword
            0x001F47D0, // Weathered Straight Sword
            0x001F6EE0, // Ornamental Straight Sword
            0x001F95F0, // Golden Epitaph
            0x001FBD00, // Nox Flowing Sword
            0x001FE410, // Inseparable Sword
            0x00203230, // Coded Sword
            0x0020A760, // Sword of Night and Flame
            0x0020CE70, // Crystal Sword
            0x002143A0, // Carian Knight's Sword
            0x00216AB0, // Sword of St. Trina
            0x002191C0, // Miquellan Knight's Sword
            0x0021B8D0, // Cane Sword
            0x0021DFE0, // Regalia of Eochaid
            0x002206F0, // Noble's Slender Sword
            0x00222E00, // Warhawk's Talon
            0x00225510, // Lazuli Glintstone Sword
            0x00227C20, // Rotten Crystal Sword
        ]);
        weapon_groups.insert("Greatswords".to_string(),vec![
            0x002DC6C0, // Bastard Sword
            0x002DEDD0, // Forked Greatsword
            0x002E14E0, // Iron Greatsword
            0x002E3BF0, // Lordsworn's Greatsword
            0x002E6300, // Knight's Greatsword
            0x002E8A10, // Flamberge
            0x002EB120, // Ordovis's Greatsword
            0x002ED830, // Alabaster Lord's Sword
            0x002EFF40, // Banished Knight's Greatsword
            0x002F2650, // Dark Moon Greatsword
            0x002F4D60, // Sacred Relic Sword
            0x002FC290, // Helphen's Steeple
            0x002FE9A0, // Blasphemous Blade
            0x003010B0, // Marais Executioner's Sword
            0x003037C0, // Sword of Milos
            0x00305ED0, // Golden Order Greatsword
            0x003085E0, // Claymore
            0x0030ACF0, // Gargoyle's Greatsword
            0x0030D400, // Death's Poker
            0x0030FB10, // Gargoyle's Blackblade
        ]);
        weapon_groups.insert("Colossal Swords".to_string(),vec![
            0x003D0900, // Greatsword
            0x003D3010, // Watchdog's Greatsword
            0x003D5720, // Maliketh's Black Blade
            0x003D7E30, // Troll's Golden Sword
            0x003DA540, // Zweihander
            0x003DCC50, // Starscourge Greatsword
            0x003DF360, // Royal Greatsword
            0x003E1A70, // Godslayer's Greatsword
            0x003E4180, // Ruins Greatsword
            0x003E8FA0, // Grafted Blade Greatsword
            0x003EB6B0, // Troll Knight's Sword
        ]);
        weapon_groups.insert("Thrusting Swords".to_string(),vec![
            0x004C4B40, // Estoc
            0x004C7250, // Cleanrot Knight's Sword
            0x004C9960, // Rapier
            0x004CC070, // Rogier's Rapier
            0x004CE780, // Antspur Rapier
            0x004D0E90, // Frozen Needle
            0x004D35A0, // Noble's Estoc
        ]);
        weapon_groups.insert("Heavy Thrusting Swords".to_string(),vec![
            0x005B8D80, // Bloody Helice
            0x005BB490, // Godskin Stitcher
            0x005BDBA0, // Great Épée
            0x005C29C0, // Dragon King's Cragblade
        ]);
        weapon_groups.insert("Curved Swords".to_string(),vec![
            0x006ACFC0, // Falchion
            0x006AF6D0, // Beastman's Curved Sword
            0x006B1DE0, // Shotel
            0x006B44F0, // Shamshir
            0x006B6C00, // Bandit's Curved Sword
            0x006B9310, // Magma Blade
            0x006BBA20, // Flowing Curved Sword
            0x006BE130, // Wing of Astel
            0x006C0840, // Scavenger's Curved Sword
            0x006C5660, // Eclipse Shotel
            0x006C7D70, // Serpent-God's Curved Sword
            0x006CA480, // Mantis Blade
            0x006CF2A0, // Scimitar
            0x006D19B0, // Grossmesser
        ]);
        weapon_groups.insert("Curved Greatswords".to_string(),vec![
            0x007A3910, // Onyx Lord's Greatsword
            0x007A6020, // Dismounter
            0x007A8730, // Bloodhound's Fang
            0x007AAE40, // Magma Wyrm's Scalesword
            0x007AD550, // Zamor Curved Sword
            0x007AFC60, // Omen Cleaver
            0x007B2370, // Monk's Flameblade
            0x007B4A80, // Beastman's Cleaver
            0x007B98A0, // Morgott's Cursed Sword
        ]);
        weapon_groups.insert("Katana".to_string(),vec![
            0x00895440, // Uchigatana
            0x00897B50, // Nagakiba
            0x0089A260, // Hand of Malenia
            0x0089C970, // Meteoric Ore Blade
            0x0089F080, // Rivers of Blood
            0x008A3EA0, // Moonveil
            0x008A65B0, // Dragonscale Blade
            0x008A8CC0, // Serpentbone Blade
        ]);
        weapon_groups.insert("Twinblades".to_string(),vec![
            0x00989680, // Twinblade
            0x0098BD90, // Godskin Peeler
            0x00990BB0, // Twinned Knight Swords
            0x009959D0, // Eleonora's Poleblade
            0x0099CF00, // Gargoyle's Twinblade
            0x0099F610, // Gargoyle's Black Blades
        ]);
        weapon_groups.insert("Hammers".to_string(),vec![
            0x00A7D8C0, // Mace
            0x00A7FFD0, // Club
            0x00A84DF0, // Curved Club
            0x00A87500, // Warpick
            0x00A89C10, // Morning Star
            0x00A8C320, // Varré's Bouquet
            0x00A8EA30, // Spiked Club
            0x00A91140, // Hammer
            0x00A93850, // Monk's Flamemace
            0x00A95F60, // Envoy's Horn
            0x00A98670, // Scepter of the All-Knowing
            0x00A9AD80, // Nox Flowing Hammer
            0x00A9D490, // Ringed Finger
            0x00A9FBA0, // Stone Club
            0x00AA22B0, // Marika's Hammer
        ]);
        weapon_groups.insert("Great Hammers".to_string(),vec![
            0x00B71B00, // Large Club
            0x00B74210, // Greathorn Hammer
            0x00B76920, // Battle Hammer
            0x00B80560, // Great Mace
            0x00B85380, // Curved Great Club
            0x00B916D0, // Celebrant's Skull
        ]);
        weapon_groups.insert("Warhammers".to_string(),vec![
            0x00B93DE0, // Pickaxe
            0x00B964F0, // Beastclaw Greathammer
            0x00B98C00, // Envoy's Long Horn
            0x00B9B310, // Cranial Vessel Candlestand
            0x00B9DA20, // Great Stars
            0x00BA0130, // Brick Hammer
            0x00BA2840, // Devourer's Scepter
            0x00BA4F50, // Rotten Battle Hammer
        ]);
        weapon_groups.insert("Flails".to_string(),vec![
            0x00C65D40, // Nightrider Flail
            0x00C68450, // Flail
            0x00C6AB60, // Family Heads
            0x00C6D270, // Bastard's Stars
            0x00C6F980, // Chainlink Flail
        ]);
        weapon_groups.insert("Axes".to_string(),vec![
            0x00D59F80, // Battle Axe
            0x00D5C690, // Forked Hatchet
            0x00D5EDA0, // Hand Axe
            0x00D614B0, // Jawbone Axe
            0x00D63BC0, // Iron Cleaver
            0x00D662D0, // Ripple Blade
            0x00D689E0, // Celebrant's Cleaver
            0x00D6D800, // Icerind Hatchet
            0x00D72620, // Highland Axe
            0x00D74D30, // Sacrificial Axe
            0x00D77440, // Rosus' Axe
            0x00D7C260, // Stormhawk Axe
        ]);
        weapon_groups.insert("Greataxes".to_string(),vec![
            0x00E4E1C0, // Greataxe
            0x00E508D0, // Warped Axe
            0x00E52FE0, // Great Omenkiller Cleaver
            0x00E556F0, // Crescent Moon Axe
            0x00E57E00, // Axe of Godrick
            0x00E5A510, // Longhaft Axe
            0x00E5CC20, // Rusted Anchor
            0x00E61A40, // Executioner's Greataxe
            0x00E68F70, // Winged Greathorn
            0x00E6B680, // Butchering Knife
            0x00E6DD90, // Gargoyle's Great Axe
            0x00E704A0, // Gargoyle's Black Axe
        ]);
        weapon_groups.insert("Spears".to_string(),vec![
            0x00F42400, // Short Spear
            0x00F44B10, // Spear
            0x00F47220, // Crystal Spear
            0x00F49930, // Clayman's Harpoon
            0x00F4C040, // Cleanrot Spear
            0x00F4E750, // Partisan
            0x00F50E60, // Celebrant's Rib-Rake
            0x00F53570, // Pike
            0x00F55C80, // Torchpole
            0x00F58390, // Bolt of Gransax
            0x00F5D1B0, // Cross-Naginata
            0x00F5F8C0, // Death Ritual Spear
            0x00F61FD0, // Inquisitor's Girandole
            0x00F646E0, // Spiked Spear
            0x00F66DF0, // Iron Spear
            0x00F69500, // Rotten Crystal Spear
        ]);
        weapon_groups.insert("Great Spears".to_string(),vec![
            0x01038D50, // Mohgwyn's Sacred Spear
            0x0103B460, // Siluria's Tree
            0x0103DB70, // Serpent-Hunter
            0x01042990, // Vyke's War Spear
            0x010450A0, // Lance
            0x010477B0, // Treespear
        ]);
        weapon_groups.insert("Halberds".to_string(),vec![
            0x0112A880, // Halberd
            0x0112CF90, // Pest's Glaive
            0x0112F6A0, // Lucerne
            0x01131DB0, // Banished Knight's Halberd
            0x011344C0, // Commander's Standard
            0x01136BD0, // Nightrider Glaive
            0x011392E0, // Ripple Crescent Halberd
            0x0113B9F0, // Vulgar Militia Saw
            0x0113E100, // Golden Halberd
            0x01140810, // Glaive
            0x01142F20, // Loretta's War Sickle
            0x01145630, // Guardian's Swordspear
            0x0114A450, // Vulgar Militia Shotel
            0x0114CB60, // Dragon Halberd
            0x0114F270, // Gargoyle's Halberd
            0x01151980, // Gargoyle's Black Halberd
        ]);
        weapon_groups.insert("Reapers".to_string(),vec![
            0x0121EAC0, // Scythe
            0x012211D0, // Grave Scythe
            0x012238E0, // Halo Scythe
            0x0122D520, // Winged Scythe
        ]);
        weapon_groups.insert("Whips".to_string(),vec![
            0x01312D00, // Whip
            0x01317B20, // Thorned Whip
            0x0131A230, // Magma Whip Candlestick
            0x0131F050, // Hoslow's Petal Whip
            0x01321760, // Giant's Red Braid
            0x01323E70, // Urumi
        ]);
        weapon_groups.insert("Fists".to_string(),vec![
            0x01406F40, // Caestus
            0x01409650, // Spiked Caestus
            0x014159A0, // Grafted Dragon
            0x014180B0, // Iron Ball
            0x0141A7C0, // Star Fist
            0x0141F5E0, // Katar
            0x01421CF0, // Clinging Bone
            0x01424400, // Veteran's Prosthesis
            0x01426B10, // Cipher Pata
        ]);
        weapon_groups.insert("Claws".to_string(),vec![
            0x014FB180, // Hookclaws
            0x014FD890, // Venomous Fang
            0x014FFFA0, // Bloodhound Claws
            0x015026B0, // Raptor Talons
        ]);
        weapon_groups.insert("Colossal Weapons".to_string(),vec![
            0x015EF3C0, // Prelate's Inferno Crozier
            0x015F1AD0, // Watchdog's Staff
            0x015F41E0, // Great Club
            0x015F68F0, // Envoy's Greathorn
            0x015F9000, // Duelist Greataxe
            0x015FB710, // Axe of Godfrey
            0x015FDE20, // Dragon Greatclaw
            0x01600530, // Staff of the Avatar
            0x01602C40, // Fallingstar Beast Jaw
            0x01607A60, // Ghiza's Wheel
            0x0160A170, // Giant-Crusher
            0x0160C880, // Golem's Halberd
            0x0160EF90, // Troll's Hammer
            0x016116A0, // Rotten Staff
            0x01613DB0, // Rotten Greataxe
        ]); 
        weapon_groups.insert("Torches".to_string(),vec![
            0x016E3600, // Torch
            0x016E8420, // Steel-Wire Torch
            0x016ED240, // St. Trina's Torch
            0x016EF950, // Ghostflame Torch
            0x016F2060, // Beast-Repellent Torch
            0x016F4770, // Sentry's Torch
        ]);
        weapon_groups.insert("Shields".to_string(),vec![
            0x01C9C380, // Buckler
            0x01C9EA90, // Perfumer's Shield
            0x01CA11A0, // Man-Serpent's Shield
            0x01CA38B0, // Rickety Shield
            0x01CA5FC0, // Pillory Shield
            0x01CAADE0, // Beastman's Jar-Shield
            0x01CAD4F0, // Red Thorn Roundshield
            0x01CAFC00, // Scripture Wooden Shield
            0x01CB2310, // Riveted Wooden Shield
            0x01CB4A20, // Blue-White Wooden Shield
            0x01CB7130, // Rift Shield
            0x01CB9840, // Iron Roundshield
            0x01CBBF50, // Gilded Iron Shield
            0x01CBE660, // Ice Crest Shield
            0x01CC0D70, // Smoldering Shield
            0x01CCA9B0, // Spiralhorn Shield
            0x01CCD0C0, // Coil Shield
            0x01D905C0, // Kite Shield
            0x01D92CD0, // Marred Leather Shield
            0x01D953E0, // Marred Wooden Shield
            0x01D97AF0, // Banished Knight's Shield
            0x01D9A200, // Albinauric Shield
            0x01D9C910, // Sun Realm Shield
            0x01D9F020, // Silver Mirrorshield
            0x01DA1730, // Round Shield
            0x01DA3E40, // Scorpion Kite Shield
            0x01DA6550, // Twinbird Kite Shield
            0x01DA8C60, // Blue-Gold Kite Shield
            0x01DB0190, // Brass Shield
            0x01DB28A0, // Great Turtle Shell
            0x01DB9DD0, // Shield of the Guilty
            0x01DBEBF0, // Carian Knight's Shield
            0x01DC8830, // Large Leather Shield
            0x01DCAF40, // Horse Crest Wooden Shield
            0x01DCD650, // Candletree Wooden Shield
            0x01DCFD60, // Flame Crest Wooden Shield
            0x01DD2470, // Hawk Crest Wooden Shield
            0x01DD4B80, // Beast Crest Heater Shield
            0x01DD7290, // Red Crest Heater Shield
            0x01DD99A0, // Blue Crest Heater Shield
            0x01DDC0B0, // Eclipse Crest Heater Shield
            0x01DDE7C0, // Inverted Hawk Heater Shield
            0x01DE0ED0, // Heater Shield
            0x01DE35E0, // Black Leather Shield
        ]);
        weapon_groups.insert("Greatshields".to_string(),vec![
            0x01E84800, // Dragon Towershield
            0x01E89620, // Distinguished Greatshield
            0x01E8BD30, // Crucible Hornshield
            0x01E8E440, // Dragonclaw Shield
            0x01E90B50, // Briar Greatshield
            0x01E98080, // Erdtree Greatshield
            0x01E9A790, // Golden Beast Crest Shield
            0x01EA1CC0, // Jellyfish Shield
            0x01EA43D0, // Fingerprint Stone Shield
            0x01EA6AE0, // Icon Shield
            0x01EA91F0, // One-Eyed Shield
            0x01EAB900, // Visage Shield
            0x01EAE010, // Spiked Palisade Shield
            0x01EB2E30, // Manor Towershield
            0x01EB5540, // Crossed-Tree Towershield
            0x01EB7C50, // Inverted Hawk Towershield
            0x01EBA360, // Ant's Skull Plate
            0x01EBCA70, // Redmane Greatshield
            0x01EBF180, // Eclipse Crest Greatshield
            0x01EC1890, // Cuckoo Greatshield
            0x01EC3FA0, // Golden Greatshield
            0x01EC66B0, // Gilded Greatshield
            0x01EC8DC0, // Haligtree Crest Greatshield
            0x01ECB4D0, // Wooden Greatshield
            0x01ECDBE0, // Lordsworn's Shield
        ]);
        weapon_groups.insert("Staves".to_string(),vec![
            0x01F78A40, // Glintstone Staff
            0x01F82680, // Crystal Staff
            0x01F84D90, // Gelmir Glintstone Staff
            0x01F874A0, // Demi-Human Queen's Staff
            0x01F8E9D0, // Carian Regal Scepter
            0x01F95F00, // Digger's Staff
            0x01F98610, // Astrologer's Staff
            0x01FA2250, // Carian Glintblade Staff
            0x01FA4960, // Prince of Death's Staff
            0x01FA7070, // Albinauric Staff
            0x01FA9780, // Academy Glintstone Staff
            0x01FABE90, // Carian Glintstone Staff
            0x01FB0CB0, // Azur's Glintstone Staff
            0x01FB33C0, // Lusat's Glintstone Staff
            0x01FB5AD0, // Meteorite Staff
            0x01FB81E0, // Staff of the Guilty
            0x01FBA8F0, // Rotten Crystal Staff
            0x01FBD000, // Staff of Loss
        ]);
        weapon_groups.insert("Seals".to_string(),vec![
            0x0206CC80, // Finger Seal
            0x0206F390, // Godslayer's Seal
            0x02071AA0, // Giant's Seal
            0x020741B0, // Gravel Stone Seal
            0x020768C0, // Clawmark Seal
            0x0207B6E0, // Golden Order Seal
            0x0207DDF0, // Erdtree Seal
            0x02080500, // Dragon Communion Seal
            0x02082C10, // Frenzied Flame Seal
        ]);
        weapon_groups.insert("Light Bows".to_string(),vec![
            0x02625A00, // Shortbow
            0x02628110, // Misbegotten Shortbow
            0x0262A820, // Red Branch Shortbow
            0x0262CF30, // Harp Bow
            0x02631D50, // Composite Bow
        ]); 
        weapon_groups.insert("Bows".to_string(),vec![
            0x02719C40, // Longbow
            0x0271C350, // Albinauric Bow
            0x0271EA60, // Horn Bow
            0x02721170, // Erdtree Bow
            0x02723880, // Serpent Bow
            0x027286A0, // Pulley Bow
            0x0272ADB0, // Black Bow
        ]);
        weapon_groups.insert("Greatbows".to_string(),vec![
            0x0280DE80, // Lion Greatbow
            0x02810590, // Golem Greatbow
            0x028153B0, // Erdtree Greatbow
            0x02817AC0, // Greatbow
        ]);
        weapon_groups.insert("Crossbows".to_string(),vec![
            0x029020C0, // Soldier's Crossbow
            0x02906EE0, // Light Crossbow
            0x029095F0, // Heavy Crossbow
            0x0290E410, // Pulley Crossbow
            0x02910B20, // Full Moon Crossbow
            0x02915940, // Arbalest
            0x0291CE70, // Crepus's Black-Key Crossbow
        ]);
        weapon_groups.insert("Ballistae".to_string(),vec![
            0x029F6300, // Hand Ballista
            0x029F8A10, // Jar Cannon
        ]);
        weapon_groups.insert("Arrows".to_string(), vec![
            0x02FAF080, // Arrow
            0x02FB1790, // Fire Arrow
            0x02FB3EA0, // Serpent Arrow
            0x02FB65B0, // Bone Arrow (Fletched)
            0x02FB8CC0, // St. Trina's Arrow
            0x02FBDAE0, // Shattershard Arrow (Fletched)
            0x02FC2900, // Rainbow Stone Arrow (Fletched)
            0x02FC5010, // Golden Arrow
            0x02FC7720, // Dwelling Arrow
            0x02FC9E30, // Bone Arrow
            0x02FCEC50, // Firebone Arrow (Fletched)
            0x02FD1360, // Firebone Arrow
            0x02FD3A70, // Poisonbone Arrow (Fletched)
            0x02FD6180, // Poisonbone Arrow
            0x02FD8890, // Sleepbone Arrow (Fletched)
            0x02FDAFA0, // Sleepbone Arrow
            0x02FDD6B0, // Stormwing Bone Arrow
            0x02FDFDC0, // Lightningbone Arrow (Fletched)
            0x02FE24D0, // Lightningbone Arrow
            0x02FE4BE0, // Rainbow Stone Arrow
            0x02FE72F0, // Shattershard Arrow
            0x02FE9A00, // Spiritflame Arrow
            0x02FEE820, // Magicbone Arrow (Fletched)
            0x02FF0F30, // Magicbone Arrow
            0x02FF3640, // Haligbone Arrow (Fletched)
            0x02FF5D50, // Haligbone Arrow
            0x02FF8460, // Bloodbone Arrow (Fletched)
            0x02FFAB70, // Bloodbone Arrow
            0x02FFD280, // Coldbone Arrow (Fletched)
            0x02FFF990, // Coldbone Arrow
            0x030020A0, // Rotbone Arrow (Fletched)
            0x030047B0, // Rotbone Arrow
        ]);
        weapon_groups.insert("Greatarrows".to_string(), vec![
            0x030A32C0, // Great Arrow
            0x030A59D0, // Golem's Great Arrow
            0x030A80E0, // Golden Great Arrow
            0x030AA7F0, // Golem's Magic Arrow
            0x030ACF00, // Radahn's Spear
            0x030AF610, // Bone Great Arrow (Fletched)
            0x030B1D20, // Bone Great Arrow
        ]);
        weapon_groups.insert("Bolts".to_string(), vec![
            0x03197500, // Bolt
            0x03199C10, // Lightning Bolt
            0x0319C320, // Perfumer's Bolt
            0x0319EA30, // Black-Key Bolt
            0x031A1140, // Burred Bolt
            0x031A3850, // Meteor Bolt
            0x031A5F60, // Explosive Bolt
            0x031A8670, // Golden Bolt
            0x031AAD80, // Lordsworn's Bolt
            0x031AD490, // Bone Bolt
            0x031AFBA0, // Firebone Bolt
            0x031B22B0, // Lightningbone Bolt
            0x031B49C0, // Magicbone Bolt
            0x031B70D0, // Haligbone Bolt
            0x031B97E0, // Poisonbone Bolt
            0x031BBEF0, // Bloodbone Bolt
            0x031BE600, // Coldbone Bolt
            0x031C0D10, // Rotbone Bolt
            0x031C3420, // Sleepbone Bolt
            0x031C5B30, // Flaming Bolt
        ]);
        weapon_groups.insert("Greatbolts".to_string(), vec![
            0x0328B740, // Ballista Bolt
            0x0328DE50, // Lightning Greatbolt
            0x03290560, // Explosive Greatbolt
            0x03292C70, // Bone Ballista Bolt
        ]);
        weapon_groups
    })
}