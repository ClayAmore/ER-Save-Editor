use std::io;
use binary_reader::BinaryReader;
use crate::read::read::Read;

#[derive(Clone)]
pub struct WorldAreaTime {
    unk0: i32,
    unk1: i32,
    unk2: i32
}

impl Default for WorldAreaTime {
    fn default() -> Self {
        Self { unk0: Default::default(), unk1: Default::default(), unk2: Default::default() }
    }
}

impl Read for WorldAreaTime {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut world_area_time = WorldAreaTime::default();
        world_area_time.unk0 = br.read_i32()?;
        world_area_time.unk1 = br.read_i32()?;
        world_area_time.unk2 = br.read_i32()?;
        Ok(world_area_time)
    }
}


#[derive(Clone)]
pub struct WorldAreaWeather {
    unk0: i32,
    unk1: i32,
    unk2: i32
}

impl Default for WorldAreaWeather {
    fn default() -> Self {
        Self { unk0: Default::default(), unk1: Default::default(), unk2: Default::default() }
    }
}

impl Read for WorldAreaWeather {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut world_area_weather = WorldAreaWeather::default();
        world_area_weather.unk0 = br.read_i32()?;
        world_area_weather.unk1 = br.read_i32()?;
        world_area_weather.unk2 = br.read_i32()?;
        Ok(world_area_weather)
    }
}


#[derive(Clone)]
pub struct PlayerCoords {
    player_coords: (f32, f32, f32),
    map_id: u32,
    player_coords2: (f32, f32, f32),
}

impl Default for PlayerCoords {
    fn default() -> Self {
        Self { player_coords: Default::default(), map_id: Default::default(), player_coords2: Default::default() }
    }
}

impl Read for PlayerCoords {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut player_coords = PlayerCoords::default();
        player_coords.player_coords = (br.read_f32()?, br.read_f32()?, br.read_f32()?);
        player_coords.map_id = br.read_u32()?;
        let _0x1fb5af = br.read_bytes(0x11)?;
        player_coords.player_coords2 = (br.read_f32()?, br.read_f32()?, br.read_f32()?);
        let _0x1fb5cc = br.read_bytes(0x10)?;
        Ok(player_coords)
    }
}


#[derive(Clone)]
pub struct GaItem2 {
    id: i32,
    unk: i32,
    reinforce_type: i32,
    unk1: i32,
}

impl Default for GaItem2 {
    fn default() -> Self {
        Self {
            id: Default::default(),
            unk: Default::default(),
            reinforce_type: Default::default(),
            unk1: Default::default()
        }
    }
}

impl Read for GaItem2 {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut ga_item = GaItem2::default();
        ga_item.id = br.read_i32()?;
        ga_item.unk = br.read_i32()?;
        ga_item.reinforce_type = br.read_i32()?;
        ga_item.unk1 = br.read_i32()?;
        Ok(ga_item)
    }
}

#[derive(Clone)]
pub struct EventFlags {
    pub flags: Vec<u8>
}

impl Default for EventFlags {
    fn default() -> Self {
        Self { 
            flags: vec![Default::default(); 0x1bf99f]
         }
    }
}

impl Read for EventFlags {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut event_flags = EventFlags::default();
        event_flags.flags.copy_from_slice(br.read_bytes(0x1bf99f)?);
        Ok(event_flags)
    }
}

#[derive(Clone)]
pub struct GaItemData {
    unk: i32,
    unk1: i32,
    ga_items: Vec<GaItem2>
}

impl Default for GaItemData {
    fn default() -> Self {
        Self {
            unk: Default::default(),
            unk1: Default::default(),
            ga_items: vec![GaItem2::default(); 0x1b58]
        }
    }
}

impl Read for GaItemData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut ga_item_data = GaItemData::default();
        ga_item_data.unk = br.read_i32()?;
        ga_item_data.unk1 = br.read_i32()?;
        for i in 0..0x1b58 {
            ga_item_data.ga_items[i] = GaItem2::read(br)?;
        }
        Ok(ga_item_data)
    }
}

