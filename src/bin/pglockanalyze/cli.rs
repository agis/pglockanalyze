use clap::{Parser, ValueEnum};
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
    /// The DDL statement to be analyzed
    pub statement: String,

    /// The database to connect to
    #[arg(long, value_name = "postgres connection string")]
    pub db: String,

    #[arg(value_enum, long = "format", default_value_t = Formatter::Plain)]
    pub formatter: Formatter,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}
