# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased] - ReleaseDate

### Added

- Added `Right::RenameatTarget`.
  ([#85](https://github.com/dlrobertson/capsicum-rs/pull/85))

- Implemented `Send` and `Sync` for `CapChannel`.
  ([#66](https://github.com/dlrobertson/capsicum-rs/pull/66))

### Changed

- Renamed `Right::Renameat` to `Right::RenameatSource`.
  ([#85](https://github.com/dlrobertson/capsicum-rs/pull/85))

- The `IoctlsBuilder`, `FcntlsBuilder`, and `RightsBuilder` structs have been
  heavily changed:
  * `FcntlsBuilder` and `RightsBuilder` are deprecated.  You may now use
    `FcntlRights` and `FileRights` directly.
  * Unlike the builder structs old `new` methods, `FcntlRights`'s and
    `FileRights`'s new methods take no arguments.  The values formerly supplied
    as arguments to `new` must now be provided to the `allow` methods.
  * `FcntlRights` and `FileRights` are now `Copy` and `Clone`.
  * `IoctlsBuilder` is now `Clone`.
  * `IoctlsBuilder`'s methods now pass by move.
  * `IoctlsBuilder::raw` and `IoctlRights::new` methods are deprecated.
  * `IoctlsBuilder`'s `add` and `remove` methods have been renamed to
    `allow`/`deny`, respectively.  Similarly named methods are added to
    `FileRights` and `FcntlRights`.  `FileRights`'s `set`/`clear` methods are
    similarly deprecated.
  ([#71](https://github.com/dlrobertson/capsicum-rs/pull/71))
  ([#88](https://github.com/dlrobertson/capsicum-rs/pull/88))

- `Rights` are now `Clone`, `Copy`, and `Eq`.
  ([#80](https://github.com/dlrobertson/capsicum-rs/pull/80))

### Fixed

- Fixed a crash that could happen within the C library, triggered by combining
  certain `Rights` values.
  ([#80](https://github.com/dlrobertson/capsicum-rs/pull/80))

### Removed

- `RightsBuilder::raw` is removed.
  ([#80](https://github.com/dlrobertson/capsicum-rs/pull/80))

- `FileRights::is_valid` is deprecated, because it is no longer useful without
  the new argument-less `FileRights::new`.
  ([#81](https://github.com/dlrobertson/capsicum-rs/pull/81))

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
