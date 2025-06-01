use crate::errors::Error;
use crate::statement::Statement;
use postgres as pg;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct AnalyzerConfig {
    /// The Postgres connection string to connect to.
    pub db_connection_uri: String,

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
    type Error = Error;

    fn try_from(config: AnalyzerConfig) -> Result<Self, Error> {
        Ok(Analyzer {
            db: postgres::Config::from_str(&config.db_connection_uri)?,
            config,
        })
    }
}

impl Analyzer {
    pub fn analyze(&self, sql: &str) -> Result<Vec<Statement>, Error> {
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

    fn finalize(&self, tx: pg::Transaction) -> Result<(), Error> {
        if self.config.commit {
            tx.commit()?
        } else {
            tx.rollback()?
        }

        Ok(())
    }
}
