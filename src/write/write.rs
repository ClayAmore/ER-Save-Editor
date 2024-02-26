use std::io;

pub trait Write where Self: Sized {
    fn write(&self) -> Result<Vec<u8>, io::Error>;
}