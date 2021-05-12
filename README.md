# Rust Space API Implementation

[![CircleCI](https://circleci.com/gh/spaceapi-community/spaceapi-server-rs/tree/master.svg?style=shield)](https://circleci.com/gh/spaceapi-community/spaceapi-server-rs/tree/master)
[![Crates.io Version](https://img.shields.io/crates/v/spaceapi-server.svg)](https://crates.io/crates/spaceapi-server)
[![Crates.io Downloads](https://img.shields.io/crates/d/spaceapi-server.svg)](https://crates.io/crates/spaceapi-server)

This is a library that allows an easy implementation of a
[SpaceAPI](https://spaceapi.io/) v0.13 or v14 server in Rust.

 * Crate Documentation: https://docs.rs/spaceapi-server/
 * Space API Reference: https://spaceapi.io/pages/docs.html


## Requirements

 * A Redis instance on the server


## Rust Version Requirements (MSRV)

This library generally tracks the latest stable Rust version but tries to
guarantee backwards compatibility with older stable versions as much as
possible. However, in many cases transitive dependencies make guaranteeing a
minimal supported Rust version impossible (see [this
discussion](https://users.rust-lang.org/t/rust-version-requirement-change-as-semver-breaking-or-not/20980/25)).


## Usage

Please take a look at [the docs](https://docs.rs/spaceapi-server/).


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
