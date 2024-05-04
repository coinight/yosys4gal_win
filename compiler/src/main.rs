pub mod pcf;
pub mod yosys_parser;
mod fitter;

use clap::{Parser, Subcommand, Args};
use crate::pcf::parse_pcf;
use crate::yosys_parser::{YosysDoc, Graph};
use crate::fitter::graph_convert;
use anyhow::{bail, Result};
use serde_json::from_slice;
use std::path::PathBuf;
use galette::chips::Chip;
use std::fs;
use env_logger;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a yosys netlist JSON file.
    Validate(ValidateArgs),
    Synth(SynthArgs),
}

#[derive(Args)]
struct ValidateArgs {
    #[arg(required = true, value_hint = clap::ValueHint::DirPath)]
    file: PathBuf,
}


#[derive(Args)]
struct SynthArgs {
    #[arg(required = true, value_hint = clap::ValueHint::DirPath)]
    netlist: PathBuf,
    #[arg(required = true, value_hint = clap::ValueHint::DirPath)]
    constraints: PathBuf,
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


fn synth(s: SynthArgs) -> Result<()> {
    let f = fs::read(s.netlist)?;

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

    // load the pcf
    let pcf_file = &fs::read(s.constraints)?;
    let pcf_string = std::str::from_utf8(pcf_file)?;
    let pcf = parse_pcf(pcf_string);

    let res = graph_convert(&g, pcf, Chip::GAL16V8)?;
    
    Ok(())
}

fn main() -> Result<()>{
    let args = Cli::parse();
    env_logger::init();
    match args.command {
        Commands::Validate(v) => validate(v),
        Commands::Synth(s) => synth(s),
    }
}
