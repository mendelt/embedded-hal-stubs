use embedded_hal::blocking::spi::{Write, WriteIter};
use embedded_hal_stubs::blocking::spi::SpiStub;
use embedded_hal_stubs::error::TestError;
use embedded_hal_stubs::returns::returns;

#[test]
fn should_return_default_result_for_try_write() {
    let mut stub = SpiStub::arrange().go();
    assert_eq!(stub.try_write(&[8u8, 7u8, 6u8]), Ok(()));
}

#[test]
fn should_arrange_results_for_try_write() {
    let mut stub = SpiStub::arrange()
        .try_write(returns(Err(TestError::StubbedError)).once())
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
        .try_write_iter(returns(Err(TestError::StubbedError)).once())
        .go();

    assert_eq!(
        stub.try_write_iter(vec![8u8, 7u8, 6u8]),
        Err(TestError::StubbedError)
    );
}
