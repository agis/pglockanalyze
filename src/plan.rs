use crate::analyzer::{AnalyzerConfig, Mode};
use crate::statement::Location;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};

#[expect(dead_code)]
pub(crate) struct Plan(Vec<Step>);

#[expect(dead_code)]
impl Plan {
    pub(crate) fn build(sql: &str, config: AnalyzerConfig) -> Self {
        let steps = Vec::new();

        if config.mode == Mode::Plain {
            let stmts = Parser::parse_sql(&PostgreSqlDialect {}, sql)?;
        } else {
            todo!()
        }

        match config.mode {
            Mode::Plain => Self(vec![Step {
                statements: vec![PlannedStatement {
                    sql: sql.to_string(),
                    location: Location {
                        start_line: 1,
                        end_line: sql.lines().count() as u64,
                    },
                }],
                finalize: if config.commit {
                    FinalizePolicy::Commit
                } else {
                    FinalizePolicy::Rollback
                },
            }]),
            Mode::SqlMigrate => todo!(),
        }
    }
}

#[expect(dead_code)]
struct Step {
    statements: Vec<PlannedStatement>,
    finalize: FinalizePolicy,
}

#[expect(dead_code)]
pub struct PlannedStatement {
    pub sql: String,
    pub location: Location,
}

#[expect(dead_code)]
enum FinalizePolicy {
    Commit,
    Rollback,
}
