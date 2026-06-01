# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace scaffold: streamparse-core, streamparse-formats, streamparse-cli.

## [0.1.0] - TBD

### Added
- Push-based incremental parser core processing input in bounded memory.
- Zero-copy tokenization with a documented copy boundary for split tokens.
- Pluggable Format trait with NDJSON and delimited implementations.
- CLI to stream-parse files or stdin and emit events or statistics.

[Unreleased]: https://github.com/leisurelyleon/streamparse/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/leisurelyleon/streamparse/releases/tag/v0.1.0
