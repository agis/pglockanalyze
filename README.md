# pglockanalyze

See what PostgreSQL locks your migrations acquired.

For use in CI and development environments.

## Status

This software is experimental and under development; not yet ready for
production use.

## Usage

```shell
$ echo 'ALTER TABLE users ALTER COLUMN name SET NOT NULL' | pglockanalyze --db 'postgresql://a:b@localhost/db'
ALTER TABLE users ALTER COLUMN name SET NOT NULL
	acquired `AccessExclusive` lock on relation `users` (oid=16386)
```

Use `--help` to see all options.
