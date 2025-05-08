use crate::errors::Error;
use postgres as pg;
use postgres::types::Oid;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

mod database;
mod lock_type;
mod table_lock_mode;
mod target;
use database::Database;
use lock_type::LockType;
use table_lock_mode::TableLockMode;
use target::Target;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Locks(HashSet<Lock>);

impl Locks {
    pub fn compute_acquired(before: HashSet<Lock>, after: HashSet<Lock>) -> Locks {
        Locks(after.difference(&before).cloned().collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lock {
    /// [pg_locks.database] database in which the lock target exists
    // Always populated since we filter for relation/object locktypes.
    database: Database,

    /// [pg_locks.mode] Name of the lock mode held or desired by this process
    mode: TableLockMode,

    /// Human-readable representation of the locked object
    lock_target: Target,
}

impl TryFrom<pg::Row> for Lock {
    type Error = Error;

    fn try_from(row: pg::Row) -> Result<Self, Self::Error> {
        let locktype = row.try_get("locktype")?;
        let lock_target_alias = row.try_get("target")?;

        let lock_target = match locktype {
            LockType::Relation => {
                let oid: Option<Oid> = row.try_get("relation")?;
                Target::Relation {
                    oid: oid.ok_or("expected relation to be non-null")?,
                    alias: lock_target_alias,
                }
            }
            LockType::Object => {
                let oid: Option<Oid> = row.try_get("objid")?;
                Target::Object {
                    oid: oid.ok_or("expected objid to be non-null")?,
                    alias: lock_target_alias,
                }
            }
        };

        let database = Database {
            oid: row.try_get("database")?,
            name: row.try_get("database_name")?,
        };

        Ok(Self {
            lock_target,
            database,
            mode: row.try_get("mode")?,
        })
    }
}

impl fmt::Display for Lock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "acquired `{:#?}` lock on {}",
            self.mode, self.lock_target
        )
    }
}

impl fmt::Display for Locks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locks = self
            .0
            .iter()
            .map(|lock| format!("\t{}", lock))
            .collect::<Vec<String>>();

        let s = if locks.is_empty() {
            "\t(no locks)"
        } else {
            &locks.join("\n")
        };

        write!(f, "{}", s)
    }
}