#[derive(Clone)]
pub struct RideGameData {
    horse_coords: (f32, f32, f32),
    horse_hp: u32
}

impl Default for RideGameData {
    fn default() -> Self {
        Self { horse_coords: Default::default(), horse_hp: Default::default() }
    }
}

impl Read for RideGameData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut ride_game_data = RideGameData::default();

        ride_game_data.horse_coords = (br.read_f32()?, br.read_f32()?, br.read_f32()?);

        let _0x1ce31 = br.read_i32()?;
        let _0x1ce35 = br.read_bytes(0x10)?;
        
        ride_game_data.horse_hp = br.read_u32()?;

        let _0x1ce49 = br.read_u32()?;

        Ok(ride_game_data)
    }
}

#[derive(Clone)]
pub struct Regions {
    unlocked_regions_count: u32,
    unlocked_regions: Vec<u32>
}

impl Default for Regions {
    fn default() -> Self {
        Self { unlocked_regions_count: Default::default(), unlocked_regions: Default::default() }
    }
}

impl Read for Regions {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut regions = Regions::default();

        regions.unlocked_regions_count = br.read_u32()?;

        for _i in 0..regions.unlocked_regions_count as usize {
            regions.unlocked_regions.push(br.read_u32()?);
        }

        Ok(regions)
    }
}

#[derive(Clone)]
pub struct EquipPhysicsData {
    slot1: i16,
    slot2: i16
}

impl Default for EquipPhysicsData {
    fn default() -> Self {
        Self { slot1: Default::default(), slot2: Default::default() }
    }
}

impl Read for EquipPhysicsData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_physics_data = EquipPhysicsData::default();

        // Slot 1
        equip_physics_data.slot1 = br.read_i16()?;
        let _0x165a8 = br.read_i16()?;

        // Slot 2
        equip_physics_data.slot2 = br.read_i16()?;
        let _0x165ac = br.read_i16()?;

        Ok(equip_physics_data)
    }
}

#[derive(Clone)]
pub struct EquipProjectile {
    projectile_id: i32,
    unk: i32,
}

impl Default for EquipProjectile {
    fn default() -> Self {
        Self { 
            projectile_id: Default::default(), 
            unk: Default::default(),
        }
    }
}

impl Read for EquipProjectile {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_projectile = EquipProjectile::default();
        equip_projectile.projectile_id = br.read_i32()?;
        equip_projectile.unk = br.read_i32()?;
        Ok(equip_projectile)
    }
}

#[derive(Clone)]
pub struct EquipProjectileData {
    projectile_count: i32,
    projectiles: Vec<EquipProjectile>
}

impl Default for EquipProjectileData {
    fn default() -> Self {
        Self { 
            projectile_count: Default::default(),
            projectiles: vec![],
        }
    }
}

impl Read for EquipProjectileData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_projectile_data = EquipProjectileData::default();
        
        // Distinct Projectile Count
        equip_projectile_data.projectile_count = br.read_i32()?;

        // Quick slot items
        for _i in 0..equip_projectile_data.projectile_count {
            equip_projectile_data.projectiles.push(EquipProjectile::read(br)?);
        }

        Ok(equip_projectile_data)
    }
}


#[derive(Clone)]
pub struct EquipItem {
    item_id: i16,
    unk: i16,
    unk2: i32,
}

impl Default for EquipItem {
    fn default() -> Self {
        Self { 
            item_id: Default::default(), 
            unk: Default::default(),
            unk2: Default::default()
        }
    }
}

impl Read for EquipItem {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_item = EquipItem::default();
        equip_item.item_id = br.read_i16()?;
        equip_item.unk = br.read_i16()?;
        equip_item.unk2 = br.read_i32()?;
        Ok(equip_item)
    }
}

