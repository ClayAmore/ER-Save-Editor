use std::{collections::HashMap, io::Error, str::FromStr, sync::{Mutex, RwLock}};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use binary_reader::{BinaryReader, Endian};
use once_cell::sync::{Lazy, OnceCell};

use crate::{db::{accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME, armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME, weapon_name::weapon_name::WEAPON_NAME}, save::save::save::Save, util::{param_structs::{EQUIP_PARAM_ACCESSORY_ST, EQUIP_PARAM_GEM_ST, EQUIP_PARAM_GOODS_ST, EQUIP_PARAM_PROTECTOR_ST, EQUIP_PARAM_WEAPON_ST}, params::params::{Row, PARAM}}};

use super::{bnd4::bnd4::BND4, params::params::Param};

pub static PARAMS: Lazy<RwLock<HashMap<Param, Vec<u8>>>> = Lazy::new(|| RwLock::new(Default::default()));

pub struct Regulation;

impl Regulation {
    pub fn init_params(save: &Save) {
        let res = Regulation::params_from_regulation(save.save_type.get_regulation());

        
        match res {
            Ok(res) => *PARAMS.write().unwrap() = res,
            Err(err) => println!("{err}"),
        }

        
    }

    pub fn equip_accessory_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_ACCESSORY_ST>> {
        static ACCESSORY_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_ACCESSORY_ST>>> = OnceCell::new();
        ACCESSORY_PARAM_MAP.get_or_init(|| { 
            let mut map = Self::get_param_map::<EQUIP_PARAM_ACCESSORY_ST>(&Param::EquipParamAccessory);
            Self::try_fill_names::<EQUIP_PARAM_ACCESSORY_ST>(&mut map, &ACCESSORY_NAME);
            map
        })
    }

    pub fn equip_gem_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_GEM_ST>> {
        static GEM_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_GEM_ST>>> = OnceCell::new();
        GEM_PARAM_MAP.get_or_init(|| { 
            let mut map = Self::get_param_map::<EQUIP_PARAM_GEM_ST>(&Param::EquipParamGem); 
            Self::try_fill_names::<EQUIP_PARAM_GEM_ST>(&mut map, &AOW_NAME);
            map
        })
    }

    pub fn equip_goods_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_GOODS_ST>> {
        static GOOD_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_GOODS_ST>>> = OnceCell::new();
        GOOD_PARAM_MAP.get_or_init(|| { 
            let mut map = Self::get_param_map::<EQUIP_PARAM_GOODS_ST>(&Param::EquipParamGoods); 
            Self::try_fill_names::<EQUIP_PARAM_GOODS_ST>(&mut map, &ITEM_NAME);
            map
        })
    }

    pub fn equip_protectors_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_PROTECTOR_ST>> {
        static PROTECTOR_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_PROTECTOR_ST>>> = OnceCell::new();
        PROTECTOR_PARAM_MAP.get_or_init(|| { 
            let mut map = Self::get_param_map::<EQUIP_PARAM_PROTECTOR_ST>(&Param::EquipParamProtector); 
            Self::try_fill_names::<EQUIP_PARAM_PROTECTOR_ST>(&mut map, &ARMOR_NAME);
            map
        })
    }

    pub fn equip_weapon_params_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_WEAPON_ST>> {
        static WEAPON_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_WEAPON_ST>>> = OnceCell::new();
        WEAPON_PARAM_MAP.get_or_init(|| { 
            let mut map = Self::get_param_map::<EQUIP_PARAM_WEAPON_ST>(&Param::EquipParamWeapon); 
            Self::try_fill_names::<EQUIP_PARAM_WEAPON_ST>(&mut map, &WEAPON_NAME);
            map
        })
    }

    fn get_param_map<T>(param: &Param) -> HashMap<u32, Row<T>> where T: Default + Clone {
        PARAM::<T>::from_bytes(&PARAMS.read().unwrap()[param])
            .unwrap()
            .rows.into_iter()
            .map(|row| (row.id, row))
            .collect::<HashMap<u32, Row<T>>>()
    }
    
