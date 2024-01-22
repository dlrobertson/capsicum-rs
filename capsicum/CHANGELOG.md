# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased] - ReleaseDate

### Added

- Implemented `Send` and `Sync` for `CapChannel`.
  ([#66](https://github.com/dlrobertson/capsicum-rs/pull/66))

### Changed

- The `IoctlsBuilder` and `FcntlsBuilder` APIs have the following changes:
  * They are both now `Clone`.
  * Their `new` methods no longer take arguments.  The value formerly supplied
    as an argument must now be supplied by the `add` methods.
  * `IoctlsBuilder`'s methods now pass by move.
  * The `IoctlsBuilder::raw`, `FcntlsBuilder::raw`, `IoctlRights::new`, and
    `FcntlRights::new` methods are all deprecated.
  ([#71](https://github.com/dlrobertson/capsicum-rs/pull/71))

### Removed

- `util::Directory` is deprecated.  Use the `cap-std` crate instead.
  ([#74](https://github.com/dlrobertson/capsicum-rs/pull/74))

## [0.3.0] - 2023-09-21

### Changed

- The `capsicum::casper::Service` trait's `SERVICE_NAME` field must now be
  defined as a `&'static CStr`.  The [cstr](https://crates.io/crates/cstr)
  crate can help.
  ([#49](https://github.com/dlrobertson/capsicum-rs/pull/49))

- `capsicum::get_mode` now returns a `bool`.
  ([#51](https://github.com/dlrobertson/capsicum-rs/pull/51))

### Fixed

- Fixed cross-building the documentation.
  ([#42](https://github.com/dlrobertson/capsicum-rs/pull/42))
