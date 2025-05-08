use postgres::types::Oid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Database {
    /// [pg_locks.database] OID of the database in which the lock target exists,
    /// or zero if the target is a shared object, or null if the target is a
    /// transaction ID
    #[serde(skip)]
    pub oid: Oid,

    pub name: String,
}
