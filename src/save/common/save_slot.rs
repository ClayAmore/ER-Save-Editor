use std::io;
use binary_reader::BinaryReader;
use crate::{read::read::Read, write::write::Write};

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

impl Write for WorldAreaTime {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.unk0.to_le_bytes());
        bytes.extend(self.unk1.to_le_bytes());
        bytes.extend(self.unk2.to_le_bytes());
        Ok(bytes)
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

impl Write for WorldAreaWeather {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.unk0.to_le_bytes());
        bytes.extend(self.unk1.to_le_bytes());
        bytes.extend(self.unk2.to_le_bytes());
        Ok(bytes)
    }
}


#[derive(Clone)]
pub struct PlayerCoords {
    pub player_coords: (f32, f32, f32),
    pub map_id: [u8; 4],
    _0x11: [u8; 0x11],
    pub player_coords2: (f32, f32, f32),
    _0x10: [u8; 0x10],
}

impl Default for PlayerCoords {
    fn default() -> Self {
        Self { 
            player_coords: Default::default(), 
            map_id: Default::default(), 
            _0x11: Default::default(), 
            player_coords2: Default::default(), 
            _0x10: Default::default() 
        }
    }
}

impl Read for PlayerCoords {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut player_coords = PlayerCoords::default();
        player_coords.player_coords = (br.read_f32()?, br.read_f32()?, br.read_f32()?);
        player_coords.map_id.copy_from_slice(br.read_bytes(4)?);
        let _0x11 = br.read_bytes(0x11)?;
        player_coords.player_coords2 = (br.read_f32()?, br.read_f32()?, br.read_f32()?);
        let _0x10: &[u8] = br.read_bytes(0x10)?;
        Ok(player_coords)
    }
}

impl Write for PlayerCoords {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.player_coords.0.to_le_bytes());
        bytes.extend(self.player_coords.1.to_le_bytes());
        bytes.extend(self.player_coords.2.to_le_bytes());
        bytes.extend(self.map_id);
        bytes.extend(self._0x11);
        bytes.extend(self.player_coords2.0.to_le_bytes());
        bytes.extend(self.player_coords2.1.to_le_bytes());
        bytes.extend(self.player_coords2.2.to_le_bytes());
        bytes.extend(self._0x10);
        Ok(bytes)
    }
}

#[derive(Clone)]
struct UknownList {
    length: i32,
    elements: Vec<u8>
}

impl Default for UknownList {
    fn default() -> Self {
        Self { length: Default::default(), elements: Default::default() }
    }
}

impl Read for UknownList {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut unk_list = UknownList::default();
        unk_list.length = br.read_i32()?;
        for byte in br.read_bytes(unk_list.length as usize)?.iter() {
            unk_list.elements.push(*byte);
        }
        Ok(unk_list)
    }
}

impl Write for UknownList {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.length.to_le_bytes());
        bytes.extend(self.elements.to_vec());
        Ok(bytes)
    }
}


#[derive(Clone)]
pub struct GaItem2 {
    pub id: u32,
    pub unk: u32,
    pub reinforce_type: u32,
    pub unk1: u32,
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
        ga_item.id = br.read_u32()?;
        ga_item.unk = br.read_u32()?;
        ga_item.reinforce_type = br.read_u32()?;
        ga_item.unk1 = br.read_u32()?;
        Ok(ga_item)
    }
}

impl Write for GaItem2 {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.id.to_le_bytes());
        bytes.extend(self.unk.to_le_bytes());
        bytes.extend(self.reinforce_type.to_le_bytes());
        bytes.extend(self.unk1.to_le_bytes());
        Ok(bytes)
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

impl Write for EventFlags {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.flags.to_vec());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct GaItemData {
    pub distinct_aquired_items_count: i32,
    pub unk1: i32,
    pub ga_items: Vec<GaItem2>
}

impl Default for GaItemData {
    fn default() -> Self {
        Self {
            distinct_aquired_items_count: Default::default(),
            unk1: Default::default(),
            ga_items: vec![GaItem2::default(); 0x1b58]
        }
    }
}

impl Read for GaItemData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut ga_item_data = GaItemData::default();
        ga_item_data.distinct_aquired_items_count = br.read_i32()?;
        ga_item_data.unk1 = br.read_i32()?;
        for i in 0..0x1b58 {
            ga_item_data.ga_items[i] = GaItem2::read(br)?;
        }
        Ok(ga_item_data)
    }
}

impl Write for GaItemData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.distinct_aquired_items_count.to_le_bytes());
        bytes.extend(self.unk1.to_le_bytes());

        for i in 0..0x1b58 {
            bytes.extend(self.ga_items[i].write()?);
        }
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct RideGameData {
    horse_coords: (f32, f32, f32),
    _0x4: i32,
    _0x10: [u8; 0x10],
    horse_hp: u32,
    _0x4_1: u32,
}

impl Default for RideGameData {
    fn default() -> Self {
        Self { 
            horse_coords: Default::default(), 
            _0x4: Default::default(), 
            _0x10: Default::default(), 
            horse_hp: Default::default(), 
            _0x4_1: Default::default() 
        }
    }
}

impl Read for RideGameData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut ride_game_data = RideGameData::default();

        ride_game_data.horse_coords = (br.read_f32()?, br.read_f32()?, br.read_f32()?);

        ride_game_data._0x4 = br.read_i32()?;
        ride_game_data._0x10.copy_from_slice(br.read_bytes(0x10)?);
        
        ride_game_data.horse_hp = br.read_u32()?;

        ride_game_data._0x4_1 = br.read_u32()?;

        Ok(ride_game_data)
    }
}

