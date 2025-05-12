use pglockanalyze::analyzer::*;
use pglockanalyze::analyzer_config::*;
use pglockanalyze::statement::*;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use std::path::PathBuf;
use std::str::FromStr;

const DB_NAME: &str = "pglatests";
const DB_USER: &str = "pglauser";
const DB_PASS: &str = "pglapass";

#[derive(Debug, Deserialize)]
struct TestCase {
    // inputs
    starting_schema: String,
    statements: String,

    // output
    expected: Vec<Statement>,
}

// we do not want to execute tests in parallel, since they act on the same
// database and we'll have deadlocks
#[test]
fn test_all() {
    test_wrap_in_transaction_rollback();
    test_no_wrap_in_transaction_commit();
}

fn test_wrap_in_transaction_rollback() {
    let fixture_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(file!())
        .parent()
        .unwrap()
        .join("fixture.yml");
    let fixtures = std::fs::read_to_string(fixture_file).unwrap();
    let test_cases: Vec<TestCase> = serde_yaml::from_str(&fixtures).unwrap();

    for test_case in &test_cases {
        reset_db(&test_case.starting_schema);

        let cfg = AnalyzerConfig {
            db_connection_uri: db(),
            distinct_transactions: false,
            commit: false,
        };

        let stmts = Analyzer::from(cfg)
            .unwrap()
            .analyze(&test_case.statements)
            .unwrap();

        let actual = serde_yaml::to_string(&stmts).unwrap();
        let expected = serde_yaml::to_string(&test_case.expected).unwrap();

        assert_eq!(actual, expected);
    }
}

fn test_no_wrap_in_transaction_commit() {
    let fixture_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(file!())
        .parent()
        .unwrap()
        .join("fixture_non_wrapping.yml");
    let fixtures = std::fs::read_to_string(fixture_file).unwrap();
    let test_cases: Vec<TestCase> = serde_yaml::from_str(&fixtures).unwrap();

    for test_case in &test_cases {
        reset_db(&test_case.starting_schema);

        let cfg = AnalyzerConfig {
            db_connection_uri: db(),
            distinct_transactions: true,
            commit: true,
        };

        let stmts = Analyzer::from(cfg)
            .unwrap()
            .analyze(&test_case.statements)
            .unwrap();

        let actual = serde_yaml::to_string(&stmts).unwrap();
        let expected = serde_yaml::to_string(&test_case.expected).unwrap();

        assert_eq!(actual, expected);
    }
}

fn reset_db(bootstrap: &str) {
    let mut client = postgres::Config::from_str(&db())
        .unwrap()
        .connect(postgres::NoTls)
        .unwrap();

    let cleanup_sql = format!(
        "DROP SCHEMA IF EXISTS public CASCADE;
             CREATE SCHEMA public;
             GRANT ALL ON SCHEMA public TO {DB_USER};
             GRANT ALL ON SCHEMA public TO public;
             SET search_path = public;"
    );

    client.batch_execute(&cleanup_sql).unwrap();
    client.batch_execute(bootstrap).unwrap();
}

fn db() -> String {
    format!(
        "postgresql://{}:{}@localhost:38471/{}",
        DB_USER, DB_PASS, DB_NAME
    )
}
