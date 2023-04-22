![master](https://github.com/pascalPost/axum-sqlx-todo/actions/workflows/rust.yml/badge.svg?branch=master)


# Compilation
The database queries are compile time checked. For this, run
`cargo install sqlx-cli`, (see
https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md),
`sqlx database create` and `sqlx migrate run` prior to `cargo build`.
