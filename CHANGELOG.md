# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### Unreleased

- [changed] Update redis to 0.20 and r2d2\_redis to 0.14 (#105, #106)
- [changed] Update quick-error to 2.0 (#106)
- [changed] Update spaceapi to 0.8 to support SpaceAPI v14 (#107)

### v0.5.0 (2020-04-21)

- [changed] Replace last use of `rustc_serialize` with `serde` (#86)
- [changed] Move to Rust 2018 edition (#88)
- [changed] Do not guarantee a fixed MSRV anymore (#99)
- [changed] Bump dependencies, including spaceapi which now uses an enum for
  the `issue_report_channels` see the [spaceapi changelog] for more details
  (#92, #97)
- [changed] Bump redis version. This changes some inner types in the
  `SpaceapiServerError` (#96, #100)
- [added] Add usage and API documentation (#94)
- [changed] Some internal types that were never meant to be
  used outside the crate are now not public anymore (#94)
- [added] Allow to set custom redis pool for custom options (#90)
- [fixed] Panic in `StateFromPeopleNowPresent` (#104)

### v0.4.1 (2018-06-28)

- [changed] Bump minimal required Rust version (#76, #78)
- [changed] Fixed various links in metadata (#75, #79)

### v0.4.0 (2017-06-19)

- [changed] Update spaceapi (0.5) and thus use serde for serialization (#58, #66)
- [changed] Update iron dependency to 0.5 (#62)
- [changed] Add `SpaceApiServerBuilder` and remove `SpaceApiServer::new()` (#68)
- [changed] Add version info statically and remove `modifiers::LibraryVersions` (#70)

### v0.3.1 (2016-09-02)

- [fixed] Removed star version dependency in `Cargo.toml` preventing publication (#55)

### v0.3.0 (2016-09-01)

- [added] Top level `get_version` function (#4)
- [added] Add `LibraryVersions` status modifier (#4)
- [changed] Introduced internal Redis connection pooling (#43)
- [changed] Updated spaceapi dependency to 0.3 (#46)
- [changed] Updated hyper dependency to 0.9 (#51)
- [changed] Updated iron dependency to 0.4 (#51)

### v0.2.0 (2016-03-14)

- [changed] Removed datastore module, use Redis directly (#10)
- [changed] `SpaceapiServer::new()` now returns a Result (#16)
- [changed] `SpaceapiServer.serve()` now returns a `HttpResult<Listening>` (#16)
- [changed] Use `ToSocketAddrs` instead of `IPv4addr` and port in `SpaceapiServer::new()` (#22)
- [added] Support status modifiers (#8)
- [added] Add simple examples (#30)

### v0.1.1 (2015-11-16)

- [fixed] Fixed metadata in `Cargo.toml`

### v0.1.0 (2015-11-14)

- First crates.io release


[spaceapi changelog]: https://github.com/spaceapi-community/spaceapi-rs/blob/master/CHANGELOG.md
