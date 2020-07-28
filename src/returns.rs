#[derive(Clone)]
pub enum Return<R> {
    Times(R, u32),
    Always(R),
}

pub fn returns<R>(value: R) -> ReturnsBuilder<R> {
    ReturnsBuilder {
        previous: Returns::default(),
        new_result: value,
    }
}

impl<R> ReturnsBuilder<R> {
    pub fn once(mut self) -> Returns<R> {
        self.previous
            .return_values
            .push(Return::Times(self.new_result, 1));
        self.previous
    }

    pub fn twice(mut self) -> Returns<R> {
        self.previous
            .return_values
            .push(Return::Times(self.new_result, 2));
        self.previous
    }

    pub fn times(mut self, n: u32) -> Returns<R> {
        self.previous
            .return_values
            .push(Return::Times(self.new_result, n));
        self.previous
    }

    pub fn always(mut self) -> Returns<R> {
        self.previous
            .return_values
            .push(Return::Always(self.new_result));
        self.previous
    }
}

pub struct ReturnsBuilder<R> {
    pub(self) previous: Returns<R>,
    pub(self) new_result: R,
}

/// Stores the set of return values for a stubbed method where T is the return type and implements
/// the fluent interface for specifying the return values
pub struct Returns<R> {
    pub(self) return_values: Vec<Return<R>>,
}

impl<R> Returns<R> {
    pub fn returns(self, value: R) -> ReturnsBuilder<R> {
        ReturnsBuilder {
            previous: self,
            new_result: value,
        }
    }
}

impl<R: Clone> Returns<R> {
    /// Get a matching result for a method call
    pub fn get_match(&mut self) -> R {
        match self.return_values.split_first() {
            Some((Return::Always(value), _)) => value.clone(),
            Some((Return::Times(value, n), tail)) if *n <= 1u32 => {
                let result = value.clone();
                self.return_values = tail.to_vec();
                result
            }
            Some((Return::Times(value, n), _)) => {
                let result = value.clone();
                self.return_values[0] = Return::Times(value.clone(), n - 1);
                result
            }
            _ => panic!("No expected result available"),
        }
    }
}

impl<R> Default for Returns<R> {
    fn default() -> Self {
        Returns {
            return_values: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::TestError;
    use cool_asserts::assert_panics;

    struct TestStub {
        on_test_method: Returns<Result<(), TestError>>,
    }

    impl TestStub {
        fn arrange() -> Self {
            TestStub {
                on_test_method: returns(Ok(())).always(),
            }
        }

        pub fn test_method(mut self, result: Returns<Result<(), TestError>>) -> Self {
            self.on_test_method = result;
            self
        }

        fn go(self) -> TestStubRunner {
            TestStubRunner {
                on_test_method: self.on_test_method,
            }
        }
    }

    struct TestStubRunner {
        on_test_method: Returns<Result<(), TestError>>,
    }

    impl TestStubRunner {
        fn test_method(&mut self, _: &[u8]) -> Result<(), TestError> {
            self.on_test_method.get_match()
        }
    }

    #[test]
    fn should_return_default_result() {
        let mut stub = TestStub::arrange().go();

        assert_eq!(stub.test_method(&[8u8, 7u8, 6u8]), Ok(()));
    }

    #[test]
    fn should_return_arranged_result() {
        let mut stub = TestStub::arrange()
            .test_method(returns(Err(TestError::StubbedError)).once())
            .go();

        assert_eq!(
            stub.test_method(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }

    #[test]
    fn should_return_once_result_only_once() {
        let mut stub = TestStub::arrange()
            .test_method(returns(Err(TestError::StubbedError)).once())
            .go();

        assert_eq!(
            stub.test_method(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );

        // This should panic the second time it's called
        assert_panics!(
            {
                let mut stub = stub;
                stub.test_method(&[]).ok();
            },
            includes("No expected result available")
        );
    }

    #[test]
    fn should_return_twice_results_twice() {
        let mut stub = TestStub::arrange()
            .test_method(returns(Err(TestError::StubbedError)).twice())
            .go();

        for _ in 0..2 {
            assert_eq!(
                stub.test_method(&[8u8, 7u8, 6u8]),
                Err(TestError::StubbedError)
            );
        }

        // This should panic the third time
        assert_panics!(
            {
                let mut stub = stub;
                stub.test_method(&[]).ok();
            },
            includes("No expected result available")
        );
    }

    #[test]
    fn should_return_times_returns_n_times() {
        let n = 48;
        let mut stub = TestStub::arrange()
            .test_method(returns(Err(TestError::StubbedError)).times(n))
            .go();

        for _ in 0..n {
            assert_eq!(stub.test_method(&[]), Err(TestError::StubbedError));
        }

        assert_panics!(
            {
                let mut stub = stub;
                stub.test_method(&[]).ok();
            },
            includes("No expected result available")
        );
    }

    #[test]
    fn should_sequence_multiple_once_results() {
        let mut stub = TestStub::arrange()
            .test_method(
                returns(Err(TestError::StubbedError))
                    .once()
                    .returns(Ok(()))
                    .once(),
            )
            .go();

        // The first time should return the first result
        assert_eq!(
            stub.test_method(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );

        // The second time the second result
        assert_eq!(stub.test_method(&[8u8, 7u8, 6u8]), Ok(()));

        // Panic after that
        assert_panics!(
            {
                let mut stub = stub;
                stub.test_method(&[]).ok();
            },
            includes("No expected result available")
        );
    }

    #[test]
    fn should_return_always_result_multiple_times() {
        let mut stub = TestStub::arrange()
            .test_method(returns(Err(TestError::StubbedError)).always())
            .go();

        for _ in [0..20].iter() {
            assert_eq!(
                stub.test_method(&[8u8, 7u8, 6u8]),
                Err(TestError::StubbedError)
            );
        }
    }

    #[test]
    fn should_return_always_result_after_once() {
        let mut stub = TestStub::arrange()
            .test_method(
                returns(Ok(()))
                    .once()
                    .returns(Err(TestError::StubbedError))
                    .always(),
            )
            .go();

        // First return the 'once' result
        assert_eq!(stub.test_method(&[8u8, 7u8, 6u8]), Ok(()));

        // And the 'always' result after that
        for _ in [0..20].iter() {
            assert_eq!(
                stub.test_method(&[8u8, 7u8, 6u8]),
                Err(TestError::StubbedError)
            );
        }
    }
}
