pub mod accessory_name {
    use once_cell::sync::Lazy;
    use std::{collections::HashMap, sync::Mutex};

    pub static ACCESSORY_NAME: Lazy<Mutex<HashMap<u32, &str>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (1000, "Crimson Amber Medallion"),
            (1001, "Crimson Amber Medallion +1"),
            (1002, "Crimson Amber Medallion +2"),
            (1010, "Cerulean Amber Medallion"),
            (1011, "Cerulean Amber Medallion +1"),
            (1012, "Cerulean Amber Medallion +2"),
            (1020, "Viridian Amber Medallion"),
            (1021, "Viridian Amber Medallion +1"),
            (1022, "Viridian Amber Medallion +2"),
            (1030, "Arsenal Charm"),
            (1031, "Arsenal Charm +1"),
            (1032, "Great-Jar's Arsenal"),
            (1040, "Erdtree's Favor"),
            (1041, "Erdtree's Favor +1"),
            (1042, "Erdtree's Favor +2"),
            (1050, "Radagon's Scarseal"),
            (1051, "Radagon's Soreseal"),
            (1060, "Starscourge Heirloom"),
            (1070, "Prosthesis-Wearer Heirloom"),
            (1080, "Stargazer Heirloom"),
            (1090, "Two Fingers Heirloom"),
            (1100, "Silver Scarab"),
            (1110, "Gold Scarab"),
            (1140, "Moon of Nokstella"),
            (1150, "Green Turtle Talisman"),
            (1160, "Stalwart Horn Charm"),
            (1161, "Stalwart Horn Charm +1"),
            (1170, "Immunizing Horn Charm"),
            (1171, "Immunizing Horn Charm +1"),
            (1180, "Clarifying Horn Charm"),
            (1181, "Clarifying Horn Charm +1"),
            (1190, "Prince of Death's Pustule"),
            (1191, "Prince of Death's Cyst"),
            (1200, "Mottled Necklace"),
            (1201, "Mottled Necklace +1"),
            (1210, "Bull-Goat's Talisman"),
            (1220, "Marika's Scarseal"),
            (1221, "Marika's Soreseal"),
            (1230, "Warrior Jar Shard"),
            (1231, "Shard of Alexander"),
            (1250, "Millicent's Prosthesis"),
            (2000, "Magic Scorpion Charm"),
            (2010, "Lightning Scorpion Charm"),
            (2020, "Fire Scorpion Charm"),
            (2030, "Sacred Scorpion Charm"),
            (2040, "Red-Feathered Branchsword"),
            (2050, "Ritual Sword Talisman"),
            (2060, "Spear Talisman"),
            (2070, "Hammer Talisman"),
            (2080, "Winged Sword Insignia"),
            (2081, "Rotten Winged Sword Insignia"),
            (2090, "Dagger Talisman"),
            (2100, "Arrow's Reach Talisman"),
            (2110, "Blue Dancer Charm"),
            (2120, "Twinblade Talisman"),
            (2130, "Axe Talisman"),
            (2140, "Lance Talisman"),
            (2150, "Arrow's Sting Talisman"),
            (2160, "Lord of Blood's Exultation"),
            (2170, "Kindred of Rot's Exultation"),
            (2180, "Claw Talisman"),
            (2190, "Roar Medallion"),
            (2200, "Curved Sword Talisman"),
            (2210, "Companion Jar"),
            (2220, "Perfumer's Talisman"),
            (3000, "Graven-School Talisman"),
            (3001, "Graven-Mass Talisman"),
            (3040, "Faithful's Canvas Talisman"),
            (3050, "Flock's Canvas Talisman"),
            (3060, "Old Lord's Talisman"),
            (3070, "Radagon Icon"),
            (3080, "Primal Glintstone Blade"),
            (3090, "Godfrey Icon"),
            (4000, "Dragoncrest Shield Talisman"),
            (4001, "Dragoncrest Shield Talisman +1"),
            (4002, "Dragoncrest Shield Talisman +2"),
            (4003, "Dragoncrest Greatshield Talisman"),
            (4010, "Spelldrake Talisman"),
            (4011, "Spelldrake Talisman +1"),
            (4012, "Spelldrake Talisman +2"),
            (4020, "Flamedrake Talisman"),
            (4021, "Flamedrake Talisman +1"),
            (4022, "Flamedrake Talisman +2"),
            (4030, "Boltdrake Talisman"),
            (4031, "Boltdrake Talisman +1"),
            (4032, "Boltdrake Talisman +2"),
            (4040, "Haligdrake Talisman"),
            (4041, "Haligdrake Talisman +1"),
            (4042, "Haligdrake Talisman +2"),
            (4050, "Pearldrake Talisman"),
            (4051, "Pearldrake Talisman +1"),
            (4052, "Pearldrake Talisman +2"),
            (4060, "Crucible Scale Talisman"),
            (4070, "Crucible Feather Talisman"),
            (4080, "Blue-Feathered Branchsword"),
            (4090, "Ritual Shield Talisman"),
            (4100, "Greatshield Talisman"),
            (4110, "Crucible Knot Talisman"),
            (5000, "Crimson Seed Talisman"),
            (5010, "Cerulean Seed Talisman"),
            (5020, "Blessed Dew Talisman"),
            (5030, "Taker's Cameo"),
            (5040, "Godskin Swaddling Cloth"),
            (5050, "Assassin's Crimson Dagger"),
            (5060, "Assassin's Cerulean Dagger"),
            (6000, "Crepus's Vial"),
            (6010, "Concealing Veil"),
            (6020, "Carian Filigreed Crest"),
            (6040, "Longtail Cat Talisman"),
            (6050, "Shabriri's Woe"),
            (6060, "Daedicar's Woe"),
            (6070, "Sacrificial Twig"),
            (6080, "Furled Finger's Trick-Mirror"),
            (6090, "Host's Trick-Mirror"),
            (6100, "Entwining Umbilical Cord"),
            (6110, "Ancestral Spirit's Horn"),
            (999999999, ""),
        ]))
    });
}
