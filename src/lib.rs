use embedded_hal::blocking::spi::Write;

#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    StubbedError, // An error was raised as part of a test
}

pub struct SpiStub {
    write_result: (u8, Result<(), TestError>),
}

impl SpiStub {
    pub fn new() -> Self {
        SpiStub {
            write_result: (0u8, Ok(())),
        }
    }

    pub fn on_try_write(&mut self, buffer: u8, result: Result<(), TestError>) {
        self.write_result = (buffer, result)
    }
}

impl Write<u8> for SpiStub {
    type Error = TestError;

    fn try_write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        self.write_result.1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut stub = SpiStub::new();
        assert_eq!(stub.try_write(&[8u8, 7u8, 6u8]), Ok(()));
    }
}
