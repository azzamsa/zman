## [Unreleased]

## [v1.0.4] - 2021-11-15

### Development
- Port to clap v2. The v3 API is always changing.

## [v1.0.3] - 2021-11-14

- Migrate to GitHub

## [v1.0.0] - 2021-04-20

- All bugs are fixed, and the API is now stabilized. Now `zman` is ready to have a show.

### Features
- Show a char to distinguish a time in JSON mode. Now a JSON output will be prefixed with `y`, `m`, or `w`. To better distinguish the output.

### Bug Fixes
- Fixed wrong month and week ratio computation. Now week and month able to reach 100% in the end of time.

## [v0.1.3] - 2021-04-20

### Bug Fixes
- Use time as an argument instead of an option. Now it's `zman year` instead of `zman --year`.

### Bug Fixes
- Fix wrong name in argument doc

## [v0.1.2] - 2020-09-29

### Bug Fixes
- Fix wrong name in argument doc

## [v0.1.1] - 2020-09-29

### Features
- Add support for bar width, full bar, and rest bar string.
- Sane default value.

## [v0.1.0] - 2020-09-29

Initial release
