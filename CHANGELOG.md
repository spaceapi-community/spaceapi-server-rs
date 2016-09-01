# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.


### v0.X.X UNRELEASED

- ...

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
