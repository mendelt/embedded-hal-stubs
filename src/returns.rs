#[derive(Clone)]
pub enum Return<T> {
    Once(T),
    Always(T),
}

/// Stores the set of return values for a stubbed method where T is the return type and implements
/// the fluent interface for specifying the return values
pub struct Returns<T> {
    return_values: Vec<Return<T>>,
}

pub fn returns<T>() -> Returns<T> {
    Returns::default()
}

impl<T> Returns<T> {
    pub fn once(mut self, value: T) -> Self {
        self.return_values.push(Return::Once(value));
        self
    }

    pub fn always(mut self, value: T) -> Self {
        self.return_values.push(Return::Always(value));
        self
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
                on_test_method: returns().always(Ok(())),
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
            .test_method(returns().once(Err(TestError::StubbedError)))
            .go();

        assert_eq!(
            stub.test_method(&[8u8, 7u8, 6u8]),
            Err(TestError::StubbedError)
        );
    }
}