#[derive(Clone)]
pub struct EquipItemData {
    quick_slot_items: Vec<EquipItem>,
    active_slot: i32,
    pouch_items: Vec<EquipItem>
}

impl Default for EquipItemData {
    fn default() -> Self {
        Self { 
            quick_slot_items: vec![EquipItem::default(); 0xa], 
            active_slot: Default::default(),
            pouch_items: vec![EquipItem::default(); 0x6],
        }
    }
}

impl Read for EquipItemData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_item_data = EquipItemData::default();

        // Quick slot items
        for i in 0..0xa {
            equip_item_data.quick_slot_items[i] = EquipItem::read(br)?;
        }

        // Current active quickslot index
        equip_item_data.active_slot = br.read_i32()?;

        // Pouch items
        for i in 0..0x6 {
            equip_item_data.pouch_items[i] = EquipItem::read(br)?;
        }

        let _0x15c7e = br.read_bytes(0x8)?;

        Ok(equip_item_data)
    }
}

#[derive(Clone)]
pub struct EquipMagicSpell {
    spell_id: i32,
    unk: i32,
}

impl Default for EquipMagicSpell {
    fn default() -> Self {
        Self { 
            spell_id: Default::default(), 
            unk: Default::default() 
        }
    }
}

impl Read for EquipMagicSpell {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_magic_spell = EquipMagicSpell::default();
        equip_magic_spell.spell_id = br.read_i32()?;
        equip_magic_spell.unk = br.read_i32()?;
        Ok(equip_magic_spell)
    }
}

#[derive(Clone)]
pub struct EquipMagicData {
    equip_magic_spells: Vec<EquipMagicSpell>,
    active_slot: i32
}

impl Default for EquipMagicData {
    fn default() -> Self {
        Self { 
            equip_magic_spells: vec![EquipMagicSpell::default(); 0xc], 
            active_slot: Default::default() 
        }
    }
}

impl Read for EquipMagicData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_magic_data = EquipMagicData::default();

        for i in 0..0xC {
            equip_magic_data.equip_magic_spells[i] = EquipMagicSpell::read(br)?;
        }

        let _0x15be6 = br.read_bytes(0x10)?;

        equip_magic_data.active_slot = br.read_i32()?;

        Ok(equip_magic_data)
    }
}


#[derive(Clone)]
pub struct EquipInventoryItem {
    pub ga_item_handle: i32,
    pub quantity: i32,
    pub inventory_index: i32
}

impl Default for EquipInventoryItem {
    fn default() -> Self {
        Self { 
            ga_item_handle: Default::default(),
            quantity: Default::default(),
            inventory_index: Default::default() 
        }
    }
}

impl Read for EquipInventoryItem{
    fn read(br: &mut BinaryReader) -> Result<EquipInventoryItem, io::Error> {
        let mut equip_inventory_item = EquipInventoryItem::default();
        
        equip_inventory_item.ga_item_handle = br.read_i32()?;
        equip_inventory_item.quantity = br.read_i32()?;
        equip_inventory_item.inventory_index = br.read_i32()?;

        Ok(equip_inventory_item)
    }
}


#[derive(Clone)]
pub struct EquipInventoryData {
    pub inventory_distinct_item_count: i32,
    pub inventory_items: Vec<EquipInventoryItem>,
    pub inventory_items2: Vec<EquipInventoryItem>
}

impl Default for EquipInventoryData {
    fn default() -> Self {
        Self { 
            inventory_distinct_item_count: Default::default(),
            inventory_items: vec![],
            inventory_items2: vec![]
        }
    }
}