impl Write for RideGameData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.horse_coords.0.to_le_bytes());
        bytes.extend(self.horse_coords.1.to_le_bytes());
        bytes.extend(self.horse_coords.2.to_le_bytes());
        bytes.extend(self._0x4.to_le_bytes());
        bytes.extend(self._0x10);
        bytes.extend(self.horse_hp.to_le_bytes());
        bytes.extend(self._0x4_1.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct Regions {
    pub unlocked_regions_count: u32,
    pub unlocked_regions: Vec<u32>
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

impl Write for Regions {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.unlocked_regions_count.to_le_bytes());

        for i in 0..self.unlocked_regions_count as usize {
            bytes.extend(self.unlocked_regions[i].to_le_bytes());
        }

        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct EquipPhysicsData {
    pub slot1: u32,
    pub slot2: u32,
}

impl Default for EquipPhysicsData {
    fn default() -> Self {
        Self { 
            slot1: Default::default(), 
            slot2: Default::default(), 
        }
    }
}

impl Read for EquipPhysicsData {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_physics_data = EquipPhysicsData::default();

        // Slot 1
        equip_physics_data.slot1 = br.read_u32()?;

        // Slot 2
        equip_physics_data.slot2 = br.read_u32()?;

        Ok(equip_physics_data)
    }
}

impl Write for EquipPhysicsData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.slot1.to_le_bytes());
        bytes.extend(self.slot2.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct EquipProjectile {
    pub projectile_id: u32,
    pub unk: i32,
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
        equip_projectile.projectile_id = br.read_u32()?;
        equip_projectile.unk = br.read_i32()?;
        Ok(equip_projectile)
    }
}

impl Write for EquipProjectile {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.projectile_id.to_le_bytes());
        bytes.extend(self.unk.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct EquipProjectileData {
    pub projectile_count: i32,
    pub projectiles: Vec<EquipProjectile>
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
impl Write for EquipProjectileData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // Distinct Projectile Count
        bytes.extend(self.projectile_count.to_le_bytes());

        // Quick slot items
        for i in 0..self.projectile_count as usize {
            bytes.extend(self.projectiles[i].write()?);
        }

        Ok(bytes)
    }
}

#[derive(Clone, Default)]
pub struct EquippedItems {
    pub left_hand_armaments: [u32; 3],
    pub right_hand_armaments: [u32; 3],
    pub arrows: [u32; 2],
    pub bolts: [u32; 2],
    _unk1: u32,
    _unk2: u32,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    _unk3: u32,
    pub talismans: [u32; 4],
    _unk4: u32, //Most likely covenant.
    pub quickitems: [u32; 0xA],
    pub pouch: [u32; 6],
    _padding17: u32,
}
impl Read for EquippedItems {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equipped_items = EquippedItems::default();
        equipped_items.left_hand_armaments[0] = br.read_u32()?;
        equipped_items.right_hand_armaments[0] = br.read_u32()?;
        equipped_items.left_hand_armaments[1] = br.read_u32()?;
        equipped_items.right_hand_armaments[1] = br.read_u32()?;
        equipped_items.left_hand_armaments[2] = br.read_u32()?;
        equipped_items.right_hand_armaments[2] = br.read_u32()?;
        equipped_items.arrows[0] = br.read_u32()?;
        equipped_items.bolts[0] = br.read_u32()?;
        equipped_items.arrows[1] = br.read_u32()?;
        equipped_items.bolts[1] = br.read_u32()?;
        equipped_items._unk1 = br.read_u32()?;
        equipped_items._unk2 = br.read_u32()?;
        equipped_items.head = br.read_u32()?;
        equipped_items.chest = br.read_u32()?;
        equipped_items.arms = br.read_u32()?;
        equipped_items.legs = br.read_u32()?;
        equipped_items._unk3 = br.read_u32()?;
        for i in 0..4 { equipped_items.talismans[i] = br.read_u32()?; }
        equipped_items._unk4 = br.read_u32()?;
        for i in 0..0xA { equipped_items.quickitems[i] = br.read_u32()?; }
        for i in 0..0x6 { equipped_items.pouch[i] = br.read_u32()?; }
        equipped_items._padding17 = br.read_u32()?;
        Ok(equipped_items)
    }
}
impl Write for EquippedItems {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.left_hand_armaments[0].to_le_bytes());
        bytes.extend(self.right_hand_armaments[0].to_le_bytes());
        bytes.extend(self.left_hand_armaments[1].to_le_bytes());
        bytes.extend(self.right_hand_armaments[1].to_le_bytes());
        bytes.extend(self.left_hand_armaments[2].to_le_bytes());
        bytes.extend(self.right_hand_armaments[2].to_le_bytes());
        bytes.extend(self.arrows[0].to_le_bytes());
        bytes.extend(self.bolts[0].to_le_bytes());
        bytes.extend(self.arrows[1].to_le_bytes());
        bytes.extend(self.bolts[1].to_le_bytes());
        bytes.extend(self._unk1.to_le_bytes());
        bytes.extend(self._unk2.to_le_bytes());
        bytes.extend(self.head.to_le_bytes());
        bytes.extend(self.chest.to_le_bytes());
        bytes.extend(self.arms.to_le_bytes());
        bytes.extend(self.legs.to_le_bytes());
        bytes.extend(self._unk3.to_le_bytes());
        for i in 0..4 { bytes.extend(self.talismans[i].to_le_bytes()); }
        bytes.extend(self._unk4.to_le_bytes());
        for i in 0..0xA { bytes.extend(self.quickitems[i].to_le_bytes()); }
        for i in 0..0x6 { bytes.extend(self.pouch[i].to_le_bytes()); }
        bytes.extend(self._padding17.to_le_bytes());
        Ok(bytes)
    }
}


#[derive(Copy, Clone)]
pub struct EquipItem {
    pub item_id: u32,
    pub equipment_index: u32,
}

impl Default for EquipItem {
    fn default() -> Self {
        Self { 
            item_id: Default::default(), 
            equipment_index: Default::default()
        }
    }
}

impl Read for EquipItem {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
        let mut equip_item = EquipItem::default();
        equip_item.item_id = br.read_u32()?;
        equip_item.equipment_index = br.read_u32()?;
        Ok(equip_item)
    }
}

