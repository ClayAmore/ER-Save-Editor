use std::{collections::HashMap, io::Error, path::PathBuf, str::FromStr};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use binary_reader::{BinaryReader, Endian};
use miniz_oxide::inflate::decompress_to_vec;

use super::{bnd4::bnd4::BND4, params::params::Param};

pub struct Regulation;

impl Regulation {
    pub fn params_from_regulation(bytes: &[u8]) -> Result<HashMap<Param, Vec<u8>>, Error>{
        let decrypted = Self::decrypt(&bytes)?;
        let decompressed = Self::decompress(&decrypted)?;
        let res = Self::unpack(&decompressed)?;
        let mut params: HashMap<Param, Vec<u8>> = HashMap::new();

        for file in &res.files {
            let path = PathBuf::from_str(&file.name).unwrap();
            let file_name = path.file_stem().unwrap().to_str().unwrap();
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
        assert_eq!(br.read_bytes(4)?, b"DFLT");
        assert_eq!(br.read_i32()?, 0x20);
        assert_eq!(br.read_u8()?, 9);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_i32()?, 0);
        assert_eq!(br.read_u8()?, 15);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_u8()?, 0);
        assert_eq!(br.read_i32()?, 0);
        assert_eq!(br.read_i32()?, 0x00010100);

        assert_eq!(br.read_bytes(4)?, b"DCA\0");
        let _compressed_header_length = br.read_i32()?;

        assert_eq!(br.read_u8()?, 0x78);
        let assert_value = br.read_u8()?;
        assert!(assert_value == 0x01 || assert_value == 0x5E || assert_value == 0x9C || assert_value == 0xDA);
        
        let compressed = br.read_bytes(compressed_size as usize - 2)?;
        let res = decompress_to_vec(compressed);

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