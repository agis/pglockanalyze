use postgres::types::{accepts, FromSql, Type};
use serde::{Deserialize, Serialize};

/// Possible values of the pg_locks.mode column
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum TableLockMode {
    /// Conflicts with the ACCESS EXCLUSIVE lock mode only.
    ///
    /// The SELECT command acquires a lock of this mode on referenced tables. In
    /// general, any query that only reads a table and does not modify it will
    /// acquire this lock mode.
    AccessShare,

    /// Conflicts with the EXCLUSIVE and ACCESS EXCLUSIVE lock modes.
    ///
    /// The SELECT command acquires a lock of this mode on all tables on which
    /// one of the FOR UPDATE, FOR NO KEY UPDATE, FOR SHARE, or FOR KEY SHARE
    /// options is specified (in addition to ACCESS SHARE locks on any other
    /// tables that are referenced without any explicit FOR ... locking option).
    RowShare,

    /// Conflicts with the SHARE, SHARE ROW EXCLUSIVE, EXCLUSIVE, and ACCESS
    /// EXCLUSIVE lock modes.
    ///
    /// The commands UPDATE, DELETE, INSERT, and MERGE acquire this lock mode on
    /// the target table (in addition to ACCESS SHARE locks on any other
    /// referenced tables). In general, this lock mode will be acquired by any
    /// command that modifies data in a table.
    RowExclusive,

    /// Conflicts with the SHARE UPDATE EXCLUSIVE, SHARE, SHARE ROW EXCLUSIVE,
    /// EXCLUSIVE, and ACCESS EXCLUSIVE lock modes. This mode protects a table
    /// against concurrent schema changes and VACUUM runs.
    ///
    /// Acquired by VACUUM (without FULL), ANALYZE, CREATE INDEX CONCURRENTLY,
    /// CREATE STATISTICS, COMMENT ON, REINDEX CONCURRENTLY, and certain ALTER
    /// INDEX and ALTER TABLE variants (for full details see the documentation
    /// of these commands).
    ShareUpdateExclusive,

    /// Conflicts with the ROW EXCLUSIVE, SHARE UPDATE EXCLUSIVE, SHARE ROW
    /// EXCLUSIVE, EXCLUSIVE, and ACCESS EXCLUSIVE lock modes. This mode
    /// protects a table against concurrent data changes.
    ///
    /// Acquired by CREATE INDEX (without CONCURRENTLY).
    Share,

    /// Conflicts with the ROW EXCLUSIVE, SHARE UPDATE EXCLUSIVE, SHARE, SHARE
    /// ROW EXCLUSIVE, EXCLUSIVE, and ACCESS EXCLUSIVE lock modes. This mode
    /// protects a table against concurrent data changes, and is self-exclusive
    /// so that only one session can hold it at a time.
    ///
    /// Acquired by CREATE TRIGGER and some forms of ALTER TABLE.
    ShareRowExclusive,

    /// Conflicts with the ROW SHARE, ROW EXCLUSIVE, SHARE UPDATE EXCLUSIVE,
    /// SHARE, SHARE ROW EXCLUSIVE, EXCLUSIVE, and ACCESS EXCLUSIVE lock
    /// modes. This mode allows only concurrent ACCESS SHARE locks, i.e., only
    /// reads from the table can proceed in parallel with a transaction holding
    /// this lock mode.
    ///
    /// Acquired by REFRESH MATERIALIZED VIEW CONCURRENTLY.
    Exclusive,

    /// Conflicts with locks of all modes (ACCESS SHARE, ROW SHARE, ROW
    /// EXCLUSIVE, SHARE UPDATE EXCLUSIVE, SHARE, SHARE ROW EXCLUSIVE,
    /// EXCLUSIVE, and ACCESS EXCLUSIVE). This mode guarantees that the holder
    /// is the only transaction accessing the table in any way.
    ///
    /// Acquired by the DROP TABLE, TRUNCATE, REINDEX, CLUSTER, VACUUM FULL, and
    /// REFRESH MATERIALIZED VIEW (without CONCURRENTLY) commands. Many forms of
    /// ALTER INDEX and ALTER TABLE also acquire a lock at this level. This is
    /// also the default lock mode for LOCK TABLE statements that do not specify
    /// a mode explicitly.
    AccessExclusive,
}

impl<'a> FromSql<'a> for TableLockMode {
    accepts!(TEXT);

    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        // TODO: don't panic
        let lock_mode = match String::from_sql(ty, raw).unwrap().as_str() {
            "ShareLock" => TableLockMode::Share,
            "RowShareLock" => TableLockMode::RowShare,
            "ExclusiveLock" => TableLockMode::Exclusive,
            "AccessShareLock" => TableLockMode::AccessShare,
            "RowExclusiveLock" => TableLockMode::RowExclusive,
            "AccessExclusiveLock" => TableLockMode::AccessExclusive,
            "ShareRowExclusiveLock" => TableLockMode::ShareRowExclusive,
            "ShareUpdateExclusiveLock" => TableLockMode::ShareUpdateExclusive,
            other => return Err(format!("invalid TableLockMode {}", other).into()),
        };

        Ok(lock_mode)
    }
}
