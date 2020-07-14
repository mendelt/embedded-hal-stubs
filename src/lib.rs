use embedded_hal::blocking::spi::Write;

#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    StubbedError  // An error was raised as part of a test
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
    use super::*;

    #[test]
    fn it_works() {
        let mut stub = SpiStub {};
        assert_eq!(stub.try_write(&[8u8, 7u8, 6u8]), Ok(()));
    }
}
