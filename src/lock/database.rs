use postgres::types::Oid;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Database {
    /// [pg_locks.database] OID of the database in which the lock target exists,
    /// or zero if the target is a shared object, or null if the target is a
    /// transaction ID
    pub oid: Oid,
    pub name: String,
}