    fn try_fill_names<T>(rows: &mut HashMap<u32, Row<T>>, map: &Lazy<Mutex<HashMap<u32, &str>>>) where T: Default + Clone {
        rows.iter_mut().for_each(|(_, entry)| {
            entry.name = match map.lock().unwrap().get(&(entry.id)) {
                Some(name) => if !name.is_empty() {name.to_string()} else {format!("[UNKOWN_{}]", entry.id)},
                None => format!("[UNKOWN_{}]", entry.id),
            };
        });
    }

    pub fn params_from_regulation(bytes: &[u8]) -> Result<HashMap<Param, Vec<u8>>, Error>{
        let decrypted = Self::decrypt(&bytes)?;
        let decompressed = Self::decompress(&decrypted)?;
        let res = Self::unpack(&decompressed)?;
        let mut params: HashMap<Param, Vec<u8>> = HashMap::new();

        for file in &res.files {
            let file_name = &file.name.split("\\").last().expect("Could not locate file name").split(".").nth(0).expect("Could not locate file name without extension");
            let param_type = Param::from_str(file_name)?;
            params.insert(param_type, file.bytes.to_vec());
        }
        Ok(params)
    }

    // Decrypt the regulation file (AES)
    fn decrypt(cipher_text: &[u8]) -> Result<Vec<u8>, Error> {
        type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
        let key = [0x99, 0xBF, 0xFC, 0x36, 0x6A, 0x6B, 0xC8, 0xC6, 0xF5, 0x82, 0x7D, 0x09, 0x36, 0x02, 0xD6, 0x76, 0xC4, 0x28, 0x92, 0xA0, 0x1C, 0x20, 0x7F, 0xB0, 0x24, 0xD3, 0xAF, 0x4E, 0x49, 0x3F, 0xEF, 0x99];
        let iv = &cipher_text[0..16];
        let mut buf = cipher_text[16..cipher_text.len()].to_vec();
        let pt: &[u8] = Aes256CbcDec::new(&key.into(), iv.into())
            .decrypt_padded_mut::<NoPadding>(&mut buf)
            .unwrap();
        Ok(pt.to_vec())
    }

    // Decompress the decrypted regulation file (compression_type: DCX_DFLT_11000_44_9_15)
    fn decompress(bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let mut br = BinaryReader::from_u8(bytes);
        br.endian = Endian::Big;

        assert_eq!(br.read_bytes(4)?, b"DCX\0");
        assert_eq!(br.read_i32()?,0x11000);
        assert_eq!(br.read_i32()?, 0x18);
        assert_eq!(br.read_i32()?, 0x24);
        assert_eq!(br.read_i32()?, 0x44);
        assert_eq!(br.read_i32()?, 0x4c);
        
        assert_eq!(br.read_bytes(4)?, b"DCS\0");
        let _decompressed_size = br.read_i32()?;
        let compressed_size = br.read_i32()?;
        
        assert_eq!(br.read_bytes(4)?, b"DCP\0");
        assert_eq!(br.read_bytes(4)?, b"ZSTD");
        assert_eq!(br.read_i32()?, 0x20);
        assert_eq!(br.read_u8()?, 0x15);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_i32()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_i32()?, 0);
        assert_eq!(br.read_i32()?, 0x00010100);

        assert_eq!(br.read_bytes(4)?, b"DCA\0");
        assert_eq!(br.read_i32()?, 8);

        let compressed = br.read_bytes(compressed_size as usize)?;
        let res = zstd::decode_all(compressed);

        match res {
            Ok(decompressed) => Ok(decompressed),
            Err(msg) => { Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{msg}"))) },
        }
    }

    // Unpack the decrypted and decompressed regulation file (BND4)
    fn unpack(bytes: &[u8]) -> Result<BND4, Error>{
        BND4::from_bytes(bytes)
    }
}