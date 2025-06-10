# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **BREAKING**: Updated Rust edition from 2018 to 2021
- Updated all dependencies to latest stable versions:
  - nom: 7.1.3 → 8.0.0 (with breaking changes handled)
  - criterion: 0.3.6 → 0.6.0 (with breaking changes handled)
  - rustyline: 14.0.0 → 16.0.0 (with breaking changes handled)
  - insta: 1.39.0 → 1.43.1
  - phf: 0.11.2 → 0.11.3
  - log: 0.4.21 → 0.4.27
  - serde: 1.0.202 → 1.0.219
- Modernized examples by removing unnecessary `extern crate` statements
- Updated benchmarks to use `std::hint::black_box` instead of deprecated `criterion::black_box`

### Security

- **FIXED**: Removed unmaintained dependencies (atty, serde_cbor) that had security advisories
- **FIXED**: Resolved RUSTSEC-2024-0375 (atty unmaintained)
- **FIXED**: Resolved RUSTSEC-2021-0127 (serde_cbor unmaintained)
- **FIXED**: Resolved RUSTSEC-2021-0145 (atty potential unaligned read)

### Improved

- Enhanced compatibility with latest Rust toolchain (1.86.0)
- Improved code quality and maintainability
- Updated documentation and examples to reflect modern Rust practices

### Performance

- **MAJOR**: 10-18% performance improvements across all benchmarks
- Implemented SmallVec for letter modifications (reduces heap allocations)
- Added `const fn` for vowel checking (compile-time optimization)
- Applied `#[inline]` attributes to performance-critical functions
- Made enums `Copy` to eliminate unnecessary clones
- Enhanced trait derivations for better performance

### API Enhancements

- Added `#[must_use]` attributes to prevent ignoring important return values
- Implemented `From`/`Into` traits for better ergonomics
- Enhanced error types with comprehensive documentation
- Added `Syllable::new()` constructor for better API
- Re-exported `Syllable` at crate root for easier access
- Improved trait derivations: `Clone`, `Debug`, `PartialEq`, `Eq`, `Copy`, `Hash`

## 0.7.0 - 2024-06-03

### Changed

- `vi::telex` & `vi::vni` are deprecated & will be removed in the next release. Users are recommended to use `vi::methods` instead.
- `vi::telex::transform_buffer` & `vi::vni::transform_buffer` are deprecated. Users are recommended to use `vi::transform_buffer` instead.

### Added

- `vi::methods` module containing method definition & transforming functions.