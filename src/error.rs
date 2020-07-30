#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    // This error can be raised as part of a test
    StubbedError,
}
