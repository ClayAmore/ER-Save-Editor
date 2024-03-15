pub mod bnd4 {
    use std::io::Error;
    use binary_reader::{BinaryReader, Endian};
    use encoding_rs::{SHIFT_JIS, UTF_16BE, UTF_16LE};

    bitflags::bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Format: u8 {
            const None = 0;
            const BigEndian = 0b0000_0001;
            const IDs = 0b0000_0010;
            const Names1 = 0b0000_0100;
            const Names2 = 0b0000_1000;
            const LongOffsets = 0b0001_0000;
            const Compression = 0b0010_0000;
            const Flag6 = 0b0100_0000;
            const Flag7 = 0b1000_0000;
            const _ = !0;
        }
    }
    impl Format {
        pub fn as_i32(&self) -> i32 {
            self.bits() as i32
        }
    }
    impl Default for Format {
        fn default() -> Self {
            Format::IDs | Format::Names1 | Format::Names2 | Format::Compression
        }
    }

    bitflags::bitflags! {
        #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct FileFlags: u8 {
            const None = 0;
            const Compressed = 0b0000_0001;
            const Flag1 = 0b0000_0010;
            const Flag2 = 0b0000_0100;
            const Flag3 = 0b0000_1000;
            const Flag4 = 0b0001_0000;
            const Flag5 = 0b0010_0000;
            const Flag6 = 0b0100_0000;
            const Flag7 = 0b1000_0000;
            const _ = !0;
        }
    }
    impl FileFlags {
        pub fn as_i32(&self) -> i32 {
            self.bits() as i32
        }
    }

    // I am only using this to read the regulation so I am setting this to (DCX_DFLT_11000_44_9_15 = 10)
    bitflags::bitflags! {
        #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct CompressionType: u8 {
            const Unkown = 0;
            const None = 1;
            const Zlib = 2;
            const Dcp_edge = 3 ;
            const Dcp_dflt = 4;
            const Dcx_edge = 5;
            const Dcx_dflt_10000_24_9 = 6 ;
            const Dcx_dflt_10000_44_9 = 7 ;
            const Dcx_dflt_11000_44_8 = 8;
            const Dcx_dflt_11000_44_9 = 9 ;
            const Dcx_dflt_11000_44_9_15 = 10;
            const Dcx_krak = 11;
        }
    }

    #[derive(Default)]
    pub struct BinderFile {
        pub flags: FileFlags,
        pub id: i32,
        pub name: String,
        pub bytes: Vec<u8>,
        pub compression_type: CompressionType,
        pub uncompressed_size: i64,
        pub data_offset: i64,
    }
    impl BinderFile {
        pub fn new(
            flags: FileFlags,
            id: i32,
            name: String,
            bytes: Vec<u8>,
        ) -> Self {
            let mut binder_file = BinderFile::default();
            binder_file.flags = flags;
            binder_file.id = id;
            binder_file.name = name;
            binder_file.bytes = bytes;
            binder_file
        }
    }

    #[allow(unused)]
    #[derive(Default)]
    pub struct BinderHeader {
        file_flags: FileFlags,
        id: i32,
        name: String,
        compression_type: CompressionType,
        compressed_size: i64,
        uncompressed_size: i64,
        data_offset: i64,
    }

    impl BinderHeader {
        pub fn read_file_data(&self, br: &mut BinaryReader) -> Result<BinderFile, Error> {
            let mut bytes: Vec<u8> = Vec::new();
            let _compression_type = CompressionType::Zlib;

            if Binder::is_compressed(self.file_flags) {
                todo!();
            }
            else {
                let prev_pos = br.pos;
                br.jmp(self.data_offset as usize);
                bytes.extend(br.read_bytes(self.uncompressed_size as usize)?);
                br.jmp(prev_pos);
            }

            Ok(BinderFile::new(
                self.file_flags,
                self.id,
                self.name.to_string(),
                bytes
            ))
        }

        pub fn read_binder4_file_header(br: &mut BinaryReader, format: Format, big_endian: bool, bit_big_endian: bool, unicode: bool) -> Result<Self, Error> {
            let byte = br.read_u8()?;
            let raw_flags = if big_endian {i32::from_be_bytes([byte, 0, 0, 0])} else {i32::from_le_bytes([byte, 0, 0, 0])};
            
            let file_flags = if bit_big_endian {raw_flags} else {
                ((raw_flags & 0b00000001) << 7) |
                ((raw_flags & 0b00000010) << 5) |
                ((raw_flags & 0b00000100) << 3) |
                ((raw_flags & 0b00001000) << 1) |
                ((raw_flags & 0b00010000) >> 1) |
                ((raw_flags & 0b00100000) >> 3) |
                ((raw_flags & 0b01000000) >> 5) |
                ((raw_flags & 0b10000000) >> 7)
            };

            assert_eq!(br.read_u8()?, 0);
            assert_eq!(br.read_u8()?, 0);
            assert_eq!(br.read_u8()?, 0);
            assert_eq!(br.read_i32()?, -1);

            let compressed_size = br.read_i64()?;
            
            let uncompressed_size = if Binder::has_compression(format) { 
                br.read_i64()?
            }
            else { 
                br.read_i32()? as i64
            };

            let data_offset = if Binder::has_long_offsets(format) {
                br.read_i64()?
            }
            else {
                br.read_i32()? as i64
            };


            let mut id = -1;
            if Binder::has_ids(format) {
                id = br.read_i32()?;
            }

            let mut name = String::new();
            if Binder::has_names(format) {
                let name_offset = br.read_i32()?;

                let saved_pos = br.pos;
                br.jmp(name_offset as usize);
                if unicode {
                    let mut bytes: Vec<u8>= Vec::new();
                    let mut pair = br.read_bytes(2)?;
                    while pair[0] != 0 || pair[1] != 0 {
                        bytes.extend(pair);
                        pair = br.read_bytes(2)?;
                    }
                    
                    name = if big_endian {
                        let (res, _enc, errors) = UTF_16BE.decode(&bytes);
                        if errors {
                            eprintln!("Failed");
                            String::new()
                        } else {
                            res.to_string()
                        }   
                    } else {
                        let (res, _enc, errors) = UTF_16LE.decode(&bytes);
                        if errors {
                            eprintln!("Failed");
                            String::new()
                        } else {
                            res.to_string()
                        }   
                    };
                }
                else {
                    let mut bytes: Vec<u8>= Vec::new();
                    let mut b = br.read_u8()?;
                    while b != 0 {
                        bytes.push(b);
                        b = br.read_u8()?;
                    }
                    let (res, _enc, errors) = SHIFT_JIS.decode(&bytes);
                    if errors {
                        eprintln!("Failed");
                    } else {
                        name = res.to_string();
                    }   
                }
                br.jmp(saved_pos);
            }

            if format == Format::Names1 {
                id = br.read_i32()?;
                assert_eq!(br.read_i32()?, 0);
            }
            

            Ok(Self {
                file_flags: FileFlags::from_bits_truncate(file_flags as u8),
                id,
                name,
                compression_type: CompressionType::Dcx_dflt_11000_44_9_15,
                compressed_size,
                uncompressed_size,
                data_offset,
            })
        }
    }


    pub struct Binder {
    }

    impl Binder {
        fn get_bnd4_header_size(format: Format) -> i64 {
            let mut size = 0x10;
            if Binder::has_long_offsets(format) {
                size += 8;
            }
            if Binder::has_compression(format) {
                size += 8;
            }
            if Binder::has_ids(format) {
                size += 4;
            }
            if Binder::has_names(format) {
                size += 4;
            }
            if format == Format::Names1 {
                size += 8;
            }
            size
        }
    
        fn has_long_offsets(format: Format) -> bool {
            (format & Format::LongOffsets).as_i32() != 0
        }
    
        fn has_compression(format: Format) -> bool {
            (format & Format::Compression).as_i32() != 0
        }
    
        fn has_ids(format: Format) -> bool {
            (format & Format::IDs).as_i32() != 0
        }
    
        fn has_names(format: Format) -> bool {
            (format & (Format::Names1 | Format::Names2)).as_i32() != 0
        }
    
        fn is_compressed(flags: FileFlags) -> bool {
            (flags& FileFlags::Compressed).as_i32() != 0
        }
    }


    #[derive(Default)]
    pub struct BND4 {
        pub files: Vec<BinderFile>,
        pub version: String,
        pub format: Format,
        pub unk04: bool,
        pub unk05: bool,
        pub big_endian: bool,
        pub bit_big_endian: bool,
        pub unicode: bool,
        pub extended: i32,
    }

    impl BND4 {
        pub fn new() -> Self {
            let mut bnd4 = BND4::default();
            bnd4.unicode = true;
            bnd4.extended = 4;
            bnd4
        }

        #[allow(unused)]
        pub fn is(&self, br: &mut BinaryReader) -> bool {
            if br.length < 4 {
                return false;
            }
            let magic = br.read_bytes(4).unwrap();
            magic == b"BND4"
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<BND4, Error>{
            let mut br = BinaryReader::from_u8(bytes);
            let mut bnd4 = BND4::new();
            bnd4.read(&mut br).expect("");
            Ok(bnd4)
        }

        pub fn read(&mut self, br: &mut BinaryReader) -> Result<(), Error>{
            let file_headers = self.read_header(br)?;
            for file_header in file_headers {
                self.files.push(file_header.read_file_data(br)?);
            }
            Ok(())
        }

        fn read_header(&mut self, br: &mut BinaryReader) -> Result<Vec<BinderHeader>, Error> {
            assert_eq!(br.read_bytes(4)?, b"BND4");

            self.unk04 = br.read_bool()?;
            self.unk05 = br.read_bool()?;
            assert_eq!(br.read_u8()?, 0);
            assert_eq!(br.read_u8()?, 0);

            assert_eq!(br.read_u8()?, 0);
            self.big_endian = br.read_bool()?;
            self.bit_big_endian = br.read_bool()?;
            assert_eq!(br.read_u8()?, 0);

            br.set_endian(if self.big_endian {binary_reader::Endian::Big} else {Endian::Little});

            let file_count = br.read_i32()?;
            assert_eq!(br.read_i64()?, 0x40);
            
            let bytes = br.read_bytes(8)?;
            let mut terminator = 0;
            for i in 0..8 {
                if bytes[i] == 0 {
                    break
                }
                terminator = i;
            }
            let (res, _enc, errors) = SHIFT_JIS.decode(&bytes[0..terminator]);
            if errors {
                eprintln!("Failed");
            } else {
                self.version = res.to_string();
            }
            
            let file_header_size = br.read_i64()?;
            let _unused00 = br.read_i64();

            self.unicode = br.read_bool()?;
            let byte = br.read_u8()?;
            let raw_format = if self.big_endian {i32::from_be_bytes([byte, 0, 0, 0])} else {i32::from_le_bytes([byte, 0, 0, 0])};
            let reverse: bool = self.bit_big_endian || (raw_format & 1) != 0 && (raw_format & 0b1000_0000) == 0;
            let format = if reverse {Format::from_bits_truncate(raw_format as u8)} else {
                Format::from_bits_truncate(
                (((raw_format & 0b00000001) << 7) |
                ((raw_format & 0b00000010) << 5) |
                ((raw_format & 0b00000100) << 3) |
                ((raw_format & 0b00001000) << 1) |
                ((raw_format & 0b00010000) >> 1) |
                ((raw_format & 0b00100000) >> 3) |
                ((raw_format & 0b01000000) >> 5) |
                ((raw_format & 0b10000000) >> 7)) as u8)
            };

            let b = br.read_u8()?;
            self.extended = -1;
            if  b == 0 {
                self.extended = 0;
            } else if b == 1 {
                self.extended = 1;
            } else if b == 4 {
                self.extended = 4;
            } else if b == 0x80 {
                self.extended = 0x80;
            };
            assert!(self.extended == 0 || self.extended == 1 || self.extended == 4 || self.extended == 0x80);
            assert_eq!(br.read_u8()?, 0);

            assert_eq!(br.read_i32()?, 0);

            if self.extended == 4 {
                let hash_table_offset = br.read_i64()?;
                let prev_pos = br.pos;
                br.jmp(hash_table_offset as usize);
                let _unused01 = br.read_i64();
                let _unused02 = br.read_i32();
                assert_eq!(br.read_u8()?, 0x10);
                assert_eq!(br.read_u8()?, 8);
                assert_eq!(br.read_u8()?, 8);
                assert_eq!(br.read_u8()?, 0);
                br.jmp(prev_pos);
            }
            else {
                assert_eq!(br.read_i64()?, 0);
            }

            if file_header_size != Binder::get_bnd4_header_size(format) {
                panic!("File header size for format {} is expected to be {:#X}, but was {:#X}", self.format.as_i32(), Binder::get_bnd4_header_size(self.format), file_header_size);
            }

            let mut file_headers: Vec<BinderHeader> = Vec::with_capacity(file_count as usize);
            for _i in 0..file_count {
                file_headers.push(BinderHeader::read_binder4_file_header(
                    br, 
                    self.format, 
                    self.big_endian, 
                    self.bit_big_endian, 
                    self.unicode
                )?);
            }

            Ok(file_headers)
        }
    }
}