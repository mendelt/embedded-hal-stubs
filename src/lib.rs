
use embedded_hal::blocking::spi::Write;
use std::io;

#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    /// An I/O-Error occurred
    Io(io::ErrorKind),
}

impl From<io::Error> for TestError {
    fn from(e: io::Error) -> Self {
        TestError::Io(e.kind())
    }
}

pub struct SpiStub {}

impl Write<u8> for SpiStub {
    type Error = TestError;

    fn try_write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
