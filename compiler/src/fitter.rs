use std::str::from_utf8;

use crate::pcf::PcfFile;
use crate::yosys_parser::{GalInput, GalSop, Graph, NamedPort, Net, Node, NodeIdx, PortDirection};
use galette::blueprint::Blueprint;
use galette::chips::Chip;
use log::{debug, info, warn};
use thiserror::Error;

use galette::gal::{Pin, Term};

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
fn get_sop_for_olmc(graph: &Graph, olmc_idx: &NodeIdx) -> Result<GalSop, MappingError> {
    let input = graph.get_node_port_conns(olmc_idx, "A");
    let sops_on_net: Vec<_> = input
        .iter()
        .filter_map(|i| {
            let sop = i.get_other(olmc_idx)?;
            if sop.1 != "Y" {
                return None;
            };
            let node = graph.get_node(&sop.0)?;
            match node {
                Node::Sop(s) => Some(s),
                _ => None,
            }
        })
        .collect();
    assert_eq!(sops_on_net.len(), 1, "Should only be one sop driving a net");
    Ok(sops_on_net[0].clone())
    // let other_node = input[0].get_other(olmc_idx).ok_or(MappingError::Unknown)?;
    // let sop = graph
    //     .get_node(&other_node.0)
    //     .ok_or(MappingError::MissingSOP)?;
    // if let Node::Sop(s) = sop {
    //     Ok(s.clone())
    // } else {
    //     Err(MappingError::MissingSOP)
    // }
}

fn map_remaining_olmc(
    graph: &Graph,
    olmc: NodeIdx,
    unused: &Vec<(usize, usize)>,
) -> Result<(usize, usize), MappingError> {
    // (index, size)
    let mut chosen_row: Option<(usize, usize)> = None;
    // FIXME: implement.
    let sop = get_sop_for_olmc(graph, &olmc)?;
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

fn find_hwpin_for_net(graph: &Graph, pcf: &PcfFile, net: &Net) -> Result<u32, MappingError> {
    // this does a double lookup. first it finds the Input on the net,
    // then it finds the port on the input of the GAL_INPUT.
    // find the input on the net.
    let inputs: Vec<&GalInput> = graph
        .find_nodes_on_net(net)
        .iter()
        .filter_map(|n| match graph.get_node(n) {
            Some(Node::Input(i)) => Some(i),
            _ => None,
        })
        .collect();

    // now we have an array of inputs, this should be one elemnt.
    if inputs.len() != 1 {
        return Err(MappingError::Unknown);
    }

    let port_nets = inputs[0]
        .connections
        .get("A")
        .ok_or(MappingError::Unknown)?;
    assert_eq!(port_nets.len(), 1, "should only be one input to GAL_INPUT");
    let pnet = &port_nets[0];

    if let Some(p) = graph.find_port(&pnet) {
        info!("Found a port after traversing inputs");
        // look up the pin.
        p.lookup(pcf)
            .ok_or(MappingError::MissingConstraint(p.clone()))
    } else {
        Err(MappingError::Unknown)
    }
}
/// Takes a gal sop, and turns it into a vec of mapped pins.
fn make_term_from_sop(graph: &Graph, sop: GalSop, pcf: &PcfFile) -> Term {
    let table = sop.parameters.table.as_bytes();

    let n_products = sop.parameters.depth;
    let product_size = sop.parameters.width;
    let chunksize = product_size * 2; // 00 for dontcare, 01 for negation, 10 for positive i think

    let input_nets = sop.connections.get("A").unwrap();

    let terms: Vec<Vec<Pin>> = table
        .chunks(chunksize as usize)
        .map(|chunk| {
            // chunk is now a block of terms.
            let terms: Vec<&str> = chunk.chunks(2).map(|c| from_utf8(c).unwrap()).collect();
            // create our term
            let pins: Vec<Pin> = terms
                .iter()
                .enumerate()
                .filter_map(|(idx, p)| {
                    let net_for_pin = input_nets.get(idx).unwrap();
                    // now use the helper to find the true hardware pin
                    let hwpin: usize =
                        find_hwpin_for_net(graph, pcf, net_for_pin).unwrap() as usize;
                    // we now have our hardware pin number!
                    match *p {
                        "01" => Some(Pin {
                            pin: hwpin,
                            neg: true,
                        }),
                        "10" => Some(Pin {
                            pin: hwpin,
                            neg: false,
                        }),
                        _ => None,
                    }
                })
                .collect();
            pins
        })
        .collect();
    assert_eq!(n_products as usize, terms.len());
    Term {
        line_num: 0,
        pins: terms,
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
                } // otherwise we do not care at this point!
            }
        } else {
            // we don't have a constraint for this port
            return Err(MappingError::MissingConstraint(port.clone()).into());
        }
    }

    debug!("Graph adj list is {:?}", graph.adjlist);

    // phase one: OLMC mapping
    // start by finding the constraints.

    let mut deferrals: Vec<NodeIdx> = Vec::new();

    // For all the OLMCs in the graph, we either map it directly since it's constrained to a pin,
    // or we defer it to later.
    for o in graph.get_olmc_idx() {
        debug!("Processing OLMC {o}");
        debug!("Value = {:?}", graph.get_node(&o));
        // find all the

        let n: &Net;
        if let Some(Node::Olmc(olmc)) = graph.get_node(&o) {
            n = &olmc.connections.get("Y").ok_or(MappingError::Unknown)?[0];
        } else {
            warn!("Could not find output net! Silently skipping");
            continue;
        }

        // if it's got a port we map it now else we defer it.

        let port = graph.find_port(n);

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

    // to map the deferred ones, we need to find the smallest SOP that is still large enough for
    // it.
    // Vec<(olmc_index,size)>
    let mut unused_rows = olmcmap
        .iter()
        .enumerate() // get the index
        .filter_map(|(i, x)| if x.is_none() { Some(i) } else { None }) // find the ones that are
        .map(|i| (i, chip.num_rows_for_olmc(i))) // get the size of the row
        .collect();

    // find the smallest row that fits.
    info!("Starting deferred mapping process");
    for olmc in deferrals {
        let row = map_remaining_olmc(graph, olmc, &unused_rows)?;
        debug!("Found a mapping for {olmc} in row {} size {}", row.0, row.1);
        // insert into the mapping
        olmcmap[row.0] = Some(olmc);
        // remove this row from the available rows
        // i.e only keep those that are not equal to this row.
        unused_rows.retain(|r| r != &row);
    }

    // at this point, we have mapped every OLMC.
    // find the SOPs and for each sop, find
    info!("Deferred mapping complete, starting SOP mapping");
    for (idx, olmc) in olmcmap.iter().enumerate() {
        match olmc {
            Some(node) => {
                debug!("Mapping node {node} at row {idx}");
                let sop = get_sop_for_olmc(graph, node)?;
                debug!("Got SOP {:?} attached to node", sop);
                let term = make_term_from_sop(graph, sop, &pcf);
                debug!("Got term {:?}", term);
            }
            None => {}
        }
    }

    Ok(bp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_sop_to_term() -> Result<()> {
        let pct = "set_io pinName 1";
        Ok(())
    }
}
