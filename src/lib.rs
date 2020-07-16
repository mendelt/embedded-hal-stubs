use embedded_hal::blocking::spi::{Transfer, Write, WriteIter};

#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    StubbedError, // An error was raised as part of a test
}

pub struct SpiStub {
    write_result: Result<(), TestError>,
    write_iter_result: Result<(), TestError>,
}

impl SpiStub {
    pub fn new() -> Self {
        SpiStub {
            write_result: Ok(()),
            write_iter_result: Ok(()),
        }
    }

    pub fn on_try_write(&mut self, result: Result<(), TestError>) {
        self.write_result = result;
    }

    pub fn on_try_write_iter(&mut self, result: Result<(), TestError>) {
        self.write_iter_result = result;
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
        self.write_iter_result.clone()
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
    fn should_return_error_on_try_write() {
        let mut stub = SpiStub::new();
        stub.on_try_write(Err(TestError::StubbedError));
        assert_eq!(
            stub.try_write(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }

    #[test]
    fn should_return_error_on_try_write_iter() {
        let mut stub = SpiStub::new();
        stub.on_try_write_iter(Err(TestError::StubbedError));
        assert_eq!(
            stub.try_write_iter(vec![8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }
}
