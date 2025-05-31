# pglockanalyze&emsp;[![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/agis/pglockanalyze/ci.yml?branch=main
[actions]: https://github.com/agis/pglockanalyze/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/pglockanalyze.svg
[crates.io]: https://crates.io/crates/pglockanalyze

*See what locks your Postgres migrations will acquire—before you run them in production.*

<p align="center">
  <img src="https://github.com/user-attachments/assets/3539ef87-8bce-436c-a826-fbdc4a7da526" />
</p>

To be used in CI and development environments; see
[pglockanalyze-action](https://github.com/agis/pglockanalyze-action) for
integration with GitHub Actions.

## Status

This software is in alpha stage - expect breaking changes between releases and a lot of rough edges.

## Rationale

Understanding the locks your migrations will acquire is crucial to avoiding
downtime in  production traffic. Tools like the [official Postgres
docs](https://www.postgresql.org/docs/current/explicit-locking.html) and
[strong_migrations](https://github.com/ankane/strong_migrations) are invaluable;
however, reasoning your way through complex DDL statements is not always
practical.

pglockanalyze is meant to complement, not replace such tools, by executing your
migrations against a test database (that you have to provision) and dynamically
identifying the locks acquired at runtime. It then prints a report of the locks
that were acquired.

By default, pglockanalyze rolls back the transactions it analyzes, so you can
safely run it against a test database without worrying about leaving it in and
inconsistent state. If you want to commit the transactions, you can use the
`--commit` option.

## Installation

You can install pglockanalyze using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```shell
$ cargo install pglockanalyze
```

We do not distribute binaries yet, but we may do so in the future.

## Usage

```shell
$ echo 'ALTER TABLE users ALTER COLUMN name SET NOT NULL' | pglockanalyze --db 'postgres://foo@bar'
ALTER TABLE users ALTER COLUMN name SET NOT NULL
	acquired `AccessExclusive` lock on relation `users` (oid=16386)
```

Use `--help` to see all options:

```shell
Usage: pglockanalyze [OPTIONS] --db <postgres connection string> [INPUT]

Arguments:
  [INPUT]  The DDL statements to analyze. If not provided or is -, read from standard input [default: -]

Options:
      --db <postgres connection string>
          The database to connect to
  -f, --format <FORMATTER>
          The output format of the analysis [default: plain] [possible values: plain, json]
      --distinct-transactions
          Execute each statement in its own transaction. By default all statements are executed in a single transaction. Implies --commit
      --commit
          Commit the transactions. By default they are rolled back
  -h, --help
          Print help
  -V, --version
          Print version
```

## License

pglockanalyze is licensed under the [Apache 2.0 license](LICENSE).
