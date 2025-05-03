use clap::Parser;
use pglockanalyze::analyzer::Analyzer;
use pglockanalyze::errors::Error;

mod cli;
use cli::Cli;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut analyzer = Analyzer::new(&cli.db)?;
    let statements = analyzer.analyze_many(&cli.statement)?;

    println!("{}", cli.formatter.format(statements));

    Ok(())
}