impl Write for EquipItem {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.item_id.to_le_bytes());
        bytes.extend(self.equipment_index.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct EquipItemData {
    pub quick_slot_items: Vec<EquipItem>,
    pub active_slot: i32,
    pub pouch_items: Vec<EquipItem>,
    _0x8: [u8; 0x8],
}

impl Default for EquipItemData {
    fn default() -> Self {
        Self { 
            quick_slot_items: vec![EquipItem::default(); 0xa], 
            active_slot: Default::default(),
            pouch_items: vec![EquipItem::default(); 0x6],
            _0x8: [0; 0x8]
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

        equip_item_data._0x8.copy_from_slice(br.read_bytes(0x8)?);

        Ok(equip_item_data)
    }
}

impl Write for EquipItemData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // Quick slot items
        for i in 0..0xa {
            bytes.extend(self.quick_slot_items[i].write()?);
        }

        bytes.extend(self.active_slot.to_le_bytes());

        // Pouch items
        for i in 0..0x6 {
            bytes.extend(self.pouch_items[i].write()?);
        }

        bytes.extend(self._0x8);

        Ok(bytes)
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

impl Write for EquipMagicSpell {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.spell_id.to_le_bytes());
        bytes.extend(self.unk.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct EquipMagicData {
    equip_magic_spells: Vec<EquipMagicSpell>,
    _0x10: [u8; 0x10],
    active_slot: i32,
}

impl Default for EquipMagicData {
    fn default() -> Self {
        Self { 
            equip_magic_spells: vec![EquipMagicSpell::default(); 0xc], 
            _0x10: [0x0; 0x10],
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

        equip_magic_data._0x10.copy_from_slice(br.read_bytes(0x10)?);

        equip_magic_data.active_slot = br.read_i32()?;

        Ok(equip_magic_data)
    }
}

impl Write for EquipMagicData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..0xC {
            bytes.extend(self.equip_magic_spells[i].write()?);
        }
        bytes.extend(self._0x10);
        bytes.extend(self.active_slot.to_le_bytes());
        Ok(bytes)
    }
}


#[derive(Copy, Clone)]
pub struct EquipInventoryItem {
    pub ga_item_handle: u32,
    pub quantity: u32,
    pub inventory_index: u32
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
        
        equip_inventory_item.ga_item_handle = br.read_u32()?;
        equip_inventory_item.quantity = br.read_u32()?;
        equip_inventory_item.inventory_index = br.read_u32()?;

        Ok(equip_inventory_item)
    }
}

impl Write for EquipInventoryItem {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.ga_item_handle.to_le_bytes());
        bytes.extend(self.quantity.to_le_bytes());
        bytes.extend(self.inventory_index.to_le_bytes());

        Ok(bytes)
    }
}


#[derive(Clone)]
pub struct EquipInventoryData {
    pub common_inventory_items_distinct_count: u32,
    pub common_items: Vec<EquipInventoryItem>,
    pub key_inventory_items_distinct_count: u32,
    pub key_items: Vec<EquipInventoryItem>,
    pub next_equip_index: u32,
    pub next_acquisition_sort_id: u32,
}

impl Default for EquipInventoryData {
    fn default() -> Self {
        Self { 
            common_inventory_items_distinct_count: Default::default(),
            common_items: vec![],
            key_inventory_items_distinct_count: Default::default(),
            key_items: vec![],
            next_equip_index: 0,
            next_acquisition_sort_id: 0,
        }
    }
}

impl EquipInventoryData {
    fn read(br: &mut BinaryReader, length1: usize, length2: usize) -> Result<EquipInventoryData, io::Error> {
        let mut equip_inventory_data = EquipInventoryData::default();

        equip_inventory_data.common_inventory_items_distinct_count = br.read_u32()?;
        
        for _i in 0..length1 {
            equip_inventory_data.common_items.push(EquipInventoryItem::read(br)?);
        }
        
        equip_inventory_data.key_inventory_items_distinct_count = br.read_u32()?;

        for _i in 0..length2 {
            equip_inventory_data.key_items.push(EquipInventoryItem::read(br)?);
        }

        equip_inventory_data.next_equip_index = br.read_u32()?;
        equip_inventory_data.next_acquisition_sort_id = br.read_u32()?;

        Ok(equip_inventory_data)
    }

    fn write(&self, length1: usize, length2: usize) -> Result<Vec<u8>, io::Error>{
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.common_inventory_items_distinct_count.to_le_bytes());

        for i in 0..length1 {
            bytes.extend(self.common_items[i].write()?);
        }

        bytes.extend(self.key_inventory_items_distinct_count.to_le_bytes());

        for i in 0..length2 {
            bytes.extend(self.key_items[i].write()?);
        }

        bytes.extend(self.next_equip_index.to_le_bytes());
        bytes.extend(self.next_acquisition_sort_id.to_le_bytes());

        Ok(bytes)
    }
}

#[derive(Clone, Default)]
pub struct ChrAsm {
    pub arm_style: u32,
    pub left_hand_active_slot: u32,
    pub right_hand_active_slot: u32,
    pub left_arrow_active_slot: u32,
    pub right_arrow_active_slot: u32,
    pub left_bolt_active_slot: u32,
    pub right_bolt_active_slot: u32,
    pub left_hand_armaments: [u32; 3],
    pub right_hand_armaments: [u32; 3],
    pub arrows: [u32; 2],
    pub bolts: [u32; 2],
    pub _0x4: u32,
    pub _0x4_1: u32,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    pub _0x4_2: u32,
    pub talismans: [u32; 4],
    pub unk: u32
}

