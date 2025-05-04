use clap::Parser;
use pglockanalyze::analyzer::Analyzer;
use pglockanalyze::errors::Error;
use std::process::exit;

mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    let mut analyzer = Analyzer::new(&cli.db).unwrap_or_else(abort);
    let statements = analyzer.analyze_many(&cli.statement).unwrap_or_else(abort);

    println!("{}", cli.formatter.format(statements));
}

fn abort<T>(e: Error) -> T {
    eprintln!("{e}");
    exit(1)
}
