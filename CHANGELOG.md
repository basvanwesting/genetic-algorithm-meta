# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.2] - 2024-07-27
### Changed
* Update genetic_algorithm dependency to v0.7.2
* Use `Wrapper`s instead of `Dispatcher`s as they keep state, behaviour is the same using `into()` (e.g. `MutateOnce::new(0.2).into()`)

## [0.7.1] - 2024-07-23
Initial version split off from genetic_algorithm v0.7.1
