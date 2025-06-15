use crate::statement::Statement;
use postgres as pg;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct AnalyzerConfig {
    /// The Postgres connection string to connect to.
    pub db_connection_string: String,

    /// If true, each statement will be executed in its own transaction.
    /// Otherwise, all statements will be executed in the same transaction.
    pub distinct_transactions: bool,

    /// If true, the transaction(s) will be committed. Otherwise they will be
    /// rolled back.
    pub commit: bool,
}

pub struct Analyzer {
    config: AnalyzerConfig,
    db: postgres::Config,
}

impl TryFrom<AnalyzerConfig> for Analyzer {
    type Error = ParseConnectionStringError;

    fn try_from(config: AnalyzerConfig) -> Result<Self, ParseConnectionStringError> {
        Ok(Analyzer {
            db: postgres::Config::from_str(&config.db_connection_string)?,
            config,
        })
    }
}

#[derive(Error, Debug)]
#[error("error creating configuration: {}", self.source)]
pub struct ParseConnectionStringError {
    #[from]
    source: postgres::Error,
}

impl Analyzer {
    pub fn analyze(&self, sql: &str) -> Result<Vec<Statement>, AnalyzeError> {
        const FETCH_PID: &str = "SELECT pg_backend_pid()";

        let stmts = Parser::parse_sql(&PostgreSqlDialect {}, sql)?;

        if self.config.distinct_transactions {
            // each statement executes in its own transaction
            let mut result = Vec::new();

            for stmt in stmts {
                let mut client = self.db.connect(postgres::NoTls)?;
                let mut tx = client.transaction()?;
                let pid = tx.query_one(FETCH_PID, &[])?.get(0);

                result.push(Statement::analyze(&self.db, &mut tx, pid, stmt)?);

                self.finalize(tx)?
            }

            Ok(result)
        } else {
            // all statements execute under a single transaction
            let mut client = self.db.connect(postgres::NoTls)?;
            let mut tx = client.transaction()?;
            let pid = tx.query_one(FETCH_PID, &[])?.get(0);

            let result = stmts
                .into_iter()
                .map(|stmt| Statement::analyze(&self.db, &mut tx, pid, stmt))
                .collect();

            self.finalize(tx)?;

            result
        }
    }

    fn finalize(&self, tx: pg::Transaction) -> Result<(), AnalyzeError> {
        if self.config.commit {
            tx.commit()
                .map_err(|e| AnalyzeError::TransactionCommit(e.to_string()))
        } else {
            tx.rollback()
                .map_err(|e| AnalyzeError::TransactionRollback(e.to_string()))
        }
    }
}

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error("{0}")]
    SQLParse(#[from] ParserError),

    #[error("{0}")]
    DatabaseConnect(#[from] pg::Error),

    #[error("error committing transaction: {0}")]
    TransactionCommit(String),

    #[error("error rolling back transaction: {0}")]
    TransactionRollback(String),

    #[error("unexpected NULL in column `{0}`")]
    UnexpectedNullColumn(LockColumn),
}

#[derive(Error, Debug)]
pub enum LockColumn {
    #[error("relation")]
    Relation,

    #[error("objid")]
    ObjectID,
}
