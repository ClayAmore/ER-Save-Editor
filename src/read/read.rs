use binary_reader::BinaryReader;
use std::io;

pub trait Read
where
    Self: Sized,
{
    fn read(br: &mut BinaryReader) -> Result<Self, io::Error>;
}
