# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased] - ReleaseDate

### Added

- Added `Right::LinkatSource`.
  ([#86](https://github.com/dlrobertson/capsicum-rs/pull/86))

- Added `Right::RenameatTarget`.
  ([#85](https://github.com/dlrobertson/capsicum-rs/pull/85))

- Implemented `Send` and `Sync` for `CapChannel`.
  ([#66](https://github.com/dlrobertson/capsicum-rs/pull/66))

### Changed

- Renamed `Right::Linkat` to `Right::LinkatTarget`.
  ([#86](https://github.com/dlrobertson/capsicum-rs/pull/86))

- Renamed `Right::Renameat` to `Right::RenameatSource`.
  ([#85](https://github.com/dlrobertson/capsicum-rs/pull/85))

- The `IoctlsBuilder` and `FcntlsBuilder` APIs have the following changes:
  * They are both now `Clone`.
  * Their `new` methods no longer take arguments.  The value formerly supplied
    as an argument must now be supplied by the `add` methods.
  * `IoctlsBuilder`'s methods now pass by move.
  * The `IoctlsBuilder::raw`, `FcntlsBuilder::raw`, `IoctlRights::new`, and
    `FcntlRights::new` methods are all deprecated.
  * Their `add` and `remove` methods have been renamed to `allow`/`deny`,
    respectively.
  ([#71](https://github.com/dlrobertson/capsicum-rs/pull/71))

- The `RightsBuilder`'s `add`/`remove` methods have been renamed to
  `allow`/`deny`, respectively.  Also, `FileRights`'s `set`/`clear` methods
  have been renamed to `allow`/`deny`.
  ([#71](https://github.com/dlrobertson/capsicum-rs/pull/71))

- More changes to `RightsBuilder`.
  * The `new` method no longer takes an argument.
  * `Rights` are now `Clone`, `Copy`, and `Eq`.
  * `RightsBuilder` is now `Clone`.
  ([#80](https://github.com/dlrobertson/capsicum-rs/pull/80))

### Fixed

- Fixed a crash that could happen within the C library, triggered by combining
  certain `Rights` values.
  ([#80](https://github.com/dlrobertson/capsicum-rs/pull/80))

### Removed

- `RightsBuilder::raw` is removed and `FileRights::new` is deprecated.
  ([#80](https://github.com/dlrobertson/capsicum-rs/pull/80))

- `FileRights::is_valid` is deprecated, because it is no longer useful without
  `FileRights::new`.
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
