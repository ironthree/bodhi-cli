## bodhi CLI client based on bodhi-rs (DEPRECATED)

> [!WARNING]
> This application no longer works as of bodhi-server 8.0.0 due to breaking
> changes to / removal of the OpenID authentication endpoint.

This CLI client for `bodhi-rs` exposes almost all functionality of the official
`bodhi` python client, and almost all features of the `bodhi-rs` rust bindings.

By default, `bodhi-cli` will store the FAS password in the session keyring, when
the user is first prompted to enter it. To disable this, pass the
`--no-store-password` / `-n` CLI switch. To ignore any previously saved
passwords, pass the `--ignore-keyring` / `-k` CLI switch (for example, if you've
changed your password).

