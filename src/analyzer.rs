use crate::errors::Error;
use crate::statement::Statement;
use std::str::FromStr;

pub struct Analyzer {
    // TODO: merge these two into a single Config struct?
    config: postgres::Config,
    wrap_in_transaction: bool,
    commit: bool,
}

impl Analyzer {
    pub fn new(
        connection: &str,
        wrap_in_transaction: bool,
        commit: bool,
    ) -> Result<Analyzer, Error> {
        Ok(Analyzer {
            config: postgres::Config::from_str(connection)?,
            wrap_in_transaction,
            commit,
        })
    }

    pub fn analyze(&self, sql: &str) -> Result<Vec<Statement>, Error> {
        const FETCH_PID: &str = "SELECT pg_backend_pid()";

        let stmts = pg_query::parse(sql)?
            .protobuf
            .stmts
            .into_iter()
            .map(|stmt| stmt.stmt.unwrap().deparse().unwrap());

        if self.wrap_in_transaction {
            // all statements executed under a single transaction
            let mut client = self.config.connect(postgres::NoTls)?;
            let mut tx = client.transaction()?;
            let pid = tx.query_one(FETCH_PID, &[])?.get(0);

            let result = stmts
                .map(|s| Statement::analyze(&self.config, s, &mut tx, pid))
                .collect();

            self.finalize(tx)?;

            result
        } else {
            // each statement executes in a new transaction
            let mut result = Vec::new();

            for stmt in stmts {
                let mut client = self.config.connect(postgres::NoTls)?;
                let mut tx = client.transaction()?;
                let pid = tx.query_one(FETCH_PID, &[])?.get(0);

                result.push(Statement::analyze(&self.config, stmt, &mut tx, pid)?);

                self.finalize(tx)?
            }

            Ok(result)
        }
    }

    fn finalize(&self, tx: postgres::Transaction) -> Result<(), Error> {
        if self.commit {
            tx.commit()?
        } else {
            tx.rollback()?
        }

        Ok(())
    }
}
