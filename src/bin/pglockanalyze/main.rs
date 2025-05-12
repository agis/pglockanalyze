use clap::Parser;
use pglockanalyze::analyzer::Analyzer;
use std::error::Error;
use std::process::exit;

mod cli;
use cli::Cli;

fn main() {
    let mut cli = Cli::parse();
    if cli.distinct_transactions {
        cli.commit = true;
    }

    let input = &cli.input.read_to_string().unwrap_or_else(abort);

    let analyzer =
        Analyzer::new(&cli.db, cli.distinct_transactions, cli.commit).unwrap_or_else(abort);
    let analysis = analyzer.analyze(input).unwrap_or_else(abort);
    let output = cli.formatter.format(analysis);

    println!("{output}");
}

fn abort<T>(e: impl Error) -> T {
    eprintln!("{e}");
    exit(1)
}