impl EquipInventoryData {
    fn read(br: &mut BinaryReader, length1: usize, length2: usize) -> Result<EquipInventoryData, io::Error> {
        let mut equip_inventory_data = EquipInventoryData::default();

        equip_inventory_data.inventory_distinct_item_count = br.read_i32()?;
        
        for _i in 0..length1 {
            equip_inventory_data.inventory_items.push(EquipInventoryItem::read(br)?);
        }
        
        let _0x1497a = br.read_i32()?;

        for _i in 0..length2 {
            equip_inventory_data.inventory_items2.push(EquipInventoryItem::read(br)?);
        }

        let _0x15b7e = br.read_i32()?;
        let _0x15b82 = br.read_i32()?;

        Ok(equip_inventory_data)
    }
}

#[derive(Clone)]
pub struct ChrAsm {
    arm_style: i32,
    left_hand_active_slot: i32,
    right_hand_active_slot: i32,
    left_arrow_active_slot: i32,
    right_arrow_active_slot: i32,
    left_bolt_active_slot: i32,
    right_bolt_active_slot: i32,
    left_hand_armament1: i32,
    right_hand_armament1: i32,
    left_hand_armament2: i32,
    right_hand_armament2: i32,
    left_hand_armament3: i32,
    right_hand_armament3: i32,
    arrows1: i32,
    bolts1: i32,
    arrows2: i32,
    bolts2: i32,
    head: i32,
    chest: i32,
    arms: i32,
    legs: i32,
    talisman1: i32,
    talisman2: i32,
    talisman3: i32,
    talisman4: i32,
    talisman5: i32
}

impl Default for ChrAsm {
    fn default() -> Self {
        Self { 
            arm_style: Default::default(),
            left_hand_active_slot: Default::default(),
            right_hand_active_slot: Default::default(),
            left_arrow_active_slot: Default::default(),
            right_arrow_active_slot: Default::default(),
            left_bolt_active_slot: Default::default(),
            right_bolt_active_slot: Default::default(),
            left_hand_armament1: Default::default(),
            right_hand_armament1: Default::default(),
            left_hand_armament2: Default::default(),
            right_hand_armament2: Default::default(),
            left_hand_armament3: Default::default(),
            right_hand_armament3: Default::default(),
            arrows1: Default::default(),
            bolts1: Default::default(),
            arrows2: Default::default(),
            bolts2: Default::default(),
            head: Default::default(),
            chest: Default::default(),
            arms: Default::default(),
            legs: Default::default(),
            talisman1: Default::default(),
            talisman2: Default::default(),
            talisman3: Default::default(),
            talisman4: Default::default(),
            talisman5: Default::default(),
        }
    }
}

impl Read for ChrAsm {
    fn read(br: &mut BinaryReader) -> Result<ChrAsm, io::Error> {
        let mut chr_asm = ChrAsm::default();

        chr_asm.arm_style = br.read_i32()?;
        chr_asm.left_hand_active_slot = br.read_i32()?;
        chr_asm.right_hand_active_slot = br.read_i32()?;
        chr_asm.left_arrow_active_slot = br.read_i32()?;
        chr_asm.right_arrow_active_slot = br.read_i32()?;
        chr_asm.left_bolt_active_slot = br.read_i32()?;
        chr_asm.right_bolt_active_slot = br.read_i32()?;

        chr_asm.left_hand_armament1 = br.read_i32()?;
        chr_asm.right_hand_armament1 = br.read_i32()?;
        chr_asm.left_hand_armament2 = br.read_i32()?;
        chr_asm.right_hand_armament2 = br.read_i32()?;
        chr_asm.left_hand_armament3 = br.read_i32()?;
        chr_asm.right_hand_armament3 = br.read_i32()?;
        chr_asm.arrows1 = br.read_i32()?;
        chr_asm.bolts1 = br.read_i32()?;
        chr_asm.arrows2 = br.read_i32()?;
        chr_asm.bolts2 = br.read_i32()?;

        let _0xca7a = br.read_i32()?;
        let _0xca7e = br.read_i32()?;

        chr_asm.head = br.read_i32()?;
        chr_asm.chest = br.read_i32()?;
        chr_asm.arms = br.read_i32()?;
        chr_asm.legs = br.read_i32()?;

        let _0xca92 = br.read_i32()?;

        chr_asm.talisman1 = br.read_i32()?;
        chr_asm.talisman2 = br.read_i32()?;
        chr_asm.talisman3 = br.read_i32()?;
        chr_asm.talisman4 = br.read_i32()?;
        chr_asm.talisman5 = br.read_i32()?;

        Ok(chr_asm)
    }
}

