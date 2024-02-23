use std::io;
use binary_reader::BinaryReader;

pub trait Read where Self: Sized {
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error>;
}