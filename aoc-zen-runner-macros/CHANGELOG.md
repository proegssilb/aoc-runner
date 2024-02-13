# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6](https://github.com/proegssilb/aoc-zen-runner/compare/aoc-zen-runner-macros-v0.1.5...aoc-zen-runner-macros-v0.1.6) - 2024-02-13

### Fixed
- *(deps)* update rust crate proc-macro2 to 1.0.78

## [0.1.5](https://github.com/proegssilb/aoc-zen-runner/compare/aoc-zen-runner-macros-v0.1.4...aoc-zen-runner-macros-v0.1.5) - 2024-01-08

### Fixed
- *(deps)* update rust crate syn to 2.0.48 ([#184](https://github.com/proegssilb/aoc-zen-runner/pull/184))
- *(deps)* update rust crate anyhow to 1.0.79 ([#186](https://github.com/proegssilb/aoc-zen-runner/pull/186))
- *(deps)* update rust crate thiserror to 1.0.56 ([#185](https://github.com/proegssilb/aoc-zen-runner/pull/185))
- *(deps)* update rust crate proc-macro2 to 1.0.76 ([#189](https://github.com/proegssilb/aoc-zen-runner/pull/189))
- *(deps)* update rust crate quote to 1.0.35 ([#190](https://github.com/proegssilb/aoc-zen-runner/pull/190))

## [0.1.4](https://github.com/proegssilb/aoc-zen-runner/compare/aoc-zen-runner-macros-v0.1.3...aoc-zen-runner-macros-v0.1.4) - 2023-12-22

### Fixed
- *(deps)* update rust crate anyhow to 1.0.76 ([#181](https://github.com/proegssilb/aoc-zen-runner/pull/181))
- *(deps)* update rust crate syn to 2.0.42 ([#180](https://github.com/proegssilb/aoc-zen-runner/pull/180))
- *(deps)* update rust crate proc-macro2 to 1.0.71 ([#182](https://github.com/proegssilb/aoc-zen-runner/pull/182))
- *(deps)* update rust crate thiserror to 1.0.51 ([#176](https://github.com/proegssilb/aoc-zen-runner/pull/176))
- *(deps)* update rust crate syn to 2.0.41 ([#174](https://github.com/proegssilb/aoc-zen-runner/pull/174))
- *(deps)* update rust crate syn to 2.0.40 ([#172](https://github.com/proegssilb/aoc-zen-runner/pull/172))

## [0.1.3](https://github.com/proegssilb/aoc-zen-runner/compare/aoc-zen-runner-macros-v0.1.2...aoc-zen-runner-macros-v0.1.3) - 2023-12-08

### Fixed
- Expanded error when cargo metadata fails.

## [0.1.2](https://github.com/proegssilb/aoc-zen-runner/compare/aoc-zen-runner-macros-v0.1.1...aoc-zen-runner-macros-v0.1.2) - 2023-12-04

### Fixed
- Prevent conflicting lints in test const names

### Other
- Prevent contradictory lints when using `aoc_case` ([#165](https://github.com/proegssilb/aoc-zen-runner/pull/165))
- Move `aoc` closer to other macros for readability

## [0.1.1](https://github.com/proegssilb/aoc-zen-runner/compare/aoc-zen-runner-macros-v0.1.0...aoc-zen-runner-macros-v0.1.1) - 2023-12-02

### Fixed
- Make failing tests show failing solution ([#159](https://github.com/proegssilb/aoc-zen-runner/pull/159))
- Easier copy/paste of output via more newlines

### Other
- release ([#142](https://github.com/proegssilb/aoc-zen-runner/pull/142))

## [0.1.0](https://github.com/proegssilb/aoc-zen-runner/releases/tag/aoc-zen-runner-macros-v0.1.0) - 2023-11-30

### Other
- Bugfix `readme` in Cargo.toml files
- Cleanup Cargo.toml files for packages.
- Implement downloading input, fix bugs in running code ([#135](https://github.com/proegssilb/aoc-zen-runner/pull/135))
- Implement v1 of "run" subcommand ([#131](https://github.com/proegssilb/aoc-zen-runner/pull/131))
- Update Rust crate proc-macro2 to 1.0.70 ([#120](https://github.com/proegssilb/aoc-zen-runner/pull/120))
- Update Rust crate criterion to 0.5.1 ([#116](https://github.com/proegssilb/aoc-zen-runner/pull/116))
- Convert repository to use a multi-year workspace as the sample code (implement [#103](https://github.com/proegssilb/aoc-zen-runner/pull/103)) ([#110](https://github.com/proegssilb/aoc-zen-runner/pull/110))
