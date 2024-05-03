use crate::pcf::PcfFile;
use crate::yosys_parser::{GalOLMC, GalSop, Graph, NamedPort, Net, Node, NodeIdx, PortDirection};
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

    #[error("Could not find a sop to fit SOP {0:?} of {1}")]
    SopTooBig(String, usize),

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

fn map_remaining_olmc(
    graph: &Graph,
    olmc: NodeIdx,
    unused: &Vec<(usize, usize)>,
) -> Result<(usize, usize), MappingError> {
    // (index, size)
    let mut chosen_row: Option<(usize, usize)> = None;
    // FIXME: implement.
    let sop = get_sop_for_olmc(graph, olmc)?;
    let sopsize: usize = sop.parameters.depth as usize;

    for (olmc_idx, size) in unused {
        match chosen_row {
            None => {
                if size > &sopsize {
                    chosen_row = Some((*olmc_idx, *size));
                }
            }
            Some(r) => {
                // we do the comparison (size > SOP Size)
                if size < &r.1 && size > &sopsize {
                    chosen_row = Some((*olmc_idx, *size));
                }
            }
        }
    }
    // at the end, if we have chosen a row, we can swap it in.
    match chosen_row {
        Some(x) => {
            info!(
                "mapping {olmc:?} size {sopsize} to row {} with size {}",
                x.0, x.1
            );
            Ok(x)
        }
        None => Err(MappingError::SopTooBig("TODO FIXME".to_string(), sopsize)),
    }
}

fn valid_inputs(chip: Chip) -> Vec<u32> {
    match chip {
        Chip::GAL16V8 => vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        Chip::GAL22V10 => vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ],
        _ => panic!("unsupported chip"),
    }
}

pub fn graph_convert(graph: &Graph, pcf: PcfFile, chip: Chip) -> anyhow::Result<Blueprint> {
    let mut bp = Blueprint::new(chip);

    // phase zero: input mapping.
    let mut pinmap: Vec<Option<String>> = vec![None; chip.num_pins()];

    let valid_inp = valid_inputs(chip);
    let mut olmcmap: Vec<Option<NodeIdx>> = vec![None; chip.num_olmcs()];

    for port in &graph.ports {
        let pin = port
            .lookup(&pcf)
            .ok_or(MappingError::MissingConstraint(port.clone()))?;
        if valid_inp.contains(&pin) {
            if let Some(olmcrow) = chip.pin_to_olmc(pin as usize) {
                if port.direction == PortDirection::Input {
                    olmcmap[olmcrow] = Some(NodeIdx(usize::MAX));
                } // otherwise we do not care!
            }
        } else {
            return Err(MappingError::Unknown.into());
        }
    }

    // phase one: OLMC mapping
    // start by finding the constraints.

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
                // FIXME: -0 to size if registered, if comb, size - 1
                info!("Found a real pin to map: Mapping node {o:?} onto row {olmc_row}");

                // check if OLMC row is already in use
                if let Some(o) = olmcmap[olmc_row] {
                    info!("already exists in {o:?}");
                    return Err(MappingError::Unknown.into());
                }
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
    let mut unused_rows = olmcmap
        .iter()
        .enumerate() // get the index
        .filter_map(|(i, x)| if x.is_none() { Some(i) } else { None }) // find the ones that are
        .map(|i| (i, chip.num_rows_for_olmc(i))) // get the size of the row
        .collect();

    // find the smallest row that fits.
    for olmc in deferrals {
        let row = map_remaining_olmc(graph, olmc, &unused_rows)?;
        // insert into the mapping
        olmcmap[row.0] = Some(olmc);
        // remove this row from the available rows
        // i.e only keep those that are not equal to this row.
        unused_rows.retain(|r| r != &row);
    }

    // at this point, we have mapped every OLMC.
    // now use the blueprint to set the settings.

    Ok(bp)
}
