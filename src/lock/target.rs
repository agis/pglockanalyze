use postgres::types::Oid;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Target depicts the resource that was locked, i.e. the acquired lock's target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Object {
        #[serde(skip)]
        oid: Oid,
        alias: String,
    },

    Relation {
        #[serde(skip)]
        oid: Oid,
        alias: String,
    },
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (kind, alias, oid) = match self {
            Target::Object { oid, alias } => ("object", alias, oid),
            Target::Relation { oid, alias } => ("relation", alias, oid),
        };
        write!(f, "{} `{}` (oid={})", kind, alias, oid)
    }
}
