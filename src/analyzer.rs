use crate::errors::Error;
use crate::statement::Statement;
use std::str::FromStr;

pub struct Analyzer {
    // TODO: merge these two into a single Config struct?
    config: postgres::Config,
    wrap_in_transaction: bool,
}

impl Analyzer {
    pub fn new(connection: &str, wrap_in_transaction: bool) -> Result<Analyzer, Error> {
        Ok(Analyzer {
            wrap_in_transaction,
            config: postgres::Config::from_str(connection)?,
        })
    }

    pub fn analyze_one(&self, sql: &str) -> Result<Statement, Error> {
        let mut client = self.config.connect(postgres::NoTls)?;
        let mut tx = client.transaction()?;
        let pid = tx.query_one("SELECT pg_backend_pid()", &[])?.get(0);

        let statements = pg_query::parse(sql)?.protobuf.stmts;
        if statements.len() != 1 {
            return Err("expected a single statement".into());
        }

        let stmt = statements[0].stmt.as_ref().unwrap().deparse()?;
        let result = Statement::analyze(&self.config, stmt, &mut tx, pid);

        tx.rollback()?;

        result
    }

    pub fn analyze_many(&self, sql: &str) -> Result<Vec<Statement>, Error> {
        let stmts = pg_query::parse(sql)?
            .protobuf
            .stmts
            .into_iter()
            .map(|stmt| stmt.stmt.unwrap().deparse().unwrap());

        if self.wrap_in_transaction {
            let mut client = self.config.connect(postgres::NoTls)?;
            let mut tx = client.transaction()?;
            let pid = tx.query_one("SELECT pg_backend_pid()", &[])?.get(0);
            let result = stmts
                .map(|s| Statement::analyze(&self.config, s, &mut tx, pid))
                .collect();
            tx.rollback()?;
            result
        } else {
            stmts.map(|s| self.analyze_one(&s)).collect()
        }
    }
}
