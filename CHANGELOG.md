# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Migrate to circleci
- Enable missing doc check feature on nightly
- Minrust and typo
- chore-migrate to jerus-org(pr [#40])

### Security

- Dependencies: update rust crate lambda_runtime to 0.12.0(pr [#39])

## [0.2.4] - 2023-01-22

### Fixed

- Update rust crate lambda_runtime to 0.7.3

## [0.2.3] - 2022-05-14

### Changed

- Update dependencies

## [0.2.2] - 2022-05-14

### Added

- Return a vec of messages

### Changed

- Update dependencies

## [0.2.1] - 2022-01-24

### Changed

- FIX: error in repository string
- Update dependences

## [0.2.0] - 2021-09-22

### Added

- Collect batch of messages
- Convert batch of messages to vec of user provider struct

[#39]: https://github.com/jerusdp/lambda_sqs/pull/39
[#40]: https://github.com/jerus-org/lambda_sqs/pull/40
[Unreleased]: https://github.com/jerus-org/lambda_sqs/compare/0.2.4...HEAD
[0.2.4]: https://github.com/jerus-org/lambda_sqs/compare/0.2.3...0.2.4
[0.2.3]: https://github.com/jerus-org/lambda_sqs/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/jerus-org/lambda_sqs/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/jerus-org/lambda_sqs/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/jerus-org/lambda_sqs/releases/tag/0.2.0
