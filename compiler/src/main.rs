mod fitter;
pub mod pcf;
pub mod yosys_parser;

use crate::fitter::{graph_convert, MappingError};
use crate::pcf::{parse_pcf, PcfFile};
use crate::yosys_parser::{Graph, YosysDoc};
use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use env_logger;
use galette::blueprint::Blueprint;
use galette::chips::Chip;
use galette::gal_builder::build;
use galette::writer::{make_jedec, Config};
use serde_json::from_slice;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use log::{info, warn};

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

#[derive(ValueEnum, Debug, Clone)]
enum ChipType {
    GAL16V8,
    GAL22V10,
}

impl ChipType {
    fn to_galette(&self) -> Chip {
        match self {
            Self::GAL16V8 => Chip::GAL16V8,
            Self::GAL22V10 => Chip::GAL22V10,
        }
    }
}

#[derive(Args)]
struct SynthArgs {
    #[arg(required = true, value_hint = clap::ValueHint::DirPath)]
    netlist: PathBuf,
    #[arg(required = true, value_hint = clap::ValueHint::DirPath)]
    constraints: PathBuf,

    #[arg(value_enum, long, default_value_t=ChipType::GAL16V8)]
    chip: ChipType,
}

fn validate(v: ValidateArgs) -> Result<()> {
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

fn load_to_graph(
    netlist: &PathBuf,
    pcf: &PcfFile,
    chip: Chip,
) -> Result<Blueprint, MappingError> {
    info!("loading netlist...");
    let f = fs::read(netlist).unwrap();

    let data: YosysDoc = from_slice(f.as_slice()).unwrap();

    let g = Graph::from(data);
    g.validate().map_err(|x| x.to_string()).unwrap();
    println!("Validation Complete!");
    println!("Stats:");
    println!("Nodes: {}", g.nodelist.len());
    println!("Edges: {}", g.adjlist.len());
    graph_convert(&g, pcf, chip)
}

fn synth(s: SynthArgs) -> Result<()> {

    // load the pcf
    let pcf_file = &fs::read(s.constraints)?;
    let pcf_string = std::str::from_utf8(pcf_file)?;
    let pcf = parse_pcf(pcf_string);

    let mut res = load_to_graph(&s.netlist, &pcf, s.chip.to_galette());

    while let Err(MappingError::SopTooBig { ref name, sop_size, wanted_size }) = res {
        warn!("Sop too large, attempting to split {name}. cur={sop_size} want={wanted_size}");
        let yosys = Command::new("yosys").args(["split_sop.tcl"]);

        res = load_to_graph(&s.netlist, &pcf, s.chip.to_galette());
    }

    let bp = res?;

    let mut gal = build(&bp)?;

    if matches!(s.chip, ChipType::GAL16V8) {
        gal.set_mode(galette::gal::Mode::Registered);
    }

    let config = Config {
        gen_pin: false,
        gen_fuse: false,
        gen_chip: false,
        jedec_sec_bit: false,
    };

    let mut file = File::create("output.jed")?;
    let jed = make_jedec(&config, &gal);

    file.write_all(jed.as_bytes())?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::init();
    match args.command {
        Commands::Validate(v) => validate(v),
        Commands::Synth(s) => synth(s),
    }
}
