#[derive(Clone)]
pub enum Return<T> {
    Once(T),
    Always(T),
}

pub fn returns<T>(value: T) -> ReturnsBuilder<T> {
    ReturnsBuilder {
        previous: Returns::default(),
        new_result: value
    }
}

impl<T> ReturnsBuilder<T> {
    pub fn once(mut self) -> Returns<T> {
        self.previous.return_values.push(Return::Once(self.new_result));
        self.previous
    }

    pub fn always(mut self) -> Returns<T> {
        self.previous.return_values.push(Return::Always(self.new_result));
        self.previous
    }
}

pub struct ReturnsBuilder<T> {
    pub(self) previous: Returns<T>,
    pub(self) new_result: T
}

/// Stores the set of return values for a stubbed method where T is the return type and implements
/// the fluent interface for specifying the return values
pub struct Returns<T> {
    pub(self) return_values: Vec<Return<T>>,
}

impl<T> Returns<T> {
    pub fn returns(self, value: T) -> ReturnsBuilder<T> {
        ReturnsBuilder {
            previous: self,
            new_result: value
        }
    }
}

impl<T: Clone> Returns<T> {
    pub fn get_by_params(&mut self) -> T {
        match self.return_values.split_first() {
            Some((Return::Always(value), _)) => value.clone(),
            Some((Return::Once(value), tail)) => {
                let result = value.clone();
                self.return_values = tail.to_vec();
                result
            }
            _ => panic!("No expected result available"),
        }
    }
}

impl<T> Default for Returns<T> {
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
            self.on_test_method.get_by_params()
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
    #[should_panic(expected = "No expected result available")]
    fn should_return_once_result_only_once() {
        let mut stub = TestStub::arrange()
            .test_method(returns(Err(TestError::StubbedError)).once())
            .go();

        assert_eq!(
            stub.test_method(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );

        // This should panic the second time it's called
        stub.test_method(&[8u8, 7u8, 6u8]).ok();
    }

    #[test]
    #[should_panic(expected = "No expected result available")]
    fn should_sequence_multiple_once_results() {
        let mut stub = TestStub::arrange()
            .test_method(
                returns(Err(TestError::StubbedError)).once()
                .returns(Ok(())).once())
            .go();

        // The first time should return the first result
        assert_eq!(
            stub.test_method(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );

        // The second time the second result
        assert_eq!(stub.test_method(&[8u8, 7u8, 6u8]), Ok(()));

        // Panic after that
        stub.test_method(&[8u8, 7u8, 6u8]).ok();
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
            .test_method(returns(Ok(())).once().returns(Err(TestError::StubbedError)).always())
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
