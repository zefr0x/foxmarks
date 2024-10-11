# Unreleased

# 2.1.0

## Added

- `--firefox-home-path` CLI option

# 2.0.3

## Dependencies

- All dependencies are up to date.

# 2.0.2

## Changed

- Now libsqlite is shiped bundled with rusqlite, to link against the system's libsqlite you can use: `--no-default-features`.

# 2.0.1

## Fixed

- Wrong path in github workflow builds.

# 2.0.0

## Improved

- Config rust for better release binary size optimizations.

# 2.0.0beta.0

## Changed

- Rename `profile-id` option as `profile-path`. <sup>`Breaking Change`</sup>

## Improved

- How firefox-type is handled internaly and externaly (CLI interface). <sup>`Breaking Change`</sup>

## Dependencies

- All dependencies are up to date.

# 1.0.2

## Added

- Auto generate man pages using `clap_mangen`

## Dependencies

### Added

- clap_mangen

### Updated

- tempfile `3.3.0` -> `3.4.0`
- clap `4.0.32` -> `4.1.6`
- clap_complete `4.0.7` -> `4.1.3`

# 1.0.1

## Improved

- Clearify panic message when no suitable profile id for firefox exist.
- Now binaries has considerably smaller size.

## Dependencies

- Update `configparser` and `clap` and `clap_complete`.

# 1.0.0

## Fixed

- Partially fix unescaping characters from CLI and config file.

## Dependencies

- Update `clap` and `clap_complete`.

# 1.0.0-beta.0

- First release.
