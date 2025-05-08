use crate::errors::Error;
use crate::lock::{Lock, Locks};
use postgres as pg;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Statement {
    pub sql: String,
    pub locks_acquired: Locks,
}

impl Statement {
    pub fn analyze(
        config: &pg::Config,
        sql: String,
        tx: &mut pg::Transaction,
        pid: i32,
    ) -> Result<Self, Error> {
        let locks_before = Self::detect_locks(config, pid)?;
        tx.execute(&sql, &[])?;
        let locks_after = Self::detect_locks(config, pid)?;
        let locks_acquired = Locks::compute_acquired(locks_before, locks_after);

        Ok(Statement {
            sql,
            locks_acquired,
        })
    }

    fn detect_locks(config: &pg::Config, pid: i32) -> Result<HashSet<Lock>, Error> {
        const SQL: &str = "\
SELECT
    l.locktype,
    l.database,
    d.datname AS database_name,
    l.relation,
    l.objid,
    l.mode,
    CASE l.locktype
        WHEN 'relation' THEN l.relation::regclass::text
        WHEN 'object'   THEN 'object: ' || l.objid::text || ' (class: ' || l.classid::regclass::text || ')'
    END AS target
FROM
    pg_catalog.pg_locks l
LEFT JOIN
    pg_catalog.pg_database d
ON
    l.database = d.oid
WHERE
    l.pid = $1
    AND l.locktype IN ('relation', 'object')
    AND l.granted";

        config
            .connect(postgres::NoTls)?
            .query(SQL, &[&pid])?
            .into_iter()
            .map(Lock::try_from)
            .collect()
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.sql, self.locks_acquired)
    }
}
