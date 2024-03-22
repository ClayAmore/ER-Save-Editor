// Modified from TGA table MassItemGib
// https://github.com/The-Grand-Archives/Elden-Ring-CT-TGA
use std::collections::BTreeMap;

use once_cell::sync::OnceCell;

pub fn aows() -> &'static BTreeMap<String, Vec<u32>> {
    static AOWS: OnceCell<BTreeMap<String, Vec<u32>>> = OnceCell::new();
    AOWS.get_or_init(|| {
        let mut aow_group = BTreeMap::new();
        aow_group.insert("Ashes of war".to_string(),vec![
            0x80002710, // Ash of War: Lion's Claw
            0x80002774, // Ash of War: Impaling Thrust
            0x800027D8, // Ash of War: Piercing Fang
            0x8000283C, // Ash of War: Spinning Slash
            0x80002904, // Ash of War: Charge Forth
            0x80002968, // Ash of War: Stamp (Upward Cut)
            0x800029CC, // Ash of War: Stamp (Sweep)
            0x80002A30, // Ash of War: Blood Tax
            0x80002A94, // Ash of War: Repeating Thrust
            0x80002AF8, // Ash of War: Wild Strikes
            0x80002B5C, // Ash of War: Spinning Strikes
            0x80002BC0, // Ash of War: Double Slash
            0x80002C24, // Ash of War: Prelate's Charge
            0x80002C88, // Ash of War: Unsheathe
            0x80002CEC, // Ash of War: Square Off
            0x80002D50, // Ash of War: Giant Hunt
            0x80002E18, // Ash of War: Loretta's Slash
            0x80002E7C, // Ash of War: Poison Moth Flight
            0x80002EE0, // Ash of War: Spinning Weapon
            0x80002FA8, // Ash of War: Storm Assault
            0x8000300C, // Ash of War: Stormcaller
            0x80003070, // Ash of War: Sword Dance
            0x80004E20, // Ash of War: Glintblade Phalanx
            0x80004E84, // Ash of War: Sacred Blade
            0x80004EE8, // Ash of War: Ice Spear
            0x80004F4C, // Ash of War: Glintstone Pebble
            0x80004FB0, // Ash of War: Bloody Slash
            0x80005014, // Ash of War: Lifesteal Fist
            0x800050DC, // Ash of War: Eruption
            0x80005140, // Ash of War: Prayerful Strike
            0x800051A4, // Ash of War: Gravitas
            0x80005208, // Ash of War: Storm Blade
            0x800052D0, // Ash of War: Earthshaker
            0x80005334, // Ash of War: Golden Land
            0x80005398, // Ash of War: Flaming Strike
            0x80005460, // Ash of War: Thunderbolt
            0x800054C4, // Ash of War: Lightning Slash
            0x80005528, // Ash of War: Carian Grandeur
            0x8000558C, // Ash of War: Carian Greatsword
            0x800055F0, // Ash of War: Vacuum Slice
            0x80005654, // Ash of War: Black Flame Tornado
            0x800056B8, // Ash of War: Sacred Ring of Light
            0x80005780, // Ash of War: Blood Blade
            0x800057E4, // Ash of War: Phantom Slash
            0x80005848, // Ash of War: Spectral Lance
            0x800058AC, // Ash of War: Chilling Mist
            0x80005910, // Ash of War: Poisonous Mist
            0x80007530, // Ash of War: Shield Bash
            0x80007594, // Ash of War: Barricade Shield
            0x800075F8, // Ash of War: Parry
            0x80007724, // Ash of War: Carian Retaliation
            0x80007788, // Ash of War: Storm Wall
            0x800077EC, // Ash of War: Golden Parry
            0x80007850, // Ash of War: Shield Crash
            0x800078B4, // Ash of War: No Skill
            0x80007918, // Ash of War: Thops's Barrier
            0x80009C40, // Ash of War: Through and Through
            0x80009CA4, // Ash of War: Barrage
            0x80009D08, // Ash of War: Mighty Shot
            0x80009DD0, // Ash of War: Enchanted Shot
            0x80009E34, // Ash of War: Sky Shot
            0x80009E98, // Ash of War: Rain of Arrows
            0x8000C3B4, // Ash of War: Hoarfrost Stomp
            0x8000C418, // Ash of War: Storm Stomp
            0x8000C47C, // Ash of War: Kick
            0x8000C4E0, // Ash of War: Lightning Ram
            0x8000C544, // Ash of War: Flame of the Redmanes
            0x8000C5A8, // Ash of War: Ground Slam
            0x8000C60C, // Ash of War: Golden Slam
            0x8000C670, // Ash of War: Waves of Darkness
            0x8000C6D4, // Ash of War: Hoarah Loux's Earthshaker
            0x8000EA60, // Ash of War: Determination
            0x8000EAC4, // Ash of War: Royal Knight's Resolve
            0x8000EB28, // Ash of War: Assassin's Gambit
            0x8000EB8C, // Ash of War: Golden Vow
            0x8000EBF0, // Ash of War: Sacred Order
            0x8000EC54, // Ash of War: Shared Order
            0x8000ECB8, // Ash of War: Seppuku
            0x8000ED1C, // Ash of War: Cragblade
            0x8000FDE8, // Ash of War: Barbaric Roar
            0x8000FE4C, // Ash of War: War Cry
            0x8000FEB0, // Ash of War: Beast's Roar
            0x8000FF14, // Ash of War: Troll's Roar
            0x8000FF78, // Ash of War: Braggart's Roar
            0x80011170, // Ash of War: Endure
            0x800111D4, // Ash of War: Vow of the Indomitable
            0x80011238, // Ash of War: Holy Ground
            0x80013880, // Ash of War: Quickstep
            0x800138E4, // Ash of War: Bloodhound's Step
            0x80013948, // Ash of War: Raptor of the Mists
            0x80014C08, // Ash of War: White Shadow's Lure
        ]);
        aow_group
    })
}


















































































































//
