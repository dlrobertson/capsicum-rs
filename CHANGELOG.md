# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased] - ReleaseDate

### Changed

- The `capsicum::casper::Service` trait's `SERVICE_NAME` field must now be
  defined as a `&'static CStr`.  The [cstr](https://crates.io/crates/cstr)
  crate can help.
  ([#49](https://github.com/dlrobertson/capsicum-rs/pull/49))

### Fixed

- Fixed cross-building the documentation.
  ([#42](https://github.com/dlrobertson/capsicum-rs/pull/42))
