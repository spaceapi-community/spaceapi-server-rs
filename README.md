# Rust Space API Implementation

[![Build Status](https://img.shields.io/travis/coredump-ch/spaceapi-server-rs/master.svg)](https://travis-ci.org/coredump-ch/spaceapi-server-rs)
[![Crates.io Version](https://img.shields.io/crates/v/spaceapi-server.svg)](https://crates.io/crates/spaceapi-server)
[![Crates.io Downloads](https://img.shields.io/crates/d/spaceapi-server.svg)](https://crates.io/crates/spaceapi-server)

This is a library that allows an easy implementation of a
[SpaceAPI](https://spaceapi.net/) v0.13 server in Rust.

- Crate Documentation: https://coredump-ch.github.io/rust-docs/spaceapi-server/spaceapi_server/index.html
- Space API Reference: https://spaceapi.net/documentation


## Usage

TODO


## Datastores

### Redis

To use the redis storage start the redis server:

    $ redis-server

(...or start it using your favorite init daemon.)

You can access the database with the `redis-cli` tool:

    % redis-cli
    127.0.0.1:6379> SET people_present 1
    OK
    127.0.0.1:6379> GET people_present
    "1"
    127.0.0.1:6379> KEYS *
    1) "people_present"

The values depend on your sensors configuration.


## Docs

You can build docs with `make docs`. Find them in the `target/doc/` directory.