#[derive(Clone)]
pub struct EquipData {
    left_hand_armament1: i32,
    right_hand_armament1: i32,
    left_hand_armament2: i32,
    right_hand_armament2: i32,
    left_hand_armament3: i32,
    right_hand_armament3: i32,
    arrows1: i32,
    bolts1: i32,
    arrows2: i32,
    bolts2: i32,
    head: i32,
    chest: i32,
    arms: i32,
    legs: i32,
    talisman1: i32,
    talisman2: i32,
    talisman3: i32,
    talisman4: i32,
    talisman5: i32,
}

impl Default for EquipData {
    fn default() -> Self {
        Self { 
            left_hand_armament1: -1,
            right_hand_armament1: -1,
            left_hand_armament2: -1,
            right_hand_armament2: -1,
            left_hand_armament3: -1,
            right_hand_armament3: -1,
            arrows1: -1,
            bolts1: -1,
            arrows2: -1,
            bolts2: -1,
            head: -1,
            chest: -1,
            arms: -1,
            legs: -1,
            talisman1: -1,
            talisman2: -1,
            talisman3: -1,
            talisman4: -1,
            talisman5: -1,
        }
    }
}

impl Read for EquipData {
    fn read(br: &mut BinaryReader) -> Result<EquipData, io::Error> {
        let mut equip_data = EquipData::default();
        equip_data.left_hand_armament1 = br.read_i32()?;
        equip_data.right_hand_armament1 = br.read_i32()?;
        equip_data.left_hand_armament2 = br.read_i32()?;
        equip_data.right_hand_armament2 = br.read_i32()?;
        equip_data.left_hand_armament3 = br.read_i32()?;
        equip_data.right_hand_armament3 = br.read_i32()?;
        equip_data.arrows1 = br.read_i32()?;
        equip_data.bolts1 = br.read_i32()?;
        equip_data.arrows2 = br.read_i32()?;
        equip_data.bolts2 = br.read_i32()?;

        let _0xca7a = br.read_i32()?;
        let _0xca7e = br.read_i32()?;

        equip_data.head = br.read_i32()?;
        equip_data.chest = br.read_i32()?;
        equip_data.arms = br.read_i32()?;
        equip_data.legs = br.read_i32()?;

        let _0xca92 = br.read_i32()?;

        equip_data.talisman1 = br.read_i32()?;
        equip_data.talisman2 = br.read_i32()?;
        equip_data.talisman3 = br.read_i32()?;
        equip_data.talisman4 = br.read_i32()?;
        equip_data.talisman5 = br.read_i32()?;
        
        Ok(equip_data)
    }
}

#[derive(Clone)]
pub struct PlayerGameData {
    pub health: u32,
    pub max_health: u32,
    pub base_max_health: u32,
    pub fp: u32,
    pub max_fp: u32,
    pub base_max_fp: u32,
    pub sp: u32,
    pub max_sp: u32,
    pub base_max_sp: u32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub level: u32,
    pub souls: u32,
    pub soulsmemory: u32,
    pub character_name: [u8; 0x20],
    pub password: [u8; 0x12],
    pub group_password1: [u8; 0x12],
    pub group_password2: [u8; 0x12],
    pub group_password3: [u8; 0x12],
    pub group_password4: [u8; 0x12],
    pub group_password5: [u8; 0x12],
}

