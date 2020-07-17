use embedded_hal::blocking::spi::{Transfer, Write, WriteIter};

#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    StubbedError, // An error was raised as part of a test
}

pub struct Returns<T> {
    return_values: Vec<T>,
}

impl<T: Clone> Returns<T> {
    fn values() -> Self {
        Returns {
            return_values: Vec::new(),
        }
    }

    fn once(mut self, value: T) -> Self {
        self.return_values.push(value);
        self
    }

    fn get_by_params(&mut self) -> T {
        // Todo, look up values by parameters of call

        let (head, tail) = self.return_values.split_first().unwrap();
        let result = head.clone();
        self.return_values = tail.to_vec();
        result
    }
}

pub struct SpiStub {
    write_result: Returns<Result<(), TestError>>,
    write_iter_result: Result<(), TestError>,
}

impl SpiStub {
    pub fn arrange() -> Self {
        SpiStub {
            write_result: Returns::values(),
            write_iter_result: Ok(()),
        }
    }

    pub fn try_write(mut self, values: Returns<Result<(), TestError>>) -> Self {
        self.write_result = values;
        self
    }

    pub fn try_write_iter(mut self, result: Result<(), TestError>) -> Self {
        self.write_iter_result = result;
        self
    }

    pub fn go(self) -> SpiStubImpl {
        SpiStubImpl {
            write_result: self.write_result,
            write_iter_result: self.write_iter_result,
        }
    }
}

pub struct SpiStubImpl {
    write_result: Returns<Result<(), TestError>>,
    write_iter_result: Result<(), TestError>,
}

impl Write<u8> for SpiStubImpl {
    type Error = TestError;

    fn try_write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        self.write_result.get_by_params()
    }
}

impl Transfer<u8> for SpiStubImpl {
    type Error = TestError;

    fn try_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        todo!()
    }
}

impl WriteIter<u8> for SpiStubImpl {
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
        let mut stub = SpiStub::arrange().go();
        assert_eq!(stub.try_write(&[8u8, 7u8, 6u8]), Ok(()));
    }

    #[test]
    fn should_return_error_on_try_write() {
        let mut stub = SpiStub::arrange()
            .try_write(Returns::values().once(Err(TestError::StubbedError)))
            .go();

        assert_eq!(
            stub.try_write(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }

    #[test]
    fn should_return_error_on_try_write_iter() {
        let mut stub = SpiStub::arrange()
            .try_write_iter(Err(TestError::StubbedError))
            .go();

        assert_eq!(
            stub.try_write_iter(vec![8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }
}