impl Read for ChrAsm {
    fn read(br: &mut BinaryReader) -> Result<ChrAsm, io::Error> {
        let mut chr_asm = ChrAsm::default();

        chr_asm.arm_style = br.read_u32()?;
        chr_asm.left_hand_active_slot = br.read_u32()?;
        chr_asm.right_hand_active_slot = br.read_u32()?;
        chr_asm.left_arrow_active_slot = br.read_u32()?;
        chr_asm.right_arrow_active_slot = br.read_u32()?;
        chr_asm.left_bolt_active_slot = br.read_u32()?;
        chr_asm.right_bolt_active_slot = br.read_u32()?;
        chr_asm.left_hand_armaments[0] = br.read_u32()?;
        chr_asm.right_hand_armaments[0] = br.read_u32()?;
        chr_asm.left_hand_armaments[1] = br.read_u32()?;
        chr_asm.right_hand_armaments[1] = br.read_u32()?;
        chr_asm.left_hand_armaments[2] = br.read_u32()?;
        chr_asm.right_hand_armaments[2] = br.read_u32()?;
        chr_asm.arrows[0] = br.read_u32()?;
        chr_asm.bolts[0] = br.read_u32()?;
        chr_asm.arrows[1] = br.read_u32()?;
        chr_asm.bolts[1] = br.read_u32()?;
        chr_asm._0x4 = br.read_u32()?;
        chr_asm._0x4_1 = br.read_u32()?;
        chr_asm.head = br.read_u32()?;
        chr_asm.chest = br.read_u32()?;
        chr_asm.arms = br.read_u32()?;
        chr_asm.legs = br.read_u32()?;
        chr_asm._0x4_2 = br.read_u32()?;
        for i in 0..4 { chr_asm.talismans[i] = br.read_u32()?; }
        chr_asm.unk = br.read_u32()?;

        Ok(chr_asm)
    }
}

impl Write for ChrAsm {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.arm_style.to_le_bytes());
        bytes.extend(self.left_hand_active_slot.to_le_bytes());
        bytes.extend(self.right_hand_active_slot.to_le_bytes());
        bytes.extend(self.left_arrow_active_slot.to_le_bytes());
        bytes.extend(self.right_arrow_active_slot.to_le_bytes());
        bytes.extend(self.left_bolt_active_slot.to_le_bytes());
        bytes.extend(self.right_bolt_active_slot.to_le_bytes());
        bytes.extend(self.left_hand_armaments[0].to_le_bytes());
        bytes.extend(self.right_hand_armaments[0].to_le_bytes());
        bytes.extend(self.left_hand_armaments[1].to_le_bytes());
        bytes.extend(self.right_hand_armaments[1].to_le_bytes());
        bytes.extend(self.left_hand_armaments[2].to_le_bytes());
        bytes.extend(self.right_hand_armaments[2].to_le_bytes());
        bytes.extend(self.arrows[0].to_le_bytes());
        bytes.extend(self.bolts[0].to_le_bytes());
        bytes.extend(self.arrows[1].to_le_bytes());
        bytes.extend(self.bolts[1].to_le_bytes());
        bytes.extend(self._0x4.to_le_bytes());
        bytes.extend(self._0x4_1.to_le_bytes());
        bytes.extend(self.head.to_le_bytes());
        bytes.extend(self.chest.to_le_bytes());
        bytes.extend(self.arms.to_le_bytes());
        bytes.extend(self.legs.to_le_bytes());
        bytes.extend(self._0x4_2.to_le_bytes());
        for i in 0..4 { bytes.extend(self.talismans[i].to_le_bytes()); }
        bytes.extend(self.unk.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone, Default)]
pub struct ChrAsm2 {
    pub left_hand_armaments: [u32; 3],
    pub right_hand_armaments: [u32; 3],
    pub arrows: [u32; 2],
    pub bolts: [u32; 2],
    _unk0: u32,
    _unk1: u32,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    _unk2: u32,
    pub talismans: [u32; 4],
    _unk3: u32,
}

impl Read for ChrAsm2 {
    fn read(br: &mut BinaryReader) -> Result<ChrAsm2, io::Error> {
        let mut chr_asm = ChrAsm2::default();
        chr_asm.left_hand_armaments[0] = br.read_u32()?;
        chr_asm.right_hand_armaments[0] = br.read_u32()?;
        chr_asm.left_hand_armaments[1] = br.read_u32()?;
        chr_asm.right_hand_armaments[1] = br.read_u32()?;
        chr_asm.left_hand_armaments[2] = br.read_u32()?;
        chr_asm.right_hand_armaments[2] = br.read_u32()?;
        chr_asm.arrows[0] = br.read_u32()?;
        chr_asm.bolts[0] = br.read_u32()?;
        chr_asm.arrows[1] = br.read_u32()?;
        chr_asm.bolts[1] = br.read_u32()?;
        chr_asm._unk0 = br.read_u32()?;
        chr_asm._unk1 = br.read_u32()?;
        chr_asm.head = br.read_u32()?;
        chr_asm.chest = br.read_u32()?;
        chr_asm.arms = br.read_u32()?;
        chr_asm.legs = br.read_u32()?;
        chr_asm._unk2 = br.read_u32()?;
        for i in 0..4 { chr_asm.talismans[i] = br.read_u32()?; }
        chr_asm._unk3 = br.read_u32()?;
        Ok(chr_asm)
    }
}

impl Write for ChrAsm2 {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.left_hand_armaments[0].to_le_bytes());
        bytes.extend(self.right_hand_armaments[0].to_le_bytes());
        bytes.extend(self.left_hand_armaments[1].to_le_bytes());
        bytes.extend(self.right_hand_armaments[1].to_le_bytes());
        bytes.extend(self.left_hand_armaments[2].to_le_bytes());
        bytes.extend(self.right_hand_armaments[2].to_le_bytes());
        bytes.extend(self.arrows[0].to_le_bytes());
        bytes.extend(self.bolts[0].to_le_bytes());
        bytes.extend(self.arrows[1].to_le_bytes());
        bytes.extend(self.bolts[1].to_le_bytes());
        bytes.extend(self._unk0.to_le_bytes());
        bytes.extend(self._unk1.to_le_bytes());
        bytes.extend(self.head.to_le_bytes());
        bytes.extend(self.chest.to_le_bytes());
        bytes.extend(self.arms.to_le_bytes());
        bytes.extend(self.legs.to_le_bytes());
        bytes.extend(self._unk2.to_le_bytes());
        for i in 0..4 { bytes.extend(self.talismans[i].to_le_bytes()); }
        bytes.extend(self._unk3.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Default, Clone)]
pub struct EquipData {
    pub left_hand_armaments: [u32; 3],
    pub right_hand_armaments: [u32; 3],
    pub arrows: [u32; 2],
    pub bolts: [u32; 2],
    _0x4: u32,
    _0x4_1: u32,
    pub head: u32,
    pub chest: u32,
    pub arms: u32,
    pub legs: u32,
    _0x4_2: u32,
    pub talismans: [u32; 4],
    unk: u32,
}

