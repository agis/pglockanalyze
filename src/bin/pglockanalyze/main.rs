mod cli;

use clap::Parser;
use cli::Cli;
use pglockanalyze::analyzer::{Analyzer, AnalyzerConfig};
use std::error::Error;
use std::process::exit;

fn main() {
    let mut cli = Cli::parse();
    if cli.distinct_transactions {
        cli.commit = true;
    }

    let input = &cli.input.read_to_string().unwrap_or_else(abort);
    let config = AnalyzerConfig {
        db_connection_uri: cli.db,
        distinct_transactions: cli.distinct_transactions,
        commit: cli.commit,
    };

    let analyzer = Analyzer::try_from(config).unwrap_or_else(abort);
    let analysis = analyzer.analyze(input).unwrap_or_else(abort);
    let output = cli.formatter.format(analysis);

    println!("{output}");
}

fn abort<T>(e: impl Error) -> T {
    eprintln!("{e}");
    exit(1)
}
