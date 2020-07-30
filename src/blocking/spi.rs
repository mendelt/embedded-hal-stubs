//! This module contains types for stubbing embedded-hal SPI traits

use crate::{
    error::TestError,
    returns::{returns, Returns},
};
use embedded_hal::blocking::spi::{Transfer, Write, WriteIter};

/// Entry point for stubbing the embedded-hal SPI traits. This is a builder that can be used to
/// program the behavior of SPI trait methods. The ```go```-method then builds a SpiStubRunner
/// struct that implements the needed traits and can be used as a stub for testing SPI behavior.
#[derive(Debug)]
pub struct SpiStub {
    on_write: Returns<Result<(), TestError>>,
    on_write_iter: Returns<Result<(), TestError>>,
}

impl SpiStub {
    /// Initialize a SpiStub builder
    pub fn arrange() -> Self {
        SpiStub {
            on_write: returns(Ok(())).always(),
            on_write_iter: returns(Ok(())).always(),
        }
    }

    /// Program try_write behavior.
    pub fn try_write(mut self, values: Returns<Result<(), TestError>>) -> Self {
        self.on_write = values;
        self
    }

    /// Program try_write_iter behavior
    pub fn try_write_iter(mut self, result: Returns<Result<(), TestError>>) -> Self {
        self.on_write_iter = result;
        self
    }

    /// Finalize builder and return a SpiStubRunner
    pub fn go(self) -> SpiStubRunner {
        SpiStubRunner {
            on_write: self.on_write,
            on_write_iter: self.on_write_iter,
        }
    }
}

/// The SpiStubRunner is returned by the SpiStub after it is finalized by calling ```SpiStub::go```. 
/// The SpiStubRunner handles the actual Spi calls during execution of tests.
#[derive(Debug)]
pub struct SpiStubRunner {
    on_write: Returns<Result<(), TestError>>,
    on_write_iter: Returns<Result<(), TestError>>,
}

impl Write<u8> for SpiStubRunner {
    type Error = TestError;

    fn try_write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        self.on_write.get_match()
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
        self.on_write_iter.get_match()
    }
}