impl Read for EquipData {
    fn read(br: &mut BinaryReader) -> Result<EquipData, io::Error> {
        let mut equip_data = EquipData::default();
        equip_data.left_hand_armaments[0] = br.read_u32()?;
        equip_data.right_hand_armaments[0] = br.read_u32()?;
        equip_data.left_hand_armaments[1] = br.read_u32()?;
        equip_data.right_hand_armaments[1] = br.read_u32()?;
        equip_data.left_hand_armaments[2] = br.read_u32()?;
        equip_data.right_hand_armaments[2] = br.read_u32()?;
        equip_data.arrows[0] = br.read_u32()?;
        equip_data.bolts[0] = br.read_u32()?;
        equip_data.arrows[1] = br.read_u32()?;
        equip_data.bolts[1] = br.read_u32()?;

        equip_data._0x4 = br.read_u32()?;
        equip_data._0x4_1 = br.read_u32()?;

        equip_data.head = br.read_u32()?;
        equip_data.chest = br.read_u32()?;
        equip_data.arms = br.read_u32()?;
        equip_data.legs = br.read_u32()?;

        equip_data._0x4_2 = br.read_u32()?;

        for i in 0..4 { equip_data.talismans[i] = br.read_u32()?; }
        equip_data.unk = br.read_u32()?;
        
        Ok(equip_data)
    }
}

