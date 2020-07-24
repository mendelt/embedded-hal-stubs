use crate::{
    error::TestError,
    returns::{returns, Returns},
};
use embedded_hal::blocking::spi::{Transfer, Write, WriteIter};

pub struct SpiStub {
    on_write: Returns<Result<(), TestError>>,
    on_write_iter: Returns<Result<(), TestError>>,
}

impl SpiStub {
    pub fn arrange() -> Self {
        SpiStub {
            on_write: returns(Ok(())).always(),
            on_write_iter: returns(Ok(())).always(),
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
