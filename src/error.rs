//! This module contains the error types for the embedded-hal-stubs library

/// Error type that is returned by stubbed methods
#[derive(PartialEq, Clone, Debug)]
pub enum TestError {
    /// This error can be raised as part of a test
    StubbedError,
}