impl Write for EquipData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.left_hand_armaments[0].to_le_bytes());
        bytes.extend(self.right_hand_armaments[0].to_le_bytes());
        bytes.extend(self.left_hand_armaments[1].to_le_bytes());
        bytes.extend(self.right_hand_armaments[1].to_le_bytes());
        bytes.extend(self.left_hand_armaments[2].to_le_bytes());
        bytes.extend(self.right_hand_armaments[2].to_le_bytes());
        bytes.extend(self.arrows[0].to_le_bytes());
        bytes.extend(self.bolts[0].to_le_bytes());
        bytes.extend(self.arrows[1].to_le_bytes());
        bytes.extend(self.bolts[1].to_le_bytes());

        bytes.extend(self._0x4.to_le_bytes());
        bytes.extend(self._0x4_1.to_le_bytes());

        bytes.extend(self.head.to_le_bytes());
        bytes.extend(self.chest.to_le_bytes());
        bytes.extend(self.arms.to_le_bytes());
        bytes.extend(self.legs.to_le_bytes());

        bytes.extend(self._0x4_2.to_le_bytes());

        for i in 0..4 { bytes.extend(self.talismans[i].to_le_bytes()); }
        bytes.extend(self.unk.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct PlayerGameData {
    _0x4: i32,
    _0x4_1: i32,
    pub health: u32,
    pub max_health: u32,
    pub base_max_health: u32,
    pub fp: u32,
    pub max_fp: u32,
    pub base_max_fp: u32,
    _0x4_2: i32,
    pub sp: u32,
    pub max_sp: u32,
    pub base_max_sp: u32,
    _0x4_3: i32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    _0x4_4: i32,
    _0x4_5: i32,
    _0x4_6: i32,
    pub level: u32,
    pub souls: u32,
    pub soulsmemory: u32,
    _0x28: [u8; 0x28],
    pub character_name: [u16; 0x10],
    _0x2: [u8; 0x2],
    pub gender: u8,
    pub arche_type: u8,
    _0x3_1: [u8; 0x3],
    pub gift: u8,
    _0x1e: [u8; 0x1e],
    pub match_making_wpn_lvl: u8,
    _0x35: [u8; 0x35],
    pub password: [u8; 0x12],
    pub group_password1: [u8; 0x12],
    pub group_password2: [u8; 0x12],
    pub group_password3: [u8; 0x12],
    pub group_password4: [u8; 0x12],
    pub group_password5: [u8; 0x12],
    _unk: [u8; 0x34]
}

impl Default for PlayerGameData {
    fn default() -> Self {
        Self { 
            _0x4: 0,
            _0x4_1: 0,
            health: Default::default(),
            max_health: Default::default(),
            base_max_health: Default::default(),
            fp: Default::default(),
            max_fp: Default::default(),
            base_max_fp: Default::default(),
            _0x4_2: 0,
            sp: Default::default(),
            max_sp: Default::default(),
            base_max_sp: Default::default(),
            _0x4_3: 0,
            vigor: Default::default(),
            mind: Default::default(),
            endurance: Default::default(),
            strength: Default::default(),
            dexterity: Default::default(),
            intelligence: Default::default(),
            faith: Default::default(),
            arcane: Default::default(),
            _0x4_4: 0,
            _0x4_5: 0,
            _0x4_6: 0,
            level: Default::default(),
            souls: Default::default(),
            soulsmemory: Default::default(),
            _0x28: [0; 0x28],
            character_name: [0;0x10],
            _0x2: [0;0x2],
            gender: 0,
            arche_type: 0,
            _0x3_1: [0;0x3],
            gift:0,
            _0x1e: [0; 0x1e],
            match_making_wpn_lvl: 0,
            _0x35: [0; 0x35],
            password: Default::default(),
            group_password1: Default::default(),
            group_password2: Default::default(),
            group_password3: Default::default(),
            group_password4: Default::default(),
            group_password5: Default::default(),
            _unk: [0x0; 0x34]
        }
    }
}

impl Read for PlayerGameData {
    fn read(br: &mut BinaryReader) -> Result<PlayerGameData, io::Error> {
        let mut player_game_data = PlayerGameData::default();
        
        player_game_data._0x4 = br.read_i32()?; 
        player_game_data._0x4_1 = br.read_i32()?;

        // Health
        player_game_data.health = br.read_u32()?;
        player_game_data.max_health = br.read_u32()?;
        player_game_data.base_max_health = br.read_u32()?;

        // FP
        player_game_data.fp = br.read_u32()?;
        player_game_data.max_fp = br.read_u32()?;
        player_game_data.base_max_fp = br.read_u32()?;

        player_game_data._0x4_2 = br.read_i32()?;

        // SP
        player_game_data.sp = br.read_u32()?;
        player_game_data.max_sp = br.read_u32()?;
        player_game_data.base_max_sp = br.read_u32()?;

        player_game_data._0x4_3 = br.read_i32()?;

        // Stats
        player_game_data.vigor = br.read_u32()?;
        player_game_data.mind = br.read_u32()?;
        player_game_data.endurance = br.read_u32()?;
        player_game_data.strength = br.read_u32()?;
        player_game_data.dexterity = br.read_u32()?;
        player_game_data.intelligence = br.read_u32()?;
        player_game_data.faith = br.read_u32()?;
        player_game_data.arcane = br.read_u32()?;

        player_game_data._0x4_4 = br.read_i32()?;
        player_game_data._0x4_5 = br.read_i32()?;
        player_game_data._0x4_6 = br.read_i32()?;

        // Level
        player_game_data.level = br.read_u32()?;
        
        // Souls
        player_game_data.souls = br.read_u32()?;
        player_game_data.soulsmemory = br.read_u32()?;

        player_game_data._0x28.copy_from_slice(br.read_bytes(0x28)?);

        // Character Name
        for i in 0..0x10 {
            player_game_data.character_name[i] = br.read_u16()?;
        }

        player_game_data._0x2.copy_from_slice(br.read_bytes(0x2)?);
        
        // Gender
        player_game_data.gender = br.read_u8()?;
        assert!(player_game_data.gender == 0 || player_game_data.gender == 1);

        // ArcheType
        player_game_data.arche_type = br.read_u8()?;

        player_game_data._0x3_1.copy_from_slice(br.read_bytes(0x3)?);
        
        // Gift
        player_game_data.gift = br.read_u8()?;

        player_game_data._0x1e.copy_from_slice(br.read_bytes(0x1e)?);

        // Weapon Match Making Level
        player_game_data.match_making_wpn_lvl  = br.read_u8()?;

        player_game_data._0x35.copy_from_slice(br.read_bytes(0x35)?);

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

        player_game_data._unk.copy_from_slice(br.read_bytes(0x34)?);

        Ok(player_game_data)
    }
}

impl Write for PlayerGameData {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self._0x4.to_le_bytes());
        bytes.extend(self._0x4_1.to_le_bytes());

        // Health
        bytes.extend(self.health.to_le_bytes());
        bytes.extend(self.max_health.to_le_bytes());
        bytes.extend(self.base_max_health.to_le_bytes());

        // FP
        bytes.extend(self.fp.to_le_bytes());
        bytes.extend(self.max_fp.to_le_bytes());
        bytes.extend(self.base_max_fp.to_le_bytes());

        bytes.extend(self._0x4_2.to_le_bytes());

        // SP
        bytes.extend(self.sp.to_le_bytes());
        bytes.extend(self.max_sp.to_le_bytes());
        bytes.extend(self.base_max_sp.to_le_bytes());

        bytes.extend(self._0x4_3.to_le_bytes());

        // Stats
        bytes.extend(self.vigor.to_le_bytes());
        bytes.extend(self.mind.to_le_bytes());
        bytes.extend(self.endurance.to_le_bytes());
        bytes.extend(self.strength.to_le_bytes());
        bytes.extend(self.dexterity.to_le_bytes());
        bytes.extend(self.intelligence.to_le_bytes());
        bytes.extend(self.faith.to_le_bytes());
        bytes.extend(self.arcane.to_le_bytes());

        bytes.extend(self._0x4_4.to_le_bytes());
        bytes.extend(self._0x4_5.to_le_bytes());
        bytes.extend(self._0x4_6.to_le_bytes());

        // Level
        bytes.extend(self.level.to_le_bytes());

        // Souls
        bytes.extend(self.souls.to_le_bytes());
        bytes.extend(self.soulsmemory.to_le_bytes());

        bytes.extend(self._0x28);

        // Character Name
        for i in 0..0x10 {
            bytes.extend(self.character_name[i].to_le_bytes());
        }

        bytes.extend(self._0x2);

        // Gender
        bytes.push(self.gender);

        // ArcheType
        bytes.push(self.arche_type);

        bytes.extend(self._0x3_1);

        // Gift
        bytes.push(self.gift);

        bytes.extend(self._0x1e);

        // Weapon Match Making Level
        bytes.push(self.match_making_wpn_lvl);

        bytes.extend(self._0x35);

        // Passwords
        bytes.extend(self.password);
        bytes.extend(self.group_password1);
        bytes.extend(self.group_password2);
        bytes.extend(self.group_password3);
        bytes.extend(self.group_password4);
        bytes.extend(self.group_password5);

        bytes.extend(self._unk);

        Ok(bytes)
    }
}

#[derive(Copy, Clone)]
pub struct GaItem {
    pub gaitem_handle: u32,
    pub item_id: u32,
    pub unk2: i32,
    pub unk3: i32,
    pub aow_gaitem_handle: u32,
    pub unk5: u8
}

impl Default for GaItem {
    fn default() -> Self {
        Self { 
            gaitem_handle: 0, 
            item_id: 0, 
            unk2: -1, 
            unk3: -1, 
            aow_gaitem_handle: u32::MAX, 
            unk5: 0
        }
    }
}