impl Default for PlayerGameData {
    fn default() -> Self {
        Self { 
            health: Default::default(),
            max_health: Default::default(),
            base_max_health: Default::default(),
            fp: Default::default(),
            max_fp: Default::default(),
            base_max_fp: Default::default(),
            sp: Default::default(),
            max_sp: Default::default(),
            base_max_sp: Default::default(),
            vigor: Default::default(),
            mind: Default::default(),
            endurance: Default::default(),
            strength: Default::default(),
            dexterity: Default::default(),
            intelligence: Default::default(),
            faith: Default::default(),
            arcane: Default::default(),
            level: Default::default(),
            souls: Default::default(),
            soulsmemory: Default::default(),
            character_name: Default::default(),
            password: Default::default(),
            group_password1: Default::default(),
            group_password2: Default::default(),
            group_password3: Default::default(),
            group_password4: Default::default(),
            group_password5: Default::default() 
        }
    }
}

impl Read for PlayerGameData {
    fn read(br: &mut BinaryReader) -> Result<PlayerGameData, io::Error> {
        let mut player_game_data = PlayerGameData::default();
        
        let _0xb960 = br.read_i32()?; 
        let _0xb964 = br.read_i32()?;

        // Health
        player_game_data.health = br.read_u32()?;
        player_game_data.max_health = br.read_u32()?;
        player_game_data.base_max_health = br.read_u32()?;

        // FP
        player_game_data.fp = br.read_u32()?;
        player_game_data.max_fp = br.read_u32()?;
        player_game_data.base_max_fp = br.read_u32()?;

        let _0xb980 = br.read_i32()?;

        // SP
        player_game_data.sp = br.read_u32()?;
        player_game_data.max_sp = br.read_u32()?;
        player_game_data.base_max_sp = br.read_u32()?;

        let _0xb990 = br.read_i32()?;

        // Stats
        player_game_data.vigor = br.read_u32()?;
        player_game_data.mind = br.read_u32()?;
        player_game_data.endurance = br.read_u32()?;
        player_game_data.strength = br.read_u32()?;
        player_game_data.dexterity = br.read_u32()?;
        player_game_data.intelligence = br.read_u32()?;
        player_game_data.faith = br.read_u32()?;
        player_game_data.arcane = br.read_u32()?;

        let _0xc826 = br.read_i32()?;
        let _0xc82a = br.read_i32()?;
        let _0xc82e = br.read_i32()?;

        // Level
        player_game_data.level = br.read_u32()?;
        
        // Souls
        player_game_data.souls = br.read_u32()?;
        player_game_data.soulsmemory = br.read_u32()?;

        let _0xc832 = br.read_bytes(0x28)?;

        // Character Name
        let character_name = br.read_bytes(0x20)?;
        player_game_data.character_name.copy_from_slice(character_name);

        let _0xc87a = br.read_bytes(0x5c);

        // Passwords
        let password = br.read_bytes(0x12)?;
        player_game_data.password.copy_from_slice(password);
        
        let group_password1 = br.read_bytes(0x12)?;
        player_game_data.group_password1.copy_from_slice(group_password1);
        
        let group_password2 = br.read_bytes(0x12)?;
        player_game_data.group_password2.copy_from_slice(group_password2);
        
        let group_password3 = br.read_bytes(0x12)?;
        player_game_data.group_password3.copy_from_slice(group_password3);
        
        let group_password4 = br.read_bytes(0x12)?;
        player_game_data.group_password4.copy_from_slice(group_password4);
        
        let group_password5 = br.read_bytes(0x12)?;
        player_game_data.group_password5.copy_from_slice(group_password5);

        Ok(player_game_data)
    }
}

#[derive(Copy, Clone)]
pub struct GaItem {
    pub gaitem_handle: i32,
    pub item_id: u32,
    pub unk2: i32,
    pub unk3: i32,
    pub aow_gaitem_handle: i32,
    pub unk5: u8
}

impl Default for GaItem {
    fn default() -> Self {
        Self { 
            gaitem_handle: -1, 
            item_id: 0, 
            unk2: -1, 
            unk3: -1, 
            aow_gaitem_handle: -1, 
            unk5: 0
        }
    }
}

