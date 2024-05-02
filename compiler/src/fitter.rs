use crate::pcf::PcfFile;
use crate::yosys_parser::{GalOLMC, Graph, NamedPort, Net};
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

    #[error("Unknown error")]
    Unknown,
}

// attempt to map graph into blueprint

#[derive(Debug, Clone)]
struct GALMapEntry(GalOLMC, usize); // data and the entry in the graph

pub fn graph_convert(graph: &Graph, pcf: PcfFile, chip: Chip) -> anyhow::Result<Blueprint> {
    let mut bp = Blueprint::new(chip);

    // phase one: OLMC mapping
    // start by finding the constraints.
    //
    let mut olmcmap: Vec<Option<usize>> = vec![None; chip.num_olmcs()];

    let mut deferrals: Vec<usize> = Vec::new();

    for o in graph.get_olmc_idx() {
        // find all the nodes named
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
                info!("Found a real pin to map: Mapping node {o} onto row {olmc_row}");
                olmcmap[olmc_row] = Some(o);
            }
            None => {
                info!("No port found, deferring placement for {o}");
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
    let unused_rows =
        olmcmap
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if !x.is_some() { Some(i) } else { None })
            .map(|i| (i, chip.num_rows_for_olmc(i)));
    // find the smallest row that fits.

    let olmc = deferrals[0];
    let mut chosen_row: Option<(usize, usize)> = None;
    // FIXME: implement.
    let sopsize = 0;

    for (olmc_idx, size) in unused_rows {

        match chosen_row {
            None => {
                if size > sopsize {
                    chosen_row = Some((olmc_idx, size));
                }
            },
            Some(r) => {
                // we do the comparison (size > SOP Size)
                if size < r.1 && size > sopsize {
                    chosen_row = Some((olmc_idx, size));
                }
            }
        }
    }



    Ok(bp)
}
