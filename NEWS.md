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
