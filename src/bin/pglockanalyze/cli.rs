use clap::{Parser, ValueEnum};
use patharg::InputArg;
use pglockanalyze::statement::Statement;

#[derive(Debug, Clone, ValueEnum)]
pub enum Formatter {
    Plain,
    Json,
}

impl Formatter {
    pub fn format(&self, stmts: Vec<Statement>) -> String {
        match self {
            Self::Json => serde_json::to_string(&stmts).unwrap(),
            Self::Plain => stmts
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The DDL statements to analyze. If not provided or is -, read from
    /// standard input.
    #[arg(default_value_t)]
    pub input: InputArg,

    /// The database to connect to
    #[arg(long, value_name = "postgres connection string")]
    pub db: String,

    /// The output format of the analysis
    #[arg(short, long = "format", value_enum, default_value_t = Formatter::Plain)]
    pub formatter: Formatter,

    /// Execute each statement in its own transaction.
    /// By default all statements are executed in a single transaction.
    /// Implies --commit
    #[arg(long, default_value_t = false)]
    pub distinct_transactions: bool,

    /// Commit the transactions. By default they are rolled back.
    #[arg(long, default_value_t = false)]
    pub commit: bool,
}
