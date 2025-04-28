# pglockanalyze

Analyze PostgreSQL locks acquired by DDL statements.

For use in CI or development environments.

## Status

This software is experimental and under development; not yet ready for
production use.

## Usage

```shell
$ ./pglockanalyze --db "postgresql://a:b@localhost/db" "ALTER TABLE users ALTER COLUMN name SET NOT NULL"
ALTER TABLE users ALTER COLUMN name SET NOT NULL
	acquired `AccessExclusive` lock on relation `users` (oid=16386)
```
