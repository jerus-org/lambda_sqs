# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- Dependencies: update dependency toolkit to v2.5.1(pr [#88])

## [0.2.26] - 2025-03-15

### Security

- Dependencies: update rust crate serde to 1.0.219(pr [#86])
- Dependencies: update rust crate tokio to 1.44.1(pr [#87])

## [0.2.25] - 2025-03-08

### Security

- Dependencies: update rust crate serde_json to 1.0.140(pr [#84])
- Dependencies: update dependency toolkit to v2.1.0(pr [#85])

## [0.2.24] - 2025-03-01

### Security

- Dependencies: update dependency toolkit to v2.0.13(pr [#83])

## [0.2.23] - 2025-02-22

### Security

- Dependencies: update serde packages(pr [#82])

## [0.2.22] - 2025-02-01

### Security

- Dependencies: update rust crate serde_json to 1.0.138(pr [#81])

## [0.2.21] - 2025-01-25

### Changed

- ci(circleci)-update configuration with new parameter(pr [#75])
- chore(config)-migrate renovate config(pr [#78])
- 👷 ci(circleci): update CI workflow with version management(pr [#79])
- 👷 ci(circleci): fix incorrect workflow indentation(pr [#80])

### Security

- Dependencies: update dependency toolkit to v2(pr [#74])
- Dependencies: update dependency toolkit to v2.0.4(pr [#76])
- Dependencies: update rust crate serde_json to 1.0.137(pr [#77])

## [0.2.20] - 2025-01-11

### Security

- Dependencies: update rust crate serde_json to 1.0.135(pr [#72])
- Dependencies: update rust crate tokio to 1.43.0(pr [#73])

## [0.2.19] - 2025-01-04

### Security

- Dependencies: update rust crate serde to 1.0.217(pr [#71])

## [0.2.18] - 2024-12-28

### Security

- Dependencies: update rust crate serde_json to 1.0.134(pr [#69])
- Dependencies: update dependency toolkit to v1.23.0(pr [#70])

## [0.2.17] - 2024-12-21

### Security

- Dependencies: update rust crate serde to 1.0.216(pr [#67])
- Dependencies: update dependency toolkit to v1.20.2(pr [#68])

## [0.2.16] - 2024-12-07

### Security

- Dependencies: update rust crate tokio to 1.42.0(pr [#66])

## [0.2.15] - 2024-11-23

### Security

- Dependencies: update rust crate serde_json to 1.0.133(pr [#65])

## [0.2.14] - 2024-11-16

### Security

- Dependencies: update rust crate serde to 1.0.215(pr [#64])

## [0.2.13] - 2024-11-09

### Changed

- ci(circleci)-update toolkit orb and add code coverage step to pipeline(pr [#63])

### Security

- Dependencies: update dependency toolkit to v1.16.1(pr [#60])
- Dependencies: update dependency toolkit to v1.18.0(pr [#62])
- Dependencies: update rust crate tokio to 1.41.1(pr [#61])

## [0.2.12] - 2024-11-02

### Security

- Dependencies: update dependency toolkit to v1.14.0(pr [#57])
- Dependencies: update rust crate serde to 1.0.214(pr [#58])
- Dependencies: update dependency toolkit to v1.15.0(pr [#59])

## [0.2.11] - 2024-10-26

### Security

- Dependencies: update rust crate serde_json to 1.0.131(pr [#52])
- Dependencies: update rust crate serde_json to 1.0.132(pr [#53])
- Dependencies: update rust crate serde to 1.0.211(pr [#54])
- Dependencies: update rust crate tokio to 1.41.0(pr [#55])
- Dependencies: update rust crate serde to 1.0.213(pr [#56])

## [0.2.10] - 2024-10-19

### Security

- Dependencies: update rust crate serde_json to 1.0.129(pr [#51])

## [0.2.9] - 2024-09-28

### Fixed

- update renovate config to enable jerus-org/circleci-toolkit and add sourceUrl(pr [#50])

## [0.2.8] - 2024-09-14

### Changed

- chore-update CircleCI config and renovate settings(pr [#47])

### Security

- Dependencies: update serde packages(pr [#48])
- Dependencies: update rust crate tokio to 1.40.0(pr [#49])

## [0.2.7] - 2024-08-03

### Changed

- ci-add bot-check context to workflows(pr [#46])

## [0.2.6] - 2024-07-27

### Changed

- ci-update jerus-org/circleci-toolkit orb version from 0.22.0 to 0.24.1(pr [#45])

### Security

- Dependencies: update rust crate lambda_runtime to 0.13.0(pr [#44])

## [0.2.5] - 2024-07-13

### Changed

- Migrate to circleci
- Enable missing doc check feature on nightly
- Minrust and typo
- chore-migrate to jerus-org(pr [#40])
- ci-upgrade jerus-org/circleci-toolkit from 0.20.0 to 0.21.0(pr [#41])
- ci-adopt all features of the toolkit and remove custom commands and jobs(pr [#42])
- ci-update jerus-org/circleci-toolkit version from 0.21.0 to 0.22.0(pr [#43])

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
[#41]: https://github.com/jerus-org/lambda_sqs/pull/41
[#42]: https://github.com/jerus-org/lambda_sqs/pull/42
[#43]: https://github.com/jerus-org/lambda_sqs/pull/43
[#44]: https://github.com/jerus-org/lambda_sqs/pull/44
[#45]: https://github.com/jerus-org/lambda_sqs/pull/45
[#46]: https://github.com/jerus-org/lambda_sqs/pull/46
[#47]: https://github.com/jerus-org/lambda_sqs/pull/47
[#48]: https://github.com/jerus-org/lambda_sqs/pull/48
[#49]: https://github.com/jerus-org/lambda_sqs/pull/49
[#50]: https://github.com/jerus-org/lambda_sqs/pull/50
[#51]: https://github.com/jerus-org/lambda_sqs/pull/51
[#52]: https://github.com/jerus-org/lambda_sqs/pull/52
[#53]: https://github.com/jerus-org/lambda_sqs/pull/53
[#54]: https://github.com/jerus-org/lambda_sqs/pull/54
[#55]: https://github.com/jerus-org/lambda_sqs/pull/55
[#56]: https://github.com/jerus-org/lambda_sqs/pull/56
[#57]: https://github.com/jerus-org/lambda_sqs/pull/57
[#58]: https://github.com/jerus-org/lambda_sqs/pull/58
[#59]: https://github.com/jerus-org/lambda_sqs/pull/59
[#60]: https://github.com/jerus-org/lambda_sqs/pull/60
[#62]: https://github.com/jerus-org/lambda_sqs/pull/62
[#63]: https://github.com/jerus-org/lambda_sqs/pull/63
[#61]: https://github.com/jerus-org/lambda_sqs/pull/61
[#64]: https://github.com/jerus-org/lambda_sqs/pull/64
[#65]: https://github.com/jerus-org/lambda_sqs/pull/65
[#66]: https://github.com/jerus-org/lambda_sqs/pull/66
[#67]: https://github.com/jerus-org/lambda_sqs/pull/67
[#68]: https://github.com/jerus-org/lambda_sqs/pull/68
[#69]: https://github.com/jerus-org/lambda_sqs/pull/69
[#70]: https://github.com/jerus-org/lambda_sqs/pull/70
[#71]: https://github.com/jerus-org/lambda_sqs/pull/71
[#72]: https://github.com/jerus-org/lambda_sqs/pull/72
[#73]: https://github.com/jerus-org/lambda_sqs/pull/73
[#74]: https://github.com/jerus-org/lambda_sqs/pull/74
[#75]: https://github.com/jerus-org/lambda_sqs/pull/75
[#76]: https://github.com/jerus-org/lambda_sqs/pull/76
[#77]: https://github.com/jerus-org/lambda_sqs/pull/77
[#78]: https://github.com/jerus-org/lambda_sqs/pull/78
[#79]: https://github.com/jerus-org/lambda_sqs/pull/79
[#80]: https://github.com/jerus-org/lambda_sqs/pull/80
[#81]: https://github.com/jerus-org/lambda_sqs/pull/81
[#82]: https://github.com/jerus-org/lambda_sqs/pull/82
[#83]: https://github.com/jerus-org/lambda_sqs/pull/83
[#84]: https://github.com/jerus-org/lambda_sqs/pull/84
[#85]: https://github.com/jerus-org/lambda_sqs/pull/85
[#86]: https://github.com/jerus-org/lambda_sqs/pull/86
[#87]: https://github.com/jerus-org/lambda_sqs/pull/87
[#88]: https://github.com/jerus-org/lambda_sqs/pull/88
[Unreleased]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.26...HEAD
[0.2.26]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.25...v0.2.26
[0.2.25]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.24...v0.2.25
[0.2.24]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.23...v0.2.24
[0.2.23]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.22...v0.2.23
[0.2.22]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.21...v0.2.22
[0.2.21]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.20...v0.2.21
[0.2.20]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.19...v0.2.20
[0.2.19]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.18...v0.2.19
[0.2.18]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.17...v0.2.18
[0.2.17]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.16...v0.2.17
[0.2.16]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.15...v0.2.16
[0.2.15]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.14...v0.2.15
[0.2.14]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.13...v0.2.14
[0.2.13]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.12...v0.2.13
[0.2.12]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.11...v0.2.12
[0.2.11]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.10...v0.2.11
[0.2.10]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/jerus-org/lambda_sqs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/jerus-org/lambda_sqs/releases/tag/v0.2.0