impl Read for GaItem {
    fn read(br: &mut BinaryReader) -> Result<GaItem, io::Error> {
        let mut ga_item = GaItem::default();
        ga_item.gaitem_handle = br.read_i32()?;
        ga_item.item_id = br.read_u32()?;

        // Weapon
        if ga_item.item_id != 0 && (ga_item.item_id & 0xf0000000) == 0 {
            ga_item.unk2 = br.read_i32()?;
            ga_item.unk3 = br.read_i32()?;
            ga_item.aow_gaitem_handle = br.read_i32()?;
            ga_item.unk5 = br.read_bytes(1)?[0];
        }
        // Armor
        else if ga_item.item_id != 0 && (ga_item.item_id & 0xf0000000) == 0x10000000 {
            ga_item.unk2 = br.read_i32()?;
            ga_item.unk3 = br.read_i32()?;
        }

        Ok(ga_item)
    }
}

#[derive(Clone)]
pub struct SaveSlot {
    pub checksum: [u8; 0x10],
    pub map_id: u32,
    pub ga_items: Vec<GaItem>,
    pub player_game_data: PlayerGameData,
    pub equip_data: EquipData,
    pub chr_asm: ChrAsm,
    pub equip_inventory_data: EquipInventoryData,
    pub equip_magic_data: EquipMagicData,
    pub equip_item_data: EquipItemData,
    pub equip_gesture_data: [i32;0x6],
    pub equip_projectile_data: EquipProjectileData,
    pub equip_physics_data: EquipPhysicsData,
    pub storage_inventory_data: EquipInventoryData,
    pub gesture_game_data: Vec<i32>,
    pub regions: Regions,
    pub ride_game_data: RideGameData,
    pub ga_item_data: GaItemData,
    pub event_flags: EventFlags,
    pub player_coords: PlayerCoords,
    pub world_area_weather: WorldAreaWeather,
    pub world_area_time: WorldAreaTime,
    pub steam_id: u64
}

impl Default for SaveSlot {
    fn default() -> Self {
        Self {
            checksum: [0x0; 0x10],
            map_id: 0,
            ga_items: vec![GaItem::default(); 0x1400],
            player_game_data: PlayerGameData::default(),
            equip_data: EquipData::default(),
            chr_asm: ChrAsm::default(),
            equip_inventory_data: EquipInventoryData::default(),
            equip_magic_data: EquipMagicData::default(),
            equip_item_data: EquipItemData::default(),
            equip_gesture_data: [Default::default(); 0x6],
            equip_projectile_data: EquipProjectileData::default(),
            equip_physics_data: EquipPhysicsData::default(),
            storage_inventory_data: EquipInventoryData::default(),
            gesture_game_data: vec![0; 0x40],
            regions: Regions::default(),
            ride_game_data: RideGameData::default(),
            event_flags: EventFlags::default(),
            ga_item_data: GaItemData::default(),
            player_coords: PlayerCoords::default(),
            world_area_weather: WorldAreaWeather::default(),
            world_area_time: WorldAreaTime::default(),
            steam_id: Default::default(),
        }
    }
}

