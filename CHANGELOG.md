# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.7.0 - 2024-06-03

### Changed

- `vi::telex` & `vi::vni` are deprecated & will be removed in the next release. Users are recommended to use `vi::methods` instead.
- `vi::telex::transform_buffer` & `vi::vni::transform_buffer` are deprecated. Users are recommended to use `vi::transform_buffer` instead.

### Added

- `vi::methods` module containing method definition & transforming functions.