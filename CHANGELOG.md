# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added

### Changed

### Removed

## [0.11.6] - 2022-06-22
### Changed
 - Don't quote generic font families in CSS. [#252](https://github.com/jonhoo/inferno/pull/252)

## [0.11.5] - 2022-06-18
### Changed
 - Moved from `lazy_static` to `once_cell`. [#249](https://github.com/jonhoo/inferno/pull/249)

## [0.11.4] - 2022-05-28
### Changed
 - Fix for Firefox canvas heigh unpredictability. [#242](https://github.com/jonhoo/inferno/pull/242)
 - Update `quick-xml` to 0.23.

## [0.11.3] - 2022-05-08
### Changed
 - Fix handling of blank and comment lines in collapsed files. [#239](https://github.com/jonhoo/inferno/pull/239)

## [0.11.2] - 2022-04-24
### Added
 - Add collapser for Visual Studio profiler. [#218](https://github.com/jonhoo/inferno/pull/218)

## [0.11.1] - 2022-03-18
### Added
 - Highlight external NodeJS dependencies. [#234](https://github.com/jonhoo/inferno/pull/234)

## [0.11.0] - 2022-03-03
### Changed
 - Allowed `--skip-after` to appear multiple times. [#231](https://github.com/jonhoo/inferno/pull/231)
 - Replaces yanked `0.10.13` as the above change is backwards-incompatible.

## [0.10.12] - 2022-02-12
### Changed
 - Re-bump dashmap

## [0.10.11] - 2022-02-05
### Changed
 - Undo dashmap bump since 5.0.0 was yanked

## [0.10.10] - 2022-01-10
### Changed
 - Bumped various internal dependencies (notably dashmap and clap)

## [0.10.9] - 2021-12-18
### Added
 - `collapse-perf` gained `--skip-after` argument. [#224](https://github.com/jonhoo/inferno/pull/224)

### Changed
 - Frame skipping will also skip the process frame. [#224](https://github.com/jonhoo/inferno/pull/224)

## [0.10.8] - 2021-11-02
### Added
 - Setting for skipping frames prior to a known frame. [#222](https://github.com/jonhoo/inferno/pull/222)

## [0.10.7] - 2021-09-01
### Changed
 - Upgraded internal dependencies. [#216](https://github.com/jonhoo/inferno/pull/216)

## [0.10.6] - 2021-05-25
### Changed
 - Highlight Rust async/await as user code. [#213](https://github.com/jonhoo/inferno/pull/213)

## [0.10.5] - 2021-05-05
### Added
 - Add Rust language palette. [#211](https://github.com/jonhoo/inferno/pull/211)

## [0.10.4] - 2021-05-04
### Changed
 - Use number of samples instead of fractions for width/x calculations. [#210](https://github.com/jonhoo/inferno/pull/210)
 - Bumped `pretty_assertions` to 0.7
 - Bumped `rand` to 0.8

## [0.10.3] - 2021-01-08
### Added
 - Support jit annotations in collapse-perf for runtimes using the jitdump format. [#202](https://github.com/jonhoo/inferno/pull/202)

### Changed
 - Decreased default minimum width from 0.1% to 0.01%. [#204](https://github.com/jonhoo/inferno/pull/204)
 - Detect if STDOUT is a TTY and if it's not, use a `BufWriter` to avoid line buffering. [#206](https://github.com/jonhoo/inferno/pull/206)

## [0.10.2] - 2020-11-30
### Changed
 - Support for invalid utf8 data in collapse. [#196](https://github.com/jonhoo/inferno/pull/196)
 - Bumped `ahash` to 0.6
   - Caused a bump of minimum supported Rust version to 1.43
 - Bumped `crossbeam-channel` to 0.5
 - Bumped `crossbeam-utils` to 0.8
 - Bumped `env-logger` to 0.8
 - Bumped `quick-xml` to 0.20

## [0.10.1] - 2020-10-05
### Added
 - Support kernel annotations for versioned vmlinux and kernel modules in collapse-perf. [#182](https://github.com/jonhoo/inferno/pull/182)
 - Support of AsyncProfiler generated stack trace in java palette. [#183](https://github.com/jonhoo/inferno/pull/183)
 - `--deterministic` for deterministic colors without weighting. [#190](https://github.com/jonhoo/inferno/pull/190)

### Changed
 - Trimmed down a few unnecessary dependencies. [#188](https://github.com/jonhoo/inferno/pull/188)

## [0.10.0] - 2020-06-20
### Added

 - Flame chart mode. Flame charts put the passage of time on the x-axis instead of the alphabet. [#125](https://github.com/jonhoo/inferno/pull/125)
 - `cargo hack` to check that all features compile. [#181](https://github.com/jonhoo/inferno/pull/181)

### Changed

 - All `Options` are now marked as `#[non_exhaustive]` so that we can
   add options without making that a breaking change. This also makes
   feature-dependent fields (like `func_nameattr` on `flamegraph`) okay.
   Unfortunately, it also means that function record update syntax won't
   work any more (`Options { ..., ..Default::default() }`). See
   https://github.com/rust-lang/rust/issues/70564#issuecomment-647031324
   for details. [#181](https://github.com/jonhoo/inferno/pull/181)

## [0.9.9] - 2020-06-03
### Changed

- In icicle/inverted mode, the details bar is now at the top where it can actually be seen. [#177](https://github.com/jonhoo/inferno/pull/177)

## [0.9.8] - 2020-05-30
### Changed

 - Fixes a regression where anonymous namespaces would be pruned. [#175](https://github.com/jonhoo/inferno/pull/175)
 - Adds support for lambda expressions in curly braces in C++ function names. [#175](https://github.com/jonhoo/inferno/pull/175)

## [0.9.7] - 2020-05-29
### Changed

 - Stop pruning `()` expressions in template position. [#174](https://github.com/jonhoo/inferno/pull/174)

## [0.9.6] - 2020-05-07
### Added

 - Support for combined event/stack lines. [#168](https://github.com/jonhoo/inferno/pull/168)

### Changed

 - Fix crash on empty traces. [#168](https://github.com/jonhoo/inferno/pull/168)
 - Also parse last sample in file. [#168](https://github.com/jonhoo/inferno/pull/168)

## [0.9.5] - 2020-03-18
### Added
- Add a new color option, color diffusion, that makes wider frames redder. This visually draws the eye towards places that need optimization. [#165](https://github.com/jonhoo/inferno/pull/165) by [@itamarst](https://github.com/itamarst).

## [0.9.4] - 2020-02-02
### Changed
- Fix bug where subtitles would often be hidden ([#161](https://github.com/jonhoo/inferno/pull/161) by [@itamarst](https://github.com/itamarst))

## [0.9.3] - 2020-02-02
### Added
- Overly long frame lanes can be shortened either on the left or the right ([#157](https://github.com/jonhoo/inferno/pull/157) by [@itamarst](https://github.com/itamarst))

### Changed
- By default, overly long frame lines are now shortened on the left ([#157](https://github.com/jonhoo/inferno/pull/157) by [@itamarst](https://github.com/itamarst))

## [0.9.2] - 2020-01-30
### Changed
- Replace `chashmap` with `dashmap` ([#155](https://github.com/jonhoo/inferno/pull/155) by [@koushiro](https://github.com/koushiro))
- Replace `fnv` with `ahash` ([#155](https://github.com/jonhoo/inferno/pull/155) by [@koushiro](https://github.com/koushiro))
- Update some outdated dependencies ([#155](https://github.com/jonhoo/inferno/pull/155) by [@koushiro](https://github.com/koushiro))
- Upgrade MSRV to 1.40.0 (required by `dashmap`) ([#155](https://github.com/jonhoo/inferno/pull/155) by [@koushiro](https://github.com/koushiro))

## [0.9.1] - 2019-10-31
### Added
- Support for collapsing the CSV output of the VTune `amplxe-cl` tool ([#148](https://github.com/jonhoo/inferno/pull/148) by [@jasonrhansen](https://github.com/jasonrhansen)).

### Changed
- The `sample` collapser now returns errors where it used to just log them in places where it doesn't make sense to continue.

## [0.9.0] - 2019-09-11
### Changed
- Support for multi-threaded collapsing was moved behind the
  `multithreaded` feature flag which is on by default ([#146](https://github.com/jonhoo/inferno/pull/146)).
- The `structopt` dependency has been updated, which bumps the minimum
  supported Rust version to 1.36.0 ([#145](https://github.com/jonhoo/inferno/pull/145)).
- Support for nameattr files was moved behind the `nameattr` feature
  flag which is on by default ([#147](https://github.com/jonhoo/inferno/pull/147)).

### Removed
- The `demangle` option for collapsers; we instead rely on the sample
  generator to demangle names, and then just do some post-processing
  fixups for common issues ([#144](https://github.com/jonhoo/inferno/pull/144)).
- The dependency on `rand`. Reduces the footprint of the crate, and also
  makes the random color choices seeded by the same number each run
  ([#146](https://github.com/jonhoo/inferno/pull/146)).

## [0.8.0] - 2019-07-24
### Added
- Changelog
- Support for collapsing the output of the `sample` tool on macOS ([#133](https://github.com/jonhoo/inferno/pull/133) by [@jasonrhansen](https://github.com/jasonrhansen)).
- Multi-core stack collapsing for _major_ speedups ([#128](https://github.com/jonhoo/inferno/pull/128) by [@bcmyers](https://github.com/bcmyers)).
- Support for "fluid drawing" of the SVG ([#136](https://github.com/jonhoo/inferno/pull/136) by [@jasonrhansen](https://github.com/jasonrhansen)).
- Make zoom and search part of browser history ([#121](https://github.com/jonhoo/inferno/pull/121) from [@AnderEnder](https://github.com/AnderEnder)).
  This is a backport of https://github.com/brendangregg/FlameGraph/pull/198 by [@versable](https://github.com/versable).
- The `--demangle` flag to collapsers for "re-doing" broken symbol demangling from DTrace or perf ([#132](https://github.com/jonhoo/inferno/pull/132) by [@jasonrhansen](https://github.com/jasonrhansen)).
- Unit tests for semantic coloring.
  JavaScript: [#129](https://github.com/jonhoo/inferno/pull/129) by [@jordins](https://github.com/jordins)
  Java: [#131](https://github.com/jonhoo/inferno/pull/131) by [@jkurian](https://github.com/jkurian)
- Cirrus CI for FreeBSD CI ([#124](https://github.com/jonhoo/inferno/pull/124) from [@AnderEnder](https://github.com/AnderEnder))

### Changed
- Moved to `IndexMap` and FNV hashing ([#127](https://github.com/jonhoo/inferno/pull/127))
- Moved CI to Azure DevOps Pipelines

[Unreleased]: https://github.com/jonhoo/inferno/compare/v0.11.6...HEAD
[0.11.6]: https://github.com/jonhoo/inferno/compare/v0.11.5...v0.11.6
[0.11.5]: https://github.com/jonhoo/inferno/compare/v0.11.4...v0.11.5
[0.11.4]: https://github.com/jonhoo/inferno/compare/v0.11.3...v0.11.4
[0.11.3]: https://github.com/jonhoo/inferno/compare/v0.11.2...v0.11.3
[0.11.2]: https://github.com/jonhoo/inferno/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/jonhoo/inferno/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/jonhoo/inferno/compare/v0.10.12...v0.11.0
[0.10.12]: https://github.com/jonhoo/inferno/compare/v0.10.11...v0.10.12
[0.10.11]: https://github.com/jonhoo/inferno/compare/v0.10.10...v0.10.11
[0.10.10]: https://github.com/jonhoo/inferno/compare/v0.10.9...v0.10.10
[0.10.9]: https://github.com/jonhoo/inferno/compare/v0.10.8...v0.10.9
[0.10.8]: https://github.com/jonhoo/inferno/compare/v0.10.7...v0.10.8
[0.10.7]: https://github.com/jonhoo/inferno/compare/v0.10.6...v0.10.7
[0.10.6]: https://github.com/jonhoo/inferno/compare/v0.10.5...v0.10.6
[0.10.5]: https://github.com/jonhoo/inferno/compare/v0.10.4...v0.10.5
[0.10.4]: https://github.com/jonhoo/inferno/compare/v0.10.3...v0.10.4
[0.10.3]: https://github.com/jonhoo/inferno/compare/v0.10.2...v0.10.3
[0.10.2]: https://github.com/jonhoo/inferno/compare/v0.10.1...v0.10.2
[0.10.1]: https://github.com/jonhoo/inferno/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/jonhoo/inferno/compare/v0.9.9...v0.10.0
[0.9.9]: https://github.com/jonhoo/inferno/compare/v0.9.7...v0.9.9
[0.9.8]: https://github.com/jonhoo/inferno/compare/v0.9.7...v0.9.8
[0.9.7]: https://github.com/jonhoo/inferno/compare/v0.9.6...v0.9.7
[0.9.6]: https://github.com/jonhoo/inferno/compare/v0.9.5...v0.9.6
[0.9.5]: https://github.com/jonhoo/inferno/compare/v0.9.4...v0.9.5
[0.9.4]: https://github.com/jonhoo/inferno/compare/v0.9.3...v0.9.4
[0.9.3]: https://github.com/jonhoo/inferno/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/jonhoo/inferno/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/jonhoo/inferno/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/jonhoo/inferno/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/jonhoo/inferno/compare/v0.7.0...v0.8.0
