use clap::{Parser, ValueEnum};
use pglockanalyze::analyzer::Analyzer;
use pglockanalyze::errors::Error;
use pglockanalyze::statement::Statement;

#[derive(Debug, Clone, ValueEnum)]
enum Formatter {
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
struct Cli {
    /// The DDL statement to be analyzed
    statement: String,

    /// The database to connect to
    #[arg(long, value_name = "postgres connection string")]
    db: String,

    #[arg(value_enum, long = "format", default_value_t = Formatter::Plain)]
    formatter: Formatter,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut analyzer = Analyzer::new(&cli.db)?;
    let statements = analyzer.analyze_many(&cli.statement)?;

    println!("{}", cli.formatter.format(statements));

    Ok(())
}
