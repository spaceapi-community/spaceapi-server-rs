# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.


### v0.3.0 UNRELEASED

- [changed] Introduced internal Redis connection pooling (#42)
- [changed] Updated spaceapi dependency to 0.3 (#46)

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
