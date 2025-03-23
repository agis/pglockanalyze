use postgres::types::Oid;
use serde::Serialize;
use std::fmt;

/// Target depicts the resource that was locked, i.e. the acquired lock's target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Target {
    #[serde(rename = "object")]
    Object { oid: Oid, alias: String },

    #[serde(rename = "relation")]
    Relation { oid: Oid, alias: String },
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Object { oid, alias } => {
                write!(f, "object `{}` (oid={})", alias, oid)
            }
            Target::Relation { oid, alias } => {
                write!(f, "relation `{}` (oid={})", alias, oid)
            }
        }
    }
}
