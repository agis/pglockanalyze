use postgres::types::{accepts, FromSql, Type};
use serde::Serialize;

/// Possible values of the pg_locks.locktype column
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum LockType {
    /// Waiting to acquire a lock on a relation
    Relation,

    /// Waiting to acquire a lock on a non-relation database object
    Object,
}

impl<'a> FromSql<'a> for LockType {
    accepts!(TEXT);

    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let lock_type = match String::from_sql(ty, raw)?.as_str() {
            "object" => Self::Object,
            "relation" => Self::Relation,
            other => return Err(format!("invalid locktype {other}").into()),
        };

        Ok(lock_type)
    }
}
