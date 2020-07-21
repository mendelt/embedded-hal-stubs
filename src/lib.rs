use embedded_hal::blocking::spi::{Transfer, Write, WriteIter};

mod error;
mod returns;

use crate::error::TestError;
use returns::{returns, Returns};

pub struct SpiStub {
    on_write: Returns<Result<(), TestError>>,
    on_write_iter: Returns<Result<(), TestError>>,
}

impl SpiStub {
    pub fn arrange() -> Self {
        SpiStub {
            on_write: returns().always(Ok(())),
            on_write_iter: returns().always(Ok(())),
        }
    }

    pub fn try_write(mut self, values: Returns<Result<(), TestError>>) -> Self {
        self.on_write = values;
        self
    }

    pub fn try_write_iter(mut self, result: Returns<Result<(), TestError>>) -> Self {
        self.on_write_iter = result;
        self
    }

    pub fn go(self) -> SpiStubRunner {
        SpiStubRunner {
            on_write: self.on_write,
            on_write_iter: self.on_write_iter,
        }
    }
}

pub struct SpiStubRunner {
    on_write: Returns<Result<(), TestError>>,
    on_write_iter: Returns<Result<(), TestError>>,
}

impl Write<u8> for SpiStubRunner {
    type Error = TestError;

    fn try_write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        self.on_write.get_by_params()
    }
}

impl Transfer<u8> for SpiStubRunner {
    type Error = TestError;

    fn try_transfer<'w>(&mut self, _: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        todo!()
    }
}

impl WriteIter<u8> for SpiStubRunner {
    type Error = TestError;

    fn try_write_iter<WI>(&mut self, _: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        self.on_write_iter.get_by_params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_default_result_for_try_write() {
        let mut stub = SpiStub::arrange().go();
        assert_eq!(stub.try_write(&[8u8, 7u8, 6u8]), Ok(()));
    }

    #[test]
    fn should_arrange_results_for_try_write() {
        let mut stub = SpiStub::arrange()
            .try_write(returns().once(Err(TestError::StubbedError)))
            .go();
        assert_eq!(stub.try_write(&[]), Err(TestError::StubbedError));
    }

    #[test]
    fn should_return_default_result_for_try_write_iter() {
        let mut stub = SpiStub::arrange().go();
        assert_eq!(stub.try_write_iter(vec![]), Ok(()));
    }

    #[test]
    fn should_arrange_results_for_try_write_iter() {
        let mut stub = SpiStub::arrange()
            .try_write_iter(returns().once(Err(TestError::StubbedError)))
            .go();

        assert_eq!(
            stub.try_write_iter(vec![8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }
}
