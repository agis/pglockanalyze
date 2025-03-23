use crate::errors::Error;
use crate::statement::Statement;
use std::str::FromStr;

pub struct Analyzer {
    config: postgres::Config,
    pub statements: Vec<Statement>,
}

impl Analyzer {
    pub fn new(connection: &str) -> Result<Analyzer, Error> {
        Ok(Analyzer {
            config: postgres::Config::from_str(connection)?,
            statements: Vec::new(),
        })
    }

    pub fn analyze_one(&mut self, sql: &str) -> Result<Statement, Error> {
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

    pub fn analyze_many(&mut self, sql: &str) -> Result<Vec<Statement>, Error> {
        pg_query::parse(sql)?
            .protobuf
            .stmts
            .into_iter()
            .map(|s| s.stmt.unwrap().deparse().unwrap())
            .map(|s| self.analyze_one(&s))
            .collect()
    }
}
