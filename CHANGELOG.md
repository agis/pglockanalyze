# Changelog

## 0.0.4 (2025-31-08)

### Added

- Implement `TryFrom<AnalyzerConfig>` for `Analyzer` [[b80a8a3](https://github.com/agis/pglockanalyze/commit/b80a8a30f79fe0a5e5ad905899437a6d71d177d6)]

### Fixed

- In certain multi-line statements the `end_line` information was be incorrect [[fdf25451d4ee68eecbd78f933f63b30e22cb7eab](https://github.com/agis/pglockanalyze/commit/fdf25451d4ee68eecbd78f933f63b30e22cb7eab))

### Changed

- Make `Locks.compute_acquired`, `Statement.analyze` and `Statement.detect_locks` methods private [[b2cc458a](https://github.com/agis/pglockanalyze/commit/b2cc458a66810a931a0c7800adddd715d7e0a643)]



## 0.0.3 (2025-05-31)

### Added

- Expose line number information for each statement analyzed [[0afd66c](https://github.com/agis/pglockanalyze/commit/0afd66c30f2016601946f1639ebf23a426d4ce04)]
