# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-01-24

### Added

- Integrated `rustfmt` for automatic code formatting.

  - Added `rustfmt.toml` configuration file to the repository.
  - Added a `rustfmt` check to the GitHub Actions CI workflow.
  - This ensures consistent code style across the project and helps maintain code readability.

- Implemented `SentenceBatcher`, a new batching strategy that splits text based on complete sentences, respecting a minimum batch size using punctuation delimiters.
- Added comprehensive unit tests for `SentenceBatcher`.
- Updated `README.md` to include `SentenceBatcher` usage and examples.
- Updated crate documentation to include `SentenceBatcher`.

## [0.2.0] - 2025-01-23

### Added

- Implemented `SentenceBatcher`, a new batching strategy that splits text based on complete sentences, respecting a minimum batch size using punctuation delimiters.
- Added comprehensive unit tests for `SentenceBatcher`.
- Updated `README.md` to include `SentenceBatcher` usage and examples.
- Updated crate documentation to include `SentenceBatcher`.
- Add Github Actions CI/CD

## [0.1.0] - 2025-01-23

### Added

- Implemented `CharCountBatcher`, a batching strategy that splits text based on a fixed character count.
- Created basic project structure with `strategies` module.
- Added unit tests for `CharCountBatcher`.
- Created `README.md` with basic usage instructions.
- Set up initial project documentation using `cargo doc`.
