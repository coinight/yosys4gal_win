pub mod pcf;
pub mod yosys_parser;

use clap::{Parser, Subcommand, Args};
use crate::yosys_parser::{YosysDoc, Graph};
use anyhow::{bail, Result};
use serde_json::from_slice;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a yosys netlist JSON file.
    Validate(ValidateArgs),
}

#[derive(Args)]
struct ValidateArgs {
    #[arg(required = true, value_hint = clap::ValueHint::DirPath)]
    file: PathBuf,
}


fn validate(v: ValidateArgs) -> Result<()>{
    let f = fs::read(v.file)?;

    let data: YosysDoc = from_slice(f.as_slice())?;

    let g = Graph::from(data);
    let res = g.validate().map_err(|x| x.to_string());
    if let Err(e) = res {
        bail!(e);
    }
    println!("Validation Complete!");
    println!("Stats:");
    println!("Nodes: {}", g.nodelist.len());
    println!("Edges: {}", g.adjlist.len());
    Ok(())
}

fn main() -> Result<()>{
    let args = Cli::parse();
    match args.command {
        Commands::Validate(v) => validate(v),
    }
}
