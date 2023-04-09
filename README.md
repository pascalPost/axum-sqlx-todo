# Compilation
The database queries are compile time checked. For this, run
`cargo install sqlx-cli`, (see
https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md),
`sqlx create db` and `sqlx migrate run` prior to `cargo build`.