impl Read for GaItem {
    fn read(br: &mut BinaryReader) -> Result<GaItem, io::Error> {
        let mut ga_item = GaItem::default();
        ga_item.gaitem_handle = br.read_u32()?;
        ga_item.item_id = br.read_u32()?;

        // Weapon
        if ga_item.item_id != 0 && (ga_item.item_id & 0xf0000000) == 0 {
            ga_item.unk2 = br.read_i32()?;
            ga_item.unk3 = br.read_i32()?;
            ga_item.aow_gaitem_handle = br.read_u32()?;
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

impl Write for GaItem {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.gaitem_handle.to_le_bytes());
        bytes.extend(self.item_id.to_le_bytes());

        if self.item_id != 0 && (self.item_id & 0xf0000000) == 0 {
            bytes.extend(self.unk2.to_le_bytes());
            bytes.extend(self.unk3.to_le_bytes());
            bytes.extend(self.aow_gaitem_handle.to_le_bytes());
            bytes.extend(self.unk5.to_le_bytes());
        }
        else if self.item_id != 0 && (self.item_id & 0xf0000000) == 0x10000000 {
            bytes.extend(self.unk2.to_le_bytes());
            bytes.extend(self.unk3.to_le_bytes());
        }

        Ok(bytes)
    }
}

#[derive(Clone)]
pub struct SaveSlot {
    pub ver: u32,
    pub map_id: [u8; 4],
    _0x18: [u8; 0x18],
    pub ga_items: Vec<GaItem>,
    pub player_game_data: PlayerGameData,
    _0xd0: [u8; 0xd0],
    pub equip_data: EquipData,
    pub chr_asm: ChrAsm,
    pub chr_asm2: ChrAsm2,
    pub equip_inventory_data: EquipInventoryData,
    pub equip_magic_data: EquipMagicData,
    pub equip_item_data: EquipItemData,
    pub equip_gesture_data: [i32;0x6],
    pub equip_projectile_data: EquipProjectileData,
    pub equipped_items: EquippedItems,
    pub equip_physics_data: EquipPhysicsData,
    _0x4: u32,
    _face_data: [u8;0x12f],
    pub storage_inventory_data: EquipInventoryData,
    pub gesture_game_data: Vec<i32>,
    pub regions: Regions,
    pub ride_game_data: RideGameData,
    _0x1: u8, 
    _0x40: [u8;0x40], 
    _0x4_1: i32, 
    _0x4_2: i32, 
    _0x4_3: i32, 
    _menu_profile_save_load: [u8; 0x1008],
    _trophy_equip_data: [u8; 0x34],
    pub ga_item_data: GaItemData,
    _tutorial_data: [u8;0x408],
    _0x1d: [u8; 0x1d],
    pub event_flags: EventFlags,
    _0x1_1: u8,
    _unk_lists: Vec<UknownList>,
    pub player_coords: PlayerCoords,
    _game_man_unkown_values: [u8; 0xf],
    _0x1_2: u32,
    _cs_net_data_chunks: Vec<u8>,
    pub world_area_weather: WorldAreaWeather,
    pub world_area_time: WorldAreaTime,
    _0x10_1: [u8; 0x10],
    pub steam_id: u64,
    _cs_ps5_activity: [u8;0x20],
    _cs_dlc: [u8;0x32],
    _0x80: [u8;0x80],
    _rest: Vec<u8>
}

impl Default for SaveSlot {
    fn default() -> Self {
        Self {
            ver: 0,
            map_id: [0x0; 4],
            _0x18: [0x0; 0x18],
            ga_items: vec![GaItem::default(); 0x1400],
            player_game_data: PlayerGameData::default(),
            _0xd0: [0x0;0xd0],
            equip_data: EquipData::default(),
            chr_asm: ChrAsm::default(),
            chr_asm2: ChrAsm2::default(),
            equip_inventory_data: EquipInventoryData::default(),
            equip_magic_data: EquipMagicData::default(),
            equip_item_data: EquipItemData::default(),
            equip_gesture_data: [Default::default(); 0x6],
            equip_projectile_data: EquipProjectileData::default(),
            equipped_items: EquippedItems::default(),
            equip_physics_data: EquipPhysicsData::default(),
            _0x4: 0,
            _face_data: [0x0;0x12f],
            storage_inventory_data: EquipInventoryData::default(),
            gesture_game_data: vec![0; 0x40],
            regions: Regions::default(),
            ride_game_data: RideGameData::default(),
            _0x1: 0,
            _0x40: [0x0;0x40],
            _0x4_1: 0,
            _0x4_2: 0,
            _0x4_3: 0,
            _menu_profile_save_load: [0x0;0x1008],
            _trophy_equip_data: [0x0;0x34],
            event_flags: EventFlags::default(),
            _0x1_1: 0,
            ga_item_data: GaItemData::default(),
            _tutorial_data: [0x0;0x408],
            _0x1d: [0x0;0x1d],
            _unk_lists: Vec::new(),
            player_coords: PlayerCoords::default(),
            _game_man_unkown_values: [0x0; 0xf],
            _0x1_2: 0,
            _cs_net_data_chunks: vec![0x0; 0x20000],
            world_area_weather: WorldAreaWeather::default(),
            world_area_time: WorldAreaTime::default(),
            _0x10_1: [0x0; 0x10],
            steam_id: Default::default(),
            _cs_ps5_activity: [0x0;0x20],
            _cs_dlc: [0x0; 0x32],
            _0x80: [0x0;0x80],
            _rest: Default::default(),
        }
    }
}

impl Read for SaveSlot {
    fn read(br: &mut BinaryReader) -> Result<SaveSlot, io::Error> {
        let mut save_slot = SaveSlot::default();

        // Slot end position
        let end = br.pos + 0x280000;

        // Unknown
        save_slot.ver = br.read_u32()?;

        // MapId
        save_slot.map_id.copy_from_slice(br.read_bytes(4)?);

        // Uknown
        save_slot._0x18.copy_from_slice(br.read_bytes(0x18)?);

        // GaItem
        for i in 0..0x1400 {
            save_slot.ga_items[i] = GaItem::read(br)?;
        }

        // Player Game Data (Health, Fp, Stats, etc...)
        save_slot.player_game_data = PlayerGameData::read(br)?;
        
        save_slot._0xd0.copy_from_slice(br.read_bytes(0xd0)?);
        
        // Equip Data
        save_slot.equip_data = EquipData::read(br)?;
        
        // Chr Asm
        save_slot.chr_asm = ChrAsm::read(br)?;

        // Chr Asm2
        save_slot.chr_asm2 = ChrAsm2::read(br)?;

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

        // Equip data again
        save_slot.equipped_items = EquippedItems::read(br)?;
        
        // Equipped physics
        save_slot.equip_physics_data = EquipPhysicsData::read(br)?;
        
        save_slot._0x4 = br.read_u32()?;
        
        // Face data (skipping it)
        save_slot._face_data.copy_from_slice(br.read_bytes(0x12f)?);
        
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

        save_slot._0x1 = br.read_bytes(1)?[0];
        save_slot._0x40.copy_from_slice(br.read_bytes(0x40)?);
        save_slot._0x4_1= br.read_i32()?;
        save_slot._0x4_2= br.read_i32()?;
        save_slot._0x4_3= br.read_i32()?;
        
        // Menu Profile Save Load (Skipping)
        save_slot._menu_profile_save_load.copy_from_slice(br.read_bytes(0x1008)?);

        // Trophy Equip Data (Skipping)
        save_slot._trophy_equip_data.copy_from_slice(br.read_bytes(0x34)?);

        // GaItems 
        save_slot.ga_item_data = GaItemData::read(br)?;

        // Tutorial Data (Skipping)
        save_slot._tutorial_data.copy_from_slice(br.read_bytes(0x408)?);

        // Unknown values (Grouping and skipping)
        save_slot._0x1d.copy_from_slice(br.read_bytes(0x1d)?);

        // Event Flags
        save_slot.event_flags = EventFlags::read(br)?;

        save_slot._0x1_1 = br.read_bytes(1)?[0];

        for _i in 0..0x5 {
            save_slot._unk_lists.push(UknownList::read(br)?);
        }
        
        // Player Coordinates
        save_slot.player_coords = PlayerCoords::read(br)?;
        
        save_slot._game_man_unkown_values.copy_from_slice(br.read_bytes(0xf)?);

        save_slot._0x1_2 = br.read_u32()?;
        // Value should always be 2 for active accounts. 0 for empty ones.
        assert!(save_slot._0x1_2 == 2 || save_slot._0x1_2 == 0);

        save_slot._cs_net_data_chunks.copy_from_slice( br.read_bytes(0x20000)?);
        
        save_slot.world_area_weather = WorldAreaWeather::read(br)?;
        save_slot.world_area_time = WorldAreaTime::read(br)?;

        save_slot._0x10_1.copy_from_slice(br.read_bytes(0x10)?);

        save_slot.steam_id = br.read_u64()?;

        // CSPS5Activity (Skipping)
        save_slot._cs_ps5_activity.copy_from_slice(br.read_bytes(0x20)?);

        // DLC?
        save_slot._cs_dlc.copy_from_slice(br.read_bytes(0x32)?);

        save_slot._0x80.copy_from_slice(br.read_bytes(0x80)?);

        save_slot._rest.extend(br.read_bytes(end - br.pos)?);

        Ok(save_slot)
    }
}

impl Write for SaveSlot {
    fn write(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        
        bytes.extend(self.ver.to_le_bytes());

        // MapId
        bytes.extend(self.map_id);

        bytes.extend(self._0x18);

        // GaItem
        for i in 0..0x1400 {
            bytes.extend(self.ga_items[i].write()?);
        }

        // Player Game Data (Health, Fp, Stats, etc...)
        bytes.extend(self.player_game_data.write()?);

        bytes.extend(self._0xd0);

        // Equip Data
        bytes.extend(self.equip_data.write()?);

        // Chr Asm
        bytes.extend(self.chr_asm.write()?);

        // Chr Asm2
        bytes.extend(self.chr_asm2.write()?);

        // Equip Inventory Data
        bytes.extend(self.equip_inventory_data.write(0xa80, 0x180)?);

        // Equip Magic Spell
        bytes.extend(self.equip_magic_data.write()?);

        // Equip Item
        bytes.extend(self.equip_item_data.write()?);

        // Equipped Gestures
        for i in 0..0x6 {
            bytes.extend(self.equip_gesture_data[i].to_le_bytes());
        }
        
        // Equipped Projectiles 
        bytes.extend(self.equip_projectile_data.write()?);

        bytes.extend(self.equipped_items.write()?);

        // Equipped physics
        bytes.extend(self.equip_physics_data.write()?);

        bytes.extend(self._0x4.to_le_bytes());

        // Face Data (sliders)
        bytes.extend(self._face_data);

        // Equip Inventory Data 2
        bytes.extend(self.storage_inventory_data.write(0x780, 0x80)?);

        // Equipped Gestures
        for i in 0..0x40 {
            bytes.extend(self.gesture_game_data[i].to_le_bytes());
        }

        // Regions
        bytes.extend(self.regions.write()?);

        // Horse Data
        bytes.extend(self.ride_game_data.write()?);

        bytes.push(self._0x1);
        bytes.extend(self._0x40);
        bytes.extend(self._0x4_1.to_le_bytes());
        bytes.extend(self._0x4_2.to_le_bytes());
        bytes.extend(self._0x4_3.to_le_bytes());

        // Menu Profile Save Load
        bytes.extend(self._menu_profile_save_load);

        // Trophy Equip Data (Skipping)
        bytes.extend(self._trophy_equip_data);

        // GaItems 
        bytes.extend(self.ga_item_data.write()?);

        // Tutorial Data 
        bytes.extend(self._tutorial_data);

        // Unknown values
        bytes.extend(self._0x1d);

        // Event Flags 
        bytes.extend(self.event_flags.write()?);

        bytes.push(self._0x1_1);

        for i in 0..0x5 as usize {
            bytes.extend(self._unk_lists[i].write()?);
        }

        // Player Coordinates
        bytes.extend(self.player_coords.write()?);

        bytes.extend(self._game_man_unkown_values);

        bytes.extend(self._0x1_2.to_le_bytes());

        // CSNetMan
        bytes.extend(self._cs_net_data_chunks.to_vec());

        // WorldAreaWeather
        bytes.extend(self.world_area_weather.write()?);

        // WorldAreaTime
        bytes.extend(self.world_area_time.write()?);

        bytes.extend(self._0x10_1);
        
        // Steam ID
        bytes.extend(self.steam_id.to_le_bytes());

        // CSPS5Activity
        bytes.extend(self._cs_ps5_activity);

        // DLC
        bytes.extend(self._cs_dlc);

        bytes.extend(self._0x80);

        // Empty calories
        bytes.extend(vec![0;0x280000-bytes.len()]);

        Ok(bytes)
    }
}