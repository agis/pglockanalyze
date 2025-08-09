use pglockanalyze::analyzer::*;
use pglockanalyze::statement::*;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use std::path::PathBuf;
use std::str::FromStr;

// these are set in Makefile
const DB_NAME: &str = "pglatests";
const DB_USER: &str = "pglauser";
const DB_PASS: &str = "pglapass";
const DB_HOST: &str = "localhost";
const DB_PORT: &str = "38471";

#[derive(Debug, Deserialize)]
struct TestCase {
    // inputs
    initial_schema: String,
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
    test_locations();
}

fn test_wrap_in_transaction_rollback() {
    let test_cases = load_fixture_file("fixture.yml");

    for test_case in &test_cases {
        reset_db(&test_case.initial_schema);

        let cfg = AnalyzerConfig {
            db_connection_string: db(),
            distinct_transactions: false,
            commit: false,
        };

        let stmts = Analyzer::try_from(cfg)
            .unwrap()
            .analyze(&test_case.statements)
            .unwrap();

        let actual = serde_yaml::to_string(&stmts).unwrap();
        let expected = serde_yaml::to_string(&test_case.expected).unwrap();

        assert_eq!(actual, expected);
    }
}

fn test_no_wrap_in_transaction_commit() {
    let test_cases = load_fixture_file("fixture_non_wrapping.yml");

    for test_case in &test_cases {
        reset_db(&test_case.initial_schema);

        let cfg = AnalyzerConfig {
            db_connection_string: db(),
            distinct_transactions: true,
            commit: true,
        };

        let stmts = Analyzer::try_from(cfg)
            .unwrap()
            .analyze(&test_case.statements)
            .unwrap();

        let actual = serde_yaml::to_string(&stmts).unwrap();
        let expected = serde_yaml::to_string(&test_case.expected).unwrap();

        assert_eq!(actual, expected);
    }
}

fn test_locations() {
    let test_cases = load_fixture_file("fixture_locations.yml");

    for test_case in &test_cases {
        reset_db(&test_case.initial_schema);

        let cfg = AnalyzerConfig {
            db_connection_string: db(),
            distinct_transactions: true,
            commit: true,
        };

        let stmts = Analyzer::try_from(cfg)
            .unwrap()
            .analyze(&test_case.statements)
            .unwrap();

        let actual = serde_yaml::to_string(&stmts).unwrap();
        let expected = serde_yaml::to_string(&test_case.expected).unwrap();

        assert_eq!(actual, expected);
    }
}

fn load_fixture_file(fname: &str) -> Vec<TestCase> {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(file!())
        .parent()
        .unwrap()
        .join(fname);

    let fixture = std::fs::read_to_string(file).unwrap();

    serde_yaml::from_str(&fixture).unwrap()
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
    format!("postgresql://{DB_USER}:{DB_PASS}@{DB_HOST}:{DB_PORT}/{DB_NAME}")
}