impl SaveSlot {
    pub fn read(br: &mut BinaryReader, index: usize) -> Result<SaveSlot, io::Error> {
        let mut save_slot = SaveSlot::default();

        // Jump to character slot
        br.jmp(0x300 + 0x280010*index);

        // Checksum
        save_slot.checksum.copy_from_slice(br.read_bytes(0x10)?);

        // Unknown
        let _0x10 = br.read_i32();

        // MapId
        save_slot.map_id = br.read_u32()?;

        // Uknown
        let _0x18 = br.read_bytes(0x18)?;

        // GaItem
        for i in 0..0x1400 {
            save_slot.ga_items[i] = GaItem::read(br)?;
        }

        // Player Game Data (Health, Fp, Stats, etc...)
        save_slot.player_game_data = PlayerGameData::read(br)?;
        
        let _0xc942 = br.read_bytes(0x34)?;
        let _0xc976 = br.read_bytes(0xd0)?;
        
        // Equip Data
        save_slot.equip_data = EquipData::read(br)?;
        
        // Chr Asm
        save_slot.chr_asm = ChrAsm::read(br)?;

        // Something to do with Chr Asm
        let _0xcb1e = br.read_bytes(0x58)?;

        // Equip Inventory Data
        save_slot.equip_inventory_data = EquipInventoryData::read(br, 0xa80, 0x180)?;

        // Equip Magic Spell 
        save_slot.equip_magic_data = EquipMagicData::read(br)?;

        // Equip Item
        save_slot.equip_item_data = EquipItemData::read(br)?;

        // Equipped Gestures
        for i in 0..0x6 {
            save_slot.equip_gesture_data[i] = br.read_i32()?;
        }

        // Equipped Projectiles 
        save_slot.equip_projectile_data = EquipProjectileData::read(br)?;

        // Same equip data again. Skipping it 
        let _0xcb1e = br.read_bytes(0x9c)?;
        
        // Equipped physics
        save_slot.equip_physics_data = EquipPhysicsData::read(br)?;
        
        let _0x165a6 = br.read_u32()?;
        // Face data (skipping it)
        let _0x165ae = br.read_bytes(0x12f)?;
        
        // Equip Inventory Data 2
        save_slot.storage_inventory_data = EquipInventoryData::read(br, 0x780, 0x80)?;

        // Equipped Gestures
        for i in 0..0x40 {
            save_slot.gesture_game_data[i] = br.read_i32()?;
        }

        // Regions
        save_slot.regions = Regions::read(br)?;

        // Horse Data 
        save_slot.ride_game_data = RideGameData::read(br)?;        

        let _0x1ce4d = br.read_bytes(1)?;
        let _0x1ce4e = br.read_bytes(0x40)?;
        let _0x1ce8e= br.read_i32()?;
        let _0x1ce92= br.read_i32()?;
        let _0x1ce96= br.read_i32()?;
        
        // Menu Profile Save Load (Skipping)
        let _menu_profile_save_load = br.read_bytes(0x1008)?;

        // Trophy Equip Data (Skipping)
        let _trophy_equip_data = br.read_bytes(0x34)?;

        // GaItems 
        save_slot.ga_item_data = GaItemData::read(br)?;

        // Tutorial Data (Skipping)
        let _tutorial_data = br.read_bytes(0x408)?;

        // Unknown values (Grouping and skipping)
        let _0x3945e = br.read_bytes(0x1d)?;

        // Event Flags
        save_slot.event_flags = EventFlags::read(br)?;

        let _0x1f9222 = br.read_bytes(1)?;

        
        for _i in 0..0x5 {
            let length = br.read_i32()? as usize;
            let _0x1f9223 = br.read_bytes(length)?;
        }
        
        save_slot.player_coords = PlayerCoords::read(br)?;
        
        let _game_man_unkown_values = br.read_bytes(0xf)?;

        let _to_be_changed_to_assert = br.read_i32()?;
        // Value should always be 2 for active accounts. 0 for empty ones.
        // assert!(br.read_i32()? == 2 || br.read_i32()? == 0);

        let _cs_net_data_chunks = br.read_bytes(0x20000)?;
        
        save_slot.world_area_weather = WorldAreaWeather::read(br)?;
        save_slot.world_area_time = WorldAreaTime::read(br)?;

        let _0x21b607 = br.read_bytes(0x10)?;

        save_slot.steam_id = br.read_u64()?;

        // CSPS5Activity (Skipping)
        let _cs_ps5_activity = br.read_bytes(0x20)?;

        // DLC?
        let _cs_dlc = br.read_bytes(0x32)?;

        let _0x21b671 = br.read_bytes(0x80)?;

        Ok(save_slot)
    }
}