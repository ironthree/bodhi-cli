# Release 2.0.0-beta.2 "Modernist Times" (January 28, 2022)

This release includes a few minor new features and changes:

- add generated bash completions for bodhi-cli
- add `create-update-overrides` subcommand to create buildroot overrides for all
  builds in a given update
- make output of most commands a little easier on the eyes
- hide some informational output behind a `--verbose` switch

# Release 2.0.0-beta.1 "Modern Times" (January 23, 2022)

Changes:

- ported to `fedora ^2.0.0-beta.1` and `bodhi ^2.0.0-beta.1`
- use `tokio` as async runtime
- add CLI flags for specifying test results in the `waive-tests` command
- remove CLI flags for querying updates based on `date_approved`
  (this value is never used or written, and is always `None` on bodhi servers)

# Release 1.1.0 "Cookie Monster" (September 25, 2021)

Changes:

- use `bodhi ^1.1` and `fedora ^1.1` for better and faster session handling
- implement code improvements suggested by clippy

# Release 1.0.2 "Move Along" (June 03, 2021)

Internal changes:

- fix some warnings raised by newer versions of clippy

# Release 1.0.1 "Better Secretz" (January 06, 2021)

Internal changes:

- update `secret-service` to `2.0` (with the new `zbus` backend)

# Release 1.0.0 "Up This Grade" (November 30, 2020)

Internal changes:

- update `bodhi` to `1.0`
- update `rpassword` to `5.0`
- update `dirs` to `3.0.1`

# Release 0.4.1 "Exclude()" (October 29, 2020)

This release only contains non-code changes:

- exclude both the `.github` folder and `rustfmt.toml` from published crates

# Release 0.4.0 "Drop()" (October 28, 2020)

Breaking Changes:

- refactored code into a binary-only crate
- dropped unused `bodhi_cli` library component

# Release 0.3.3 "Bump2()" (June 22, 2020)

Changes:

- bump `bodhi` dependency to `^0.6` for bodhi 5.4.0 server support

# Release 0.3.1 "Patch()" (Mar. 09, 2020)

Incremental improvements:

- fix the description of password saved in the session keyring

# Release 0.3.0 "ForgetMeNot()" (Mar. 08, 2020)

New features:

- store FAS password in the session keyring (using `libsecret` /
  `SecretService` D-Bus API)
- disable storing the password by using the `--no-store-password` CLI flag
- ignore the previously stored password by using the `--ignore-keyring` CLI flag

# Release 0.2.5 "Limit()" (Feb. 24, 2020)

Incremental improvements:

- warn about long-running queries (all updates, all overrides)
- require `--force` CLI switch to run those queries anyway

# Release 0.2.4 "Bump()" (Feb. 17, 2020)

Incremental improvements:

- update dependencies
- use more nice structopt features (colored error messages, etc.)

# Release 0.2.3 "Fix()" (Jan. 31, 2020)

Fix a typo in the `term_size` dependency version.

# Release 0.2.2 "Update()" (Jan. 19, 2020)

Incremental improvements:

- bump `bodhi` requirement to 0.5.0
- adapt to minor API changes

# Release 0.2.1 "Require()" (Jan. 19, 2020)

New features:

- allow setting `require_bugs` and `require_testcases` attributes when creating
  an update with the CLI

# Release 0.2.0 "Parse()" (Jan. 17, 2020)

Incremental improvements:

- bump `bodhi` requirement to 0.4.0
- adapt to minor API changes
- drop parsing and conversion code in favor of the new features in `bodhi`
- simplify `structopt` handling (automatic parsing of CLI arguments into enums)

# Release 0.1.0 "Clean()" (Jan. 17, 2020)

Incremental improvements:

- refactor code into separate modules
- deduplicate common functionality (like converting from strings to enums)

# Release 0.0.1 "Init()" (Jan. 15, 2020)

Initial, rough implementation of a bodhi CLI based on the rust bindings.

Features:

- almost-parity with the official python bodhi CLI client
- additional feature: automatically de-paginate paginated results
- additional feature: nice progress bars for long-running queries
- additional feature: optional JSON output of queries
