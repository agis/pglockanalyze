use crate::analyzer_config::AnalyzerConfig;
use crate::errors::Error;
use crate::statement::Statement;
use std::str::FromStr;

pub struct Analyzer {
    config: AnalyzerConfig,
    db: postgres::Config,
}

impl Analyzer {
    pub fn from(config: AnalyzerConfig) -> Result<Analyzer, Error> {
        Ok(Analyzer {
            db: postgres::Config::from_str(&config.db_connection_uri)?,
            config,
        })
    }

    pub fn analyze(&self, sql: &str) -> Result<Vec<Statement>, Error> {
        const FETCH_PID: &str = "SELECT pg_backend_pid()";

        let stmts = pg_query::parse(sql)?
            .protobuf
            .stmts
            .into_iter()
            .map(|stmt| stmt.stmt.unwrap().deparse().unwrap());

        if self.config.distinct_transactions {
            // each statement executes in its own transaction
            let mut result = Vec::new();

            for stmt in stmts {
                let mut client = self.db.connect(postgres::NoTls)?;
                let mut tx = client.transaction()?;
                let pid = tx.query_one(FETCH_PID, &[])?.get(0);

                result.push(Statement::analyze(&self.db, stmt, &mut tx, pid)?);

                self.finalize(tx)?
            }

            Ok(result)
        } else {
            // all statements execute under a single transaction
            let mut client = self.db.connect(postgres::NoTls)?;
            let mut tx = client.transaction()?;
            let pid = tx.query_one(FETCH_PID, &[])?.get(0);

            let result = stmts
                .map(|s| Statement::analyze(&self.db, s, &mut tx, pid))
                .collect();

            self.finalize(tx)?;

            result
        }
    }

    fn finalize(&self, tx: postgres::Transaction) -> Result<(), Error> {
        if self.config.commit {
            tx.commit()?
        } else {
            tx.rollback()?
        }

        Ok(())
    }
}
