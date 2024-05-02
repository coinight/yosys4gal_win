use crate::pcf::PcfFile;
use crate::yosys_parser::{GalOLMC, GalSop, Graph, NamedPort, Net, Node, NodeIdx};
use galette::blueprint::Blueprint;
use galette::chips::Chip;
use log::info;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("OLMC missing output: {0}")]
    OLMCMissingOutput(String),

    #[error("Could not find constraint for port {}", .0.name)]
    MissingConstraint(NamedPort),

    #[error("Could not find the SOP input")]
    MissingSOP,

    #[error("Could not find a sop to fit {0}")]
    SopTooBig(usize),

    #[error("Unknown error")]
    Unknown,
}

// attempt to map graph into blueprint

/// Acquire the SOP associated with the OLMC. If it's 
fn get_sop_for_olmc(graph: &Graph, olmc_idx: NodeIdx) -> Result<GalSop, MappingError> {
    let input = graph.get_node_port_conns(olmc_idx, "A");
    assert_eq!(input.len(), 1, "OLMC input should have one netadjpair");
    let other_node = input[0].get_other(olmc_idx).ok_or(MappingError::Unknown)?;
    let sop = graph
        .get_node(other_node.0)
        .ok_or(MappingError::MissingSOP)?;
    if let Node::Sop(s) = sop {
        Ok(s.clone())
    } else {
        Err(MappingError::MissingSOP)
    }
}

fn map_remaining_olcm(graph: &Graph, olmc: NodeIdx, unused: Vec<(usize, usize)>) -> Result<usize, MappingError> {
    // (index, size)
    let mut chosen_row: Option<(usize, usize)> = None;
    // FIXME: implement.
    let sopsize: usize = get_sop_for_olmc(graph, olmc)?.parameters.depth as usize;

    for (olmc_idx, size) in unused {
        match chosen_row {
            None => {
                if size > sopsize {
                    chosen_row = Some((olmc_idx, size));
                }
            }
            Some(r) => {
                // we do the comparison (size > SOP Size)
                if size < r.1 && size > sopsize {
                    chosen_row = Some((olmc_idx, size));
                }
            }
        }
    }
    // at the end, if we have chosen a row, we can swap it in.
    match chosen_row {
        Some((row, size)) => {
            info!("mapping {olmc:?} size {sopsize} to row {row} with size {size}");
            Ok(row)
        },
        None => {
            Err(MappingError::SopTooBig(sopsize))
        }
    } 
}


pub fn graph_convert(graph: &Graph, pcf: PcfFile, chip: Chip) -> anyhow::Result<Blueprint> {
    let mut bp = Blueprint::new(chip);

    // phase one: OLMC mapping
    // start by finding the constraints.
    //
    let mut olmcmap: Vec<Option<NodeIdx>> = vec![None; chip.num_olmcs()];

    let mut deferrals: Vec<NodeIdx> = Vec::new();

    // For all the OLMCs in the graph, we either map it directly since it's constrained to a pin,
    // or we defer it to later.
    for o in graph.get_olmc_idx() {
        // find all the
        let others: Vec<Net> = graph
            .get_node_port_conns(o, "Y")
            .iter()
            .map(|adj| adj.net.clone())
            .collect();

        // if it's got a port we map it now else we defer it.

        let port = graph.find_port(&others[0]);

        match port {
            Some(port) => {
                info!("Found a port, performing port lookup");
                let pin = port
                    .lookup(&pcf)
                    .ok_or(MappingError::MissingConstraint(port.clone()))?;
                let olmc_row = chip
                    .pin_to_olmc(pin.try_into()?)
                    .ok_or(MappingError::Unknown)?;
                // TODO: check size of row vs size of SOP
                info!("Found a real pin to map: Mapping node {o:?} onto row {olmc_row}");
                olmcmap[olmc_row] = Some(o);
            }
            None => {
                info!("No port found, deferring placement for {o:?}");
                deferrals.push(o)
            }
        }
    }
    // at this point, we should have mapped
    let num_mapped = olmcmap.iter().filter(|x| x.is_some()).count();
    info!("Mapped {num_mapped} OLMCS, {} deferred", deferrals.len());

    // to map remainders, we need to find the smallest SOP.
    //
    // Vec<(olmc_index,size)>
    let unused_rows = olmcmap
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if !x.is_some() { Some(i) } else { None })
        .map(|i| (i, chip.num_rows_for_olmc(i)));
    // find the smallest row that fits.

    let olmc = deferrals[0];

    Ok(bp)
}
