#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    StubbedError, // An error was raised as part of a test
}
