use embedded_hal::blocking::spi::{Transfer, Write, WriteIter};

#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    StubbedError, // An error was raised as part of a test
}

pub struct SpiStub {
    write_result: Result<(), TestError>,
}

impl SpiStub {
    pub fn new() -> Self {
        SpiStub {
            write_result: Ok(()),
        }
    }

    pub fn on_try_write(&mut self, result: Result<(), TestError>) {
        self.write_result = result;
    }
}

impl Write<u8> for SpiStub {
    type Error = TestError;

    fn try_write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        self.write_result.clone()
    }
}

impl Transfer<u8> for SpiStub {
    type Error = TestError;

    fn try_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        todo!()
    }
}

impl WriteIter<u8> for SpiStub {
    type Error = TestError;

    fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_stub() {
        let mut stub = SpiStub::new();
        assert_eq!(stub.try_write(&[8u8, 7u8, 6u8]), Ok(()));
    }

    #[test]
    fn should_return_error() {
        let mut stub = SpiStub::new();
        stub.on_try_write(Err(TestError::StubbedError));
        assert_eq!(
            stub.try_write(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }
}
