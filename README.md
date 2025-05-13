# pglockanalyze&emsp;[![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/agis/pglockanalyze/ci.yml?branch=main
[actions]: https://github.com/agis/pglockanalyze/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/pglockanalyze.svg
[crates.io]: https://crates.io/crates/pglockanalyze

See what PostgreSQL locks your migrations acquired.

For use in CI and development environments.

## Status

This software is experimental and under development.

## Rationale

Understanding what locks your migrations will acquire is critical if you want to
avoid inducing downtime in your production traffic. To that end, we have
different tools like the [official Postgres
documentation](https://www.postgresql.org/docs/current/explicit-locking.html)
and [strong_migrations](https://github.com/ankane/strong_migrations).

While such tools are crucial and battle-tested, reasoning your way through the
locks your migrations will acquire is not always easy. Static analysis can only
get you so far; it's possible that a new Postgres version might change the types
of locks a particular sequence of DDL statements might acquire.

pglockanalyze is meant to act as a complement to the above tools. It actually
executes your migrations and detects at run-time the locks they acquired. It is
meant to be integrated to CI pipelines and/or development environments.

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
