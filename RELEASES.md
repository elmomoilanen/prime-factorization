# Release 1.0.5 (06-12-2024)

### Fixed

- Bug in Fermat's factorization method (factors found by this method are not necessarily prime factors)

### Changed

- Set max number of Fermat's factorization rounds to three (previously five)
- Minor code refactoring wrt. wheel and trial division factorization methods
- Documentation

# Release 1.0.4 (10-06-2023)

### Fixed

- Clippy uninlined-format-args warnings

### Changed

- Default thread count when running elliptic curve factorization
- Documentation
- Keywords in Cargo.toml

# Release 1.0.3 (19-11-2022)

### Changed

- Explicit derefs removed when auto-deref possible
- Documentation
- Disable overflow-checks in dev mode by default

### Added

- Version number CLI argument

# Release 1.0.2 (13-10-2022)

### Fixed

- Set 0^0 to equal 1

### Changed

- Optimize elliptic-curve computations (Montgomery ladder)
- Documentation

### Added

- Benchmarking
- itertools crate

# Release 1.0.1 (12-08-2022)

### Fixed

- Licence link in README

### Changed

- Documentation

# Release 1.0.0 (10-08-2022)

### Added

- Everything
