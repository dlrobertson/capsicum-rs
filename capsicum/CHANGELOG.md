# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

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
