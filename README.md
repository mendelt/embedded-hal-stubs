# Experimental Rust lib for mock and stub helpers for the embedded HAL library
This is an experimental library to see if it is possible to write a test helpers for the Embedded
HAL that allow for a more AAA (Arrange, Act, Assert) style of testing, while giving the user the
freedom to choose to use the helpers either as Stubs or as Mocks.

The main goals of this project are;
- [ ] Support a style of testing where you can first Arrange the behavior of test dependencies, Act out the test behaviors and afterwards Assert if the code under test performed the right actions.
- [ ] Support assertions that test if the test helpers provided by this library were called in the right order, even when different test helpers were used.
- [ ] A fluent interface for specifying behaviors and assertions.
- [ ] Support all of embedded-hal 1.0

For now this is an experimental library for my own use. If you're interested in a complete library
for writing actual tests I would recommend the [Embedded HAL Mock](https://github.com/dbrgn/embedded-hal-mock/)
library which is way more feature complete, polished and stable.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
