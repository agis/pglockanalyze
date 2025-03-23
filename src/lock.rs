use crate::errors::Error;
use postgres as pg;
use postgres::types::Oid;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt;

mod lock_type;
mod table_lock_mode;
mod target;
use lock_type::LockType;
use table_lock_mode::TableLockMode;
use target::Target;

#[derive(Debug, Serialize)]
pub struct Locks(pub HashSet<Lock>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Lock {
    /// Human-readable representation of the locked object
    lock_target: Target,

    /// [pg_locks.locktype] Type of the lockable object
    #[serde(skip)]
    locktype: LockType,

    /// [pg_locks.database] OID of the database in which the lock target exists,
    /// or zero if the target is a shared object, or null if the target is a
    /// transaction ID
    database: Option<Oid>,

    /// [pg_locks.mode] Name of the lock mode held or desired by this process
    mode: TableLockMode,
}

impl Lock {
    pub fn compute_acquired(before: HashSet<Self>, after: HashSet<Self>) -> Locks {
        let mut acquired_locks = HashSet::new();

        for lock in after.difference(&before) {
            acquired_locks.insert(lock.clone());
        }

        Locks(acquired_locks)
    }
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

        Ok(Self {
            locktype,
            lock_target,
            database: row.try_get("database")?,
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
