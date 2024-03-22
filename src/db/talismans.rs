// Modified from TGA table MassItemGib
// https://github.com/The-Grand-Archives/Elden-Ring-CT-TGA
use std::collections::BTreeMap;

use once_cell::sync::OnceCell;

pub fn talismans() -> &'static BTreeMap<String, Vec<u32>> {
    static TALISMANS: OnceCell<BTreeMap<String, Vec<u32>>> = OnceCell::new();
    TALISMANS.get_or_init(|| {
        let mut talisman_group = BTreeMap::new();
        talisman_group.insert("Talismans".to_string(),vec![
            0x200003E8, // Crimson Amber Medallion
            0x200003E9, // Crimson Amber Medallion +1
            0x200003EA, // Crimson Amber Medallion +2
            0x200003F2, // Cerulean Amber Medallion
            0x200003F3, // Cerulean Amber Medallion +1
            0x200003F4, // Cerulean Amber Medallion +2
            0x200003FC, // Viridian Amber Medallion
            0x200003FD, // Viridian Amber Medallion +1
            0x200003FE, // Viridian Amber Medallion +2
            0x20000406, // Arsenal Charm
            0x20000407, // Arsenal Charm +1
            0x20000408, // Great-Jar's Arsenal
            0x20000410, // Erdtree's Favor
            0x20000411, // Erdtree's Favor +1
            0x20000412, // Erdtree's Favor +2
            0x2000041A, // Radagon's Scarseal
            0x2000041B, // Radagon's Soreseal
            0x20000424, // Starscourge Heirloom
            0x2000042E, // Prosthesis-Wearer Heirloom
            0x20000438, // Stargazer Heirloom
            0x20000442, // Two Fingers Heirloom
            0x2000044C, // Silver Scarab
            0x20000456, // Gold Scarab
            0x20000474, // Moon of Nokstella
            0x2000047E, // Green Turtle Talisman
            0x20000488, // Stalwart Horn Charm
            0x20000489, // Stalwart Horn Charm +1
            0x20000492, // Immunizing Horn Charm
            0x20000493, // Immunizing Horn Charm +1
            0x2000049C, // Clarifying Horn Charm
            0x2000049D, // Clarifying Horn Charm +1
            0x200004A6, // Prince of Death's Pustule
            0x200004A7, // Prince of Death's Cyst
            0x200004B0, // Mottled Necklace
            0x200004B1, // Mottled Necklace +1
            0x200004BA, // Bull-Goat's Talisman
            0x200004C4, // Marika's Scarseal
            0x200004C5, // Marika's Soreseal
            0x200004CE, // Warrior Jar Shard
            0x200004CF, // Shard of Alexander
            0x200004E2, // Millicent's Prosthesis
            0x200007D0, // Magic Scorpion Charm
            0x200007DA, // Lightning Scorpion Charm
            0x200007E4, // Fire Scorpion Charm
            0x200007EE, // Sacred Scorpion Charm
            0x200007F8, // Red-Feathered Branchsword
            0x20000802, // Ritual Sword Talisman
            0x2000080C, // Spear Talisman
            0x20000816, // Hammer Talisman
            0x20000820, // Winged Sword Insignia
            0x20000821, // Rotten Winged Sword Insignia
            0x2000082A, // Dagger Talisman
            0x20000834, // Arrow's Reach Talisman
            0x2000083E, // Blue Dancer Charm
            0x20000848, // Twinblade Talisman
            0x20000852, // Axe Talisman
            0x2000085C, // Lance Talisman
            0x20000866, // Arrow's Sting Talisman
            0x20000870, // Lord of Blood's Exultation
            0x2000087A, // Kindred of Rot's Exultation
            0x20000884, // Claw Talisman
            0x2000088E, // Roar Medallion
            0x20000898, // Curved Sword Talisman
            0x200008A2, // Companion Jar
            0x200008AC, // Perfumer's Talisman
            0x20000BB8, // Graven-School Talisman
            0x20000BB9, // Graven-Mass Talisman
            0x20000BE0, // Faithful's Canvas Talisman
            0x20000BEA, // Flock's Canvas Talisman
            0x20000BF4, // Old Lord's Talisman
            0x20000BFE, // Radagon Icon
            0x20000C08, // Primal Glintstone Blade
            0x20000C12, // Godfrey Icon
            0x20000FA0, // Dragoncrest Shield Talisman
            0x20000FA1, // Dragoncrest Shield Talisman +1
            0x20000FA2, // Dragoncrest Shield Talisman +2
            0x20000FA3, // Dragoncrest Greatshield Talisman
            0x20000FAA, // Spelldrake Talisman
            0x20000FAB, // Spelldrake Talisman +1
            0x20000FAC, // Spelldrake Talisman +2
            0x20000FB4, // Flamedrake Talisman
            0x20000FB5, // Flamedrake Talisman +1
            0x20000FB6, // Flamedrake Talisman +2
            0x20000FBE, // Boltdrake Talisman
            0x20000FBF, // Boltdrake Talisman +1
            0x20000FC0, // Boltdrake Talisman +2
            0x20000FC8, // Haligdrake Talisman
            0x20000FC9, // Haligdrake Talisman +1
            0x20000FCA, // Haligdrake Talisman +2
            0x20000FD2, // Pearldrake Talisman
            0x20000FD3, // Pearldrake Talisman +1
            0x20000FD4, // Pearldrake Talisman +2
            0x20000FDC, // Crucible Scale Talisman
            0x20000FE6, // Crucible Feather Talisman
            0x20000FF0, // Blue-Feathered Branchsword
            0x20000FFA, // Ritual Shield Talisman
            0x20001004, // Greatshield Talisman
            0x2000100E, // Crucible Knot Talisman
            0x20001388, // Crimson Seed Talisman
            0x20001392, // Cerulean Seed Talisman
            0x2000139C, // Blessed Dew Talisman
            0x200013A6, // Taker's Cameo
            0x200013B0, // Godskin Swaddling Cloth
            0x200013BA, // Assassin's Crimson Dagger
            0x200013C4, // Assassin's Cerulean Dagger
            0x20001770, // Crepus's Vial
            0x2000177A, // Concealing Veil
            0x20001784, // Carian Filigreed Crest
            0x20001798, // Longtail Cat Talisman
            0x200017A2, // Shabriri's Woe
            0x200017AC, // Daedicar's Woe
            0x200017B6, // Sacrificial Twig
            0x200017C0, // Furled Finger's Trick-Mirror
            0x200017CA, // Host's Trick-Mirror
            0x200017D4, // Entwining Umbilical Cord
            0x200017DE, // Ancestral Spirit's Horn
        ]);
        talisman_group
    })
}


















































































































//
