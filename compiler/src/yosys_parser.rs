use crate::pcf::PcfFile;
use galette::gal::Pin;
use log::info;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, BoolFromInt};
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Debug, Serialize, Clone, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Net {
    #[serde(rename = "x")]
    NotConnected,
    #[serde(rename = "1")]
    LiteralOne,
    #[serde(rename = "0")]
    LiteralZero,
    #[serde(untagged)]
    N(u32),
}

/// The GAL_INPUT marks an external ipnut
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GalInput {
    pub connections: HashMap<String, Vec<Net>>,
}

// Custom deserializer for the strange string of binary
// for some reason the json outputs 00000000000111 for some numbers.
// this converts them back into binary.
fn from_binstr<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u32::from_str_radix(s.as_str(), 2).map_err(D::Error::custom)
}
fn bool_from_binstr<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let val = u32::from_str_radix(s.as_str(), 2).map_err(D::Error::custom)?;
    Ok(val == 1)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct GalSopParameters {
    #[serde(deserialize_with = "from_binstr")]
    pub depth: u32,
    pub table: String,
    #[serde(deserialize_with = "from_binstr")]
    pub width: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GalSop {
    pub connections: HashMap<String, Vec<Net>>,
    pub parameters: GalSopParameters,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct GALOLMCParameters {
    #[serde(deserialize_with = "bool_from_binstr")]
    pub inverted: bool,
    #[serde(deserialize_with = "bool_from_binstr")]
    pub registered: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GalOLMC {
    pub parameters: GALOLMCParameters,
    pub connections: HashMap<String, Vec<Net>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum YosysCell {
    #[serde(rename = "GAL_SOP", alias = "GAL_1SOP")]
    Sop(GalSop),
    #[serde(rename = "GAL_INPUT")]
    Input(GalInput),
    #[serde(rename = "GAL_OLMC")]
    OLMC(GalOLMC),
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModPort {
    pub bits: Vec<Net>,
    #[serde(default)]
    #[serde_as(as = "BoolFromInt")]
    pub upto: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "direction")]
pub enum Port {
    Input(ModPort),
    Output(ModPort),
    InOut(ModPort),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    pub ports: HashMap<String, Port>,
    pub cells: HashMap<String, YosysCell>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YosysDoc {
    pub creator: String,
    pub modules: HashMap<String, Module>,
}

impl GalSop {
    // extract the logic table using the parameters.
    // pub fn parse_table(self) -> Vec<Vec<Pin>> {
    //     // get the list of inputs which is a Vec<u32>
    //     let table = self.parameters.table.as_bytes();
    //
    //     // split into products
    //     let product_count = self.parameters.depth;
    //     let product_size = self.parameters.width;
    //     // we need to have enough elements.
    //     assert!(table.len() == (product_count * product_size * 2) as usize);
    //
    //     let chunksize = product_size * 2;
    //
    //     table
    //         .chunks(chunksize as usize)
    //         .map(|prod| {
    //             let mut term = Vec::new();
    //             for i in 0..product_size {
    //                 let i = i as usize;
    //                 let pin_num = self.connections.inputs.get(i).unwrap(); // this should never panic.
    //                 let pin = table_seg_to_pin(pin_num, &prod[2 * i..2 * i + 1]);
    //                 if let Some(p) = pin {
    //                     term.push(p);
    //                 }
    //             }
    //             // now that we've accumulated all of the products, add it to the products list.
    //             term
    //         })
    //         .collect()
    // }
}

// table_seg_to_pin reads the small chunk for a specific input to a
// sop and creates a gal pin for it.
fn table_seg_to_pin(num: &Net, seg: &[u8]) -> Option<Pin> {
    let Net::N(p) = num else {
        todo!("specials");
    };
    let pin_num: usize = *p as usize;
    // convert to string for dumb reasons.
    match str::from_utf8(seg).unwrap() {
        "01" => Some(Pin {
            pin: pin_num,
            neg: true,
        }), // negated
        "10" => Some(Pin {
            pin: pin_num,
            neg: false,
        }), // normal
        _ => None,
    }
}

/* constraint mapping pipeline
 * w
 * take yosys document -> look at top-level module ports
 * use constraints pcf file to map. This creates a HashMap<Net, u32> for mapping nets to pins.
 * only do this for *ports*, not cells. now we have a net->pin map, where we know that there's
 * hits.
 */

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum PortDirection {
    Input,
    Output,
    Inout,
}
/// NamedPort is our internal representation of a port.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NamedPort {
    pub name: String,
    pub net: Net,
    pub direction: PortDirection,
}

use std::cmp::Ordering;
impl NamedPort {
    fn new(name: &str, net: &Net, dir: &PortDirection) -> Self {
        NamedPort {
            name: name.to_owned(),
            net: net.clone(),
            direction: dir.clone(),
        }
    }

    /// Takes a module port, and splits into N NamedPorts, where the name
    /// is converted to match the pcf format <name>[index] if there is more than
    /// one bit in the port
    fn new_split(base_name: &str, port: ModPort, dir: PortDirection) -> Vec<NamedPort> {
        match port.bits.len().cmp(&1) {
            Ordering::Greater => port
                .bits
                .iter()
                .enumerate()
                .map(|(idx, n)| NamedPort::new(&format!("{base_name}[{idx}]"), n, &dir))
                .collect(),
            Ordering::Equal => {
                vec![NamedPort::new(base_name, &port.bits[0], &dir)]
            }
            _ => panic!("no bits on this port!"),
        }
    }
    /// Retrieves the port mapping for this port, given a PCF file.
    pub fn lookup(&self, pcf: &PcfFile) -> Option<u32> {
        //NOTE: since NamedPort is exactly (1) pin, we always use the pin case.
        // When constructing, if we have a port with multiple bits, we split it (see `new_split`)
        pcf.pin(&self.name)
    }
}

/// NodeIdx is an index into the node list to reference a specific node.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeIdx(pub usize);

impl fmt::Display for NodeIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Nodeindex: {})", self.0)
    }
}


#[derive(Debug, Clone)]
pub struct NetAdjPair {
    pub net: Net,
    idx1: NodeIdx,
    port1: String,
    port2: String,
    idx2: NodeIdx,
}

impl PartialEq for NetAdjPair {
    // equality is non-directional, so we have to manually override.
    //
    fn eq(&self, other: &Self) -> bool {
        if self.net != other.net {
            return false;
        }
        // we know the nets are the same
        if self.port1 == other.port1
            && self.port2 == other.port2
            && self.idx1 == other.idx1
            && self.idx2 == other.idx2
        {
            return true;
        }
        // the same, but the direction is backwards. for our intents this is fine.
        if self.port1 == other.port1
            && self.port2 == other.port1
            && self.idx1 == other.idx2
            && self.idx2 == other.idx1
        {
            return true;
        }
        false
    }
}

impl Eq for NetAdjPair {}

impl NetAdjPair {
    pub fn uses_net(&self, net: &Net) -> bool {
        net == &self.net
    }
    pub fn uses_nodeport(&self, idx: &NodeIdx, port: &str) -> bool {
        (&self.idx1 == idx && self.port1 == port) || (&self.idx2 == idx && self.port2 == port)
    }
    pub fn get_other(&self, my_idx: &NodeIdx) -> Option<(NodeIdx, &str)> {
        if my_idx == &self.idx1 {
            Some((self.idx2, &self.port2))
        } else if my_idx == &self.idx2 {
            Some((self.idx1, &self.port1))
        } else {
            None
        }
    }
}

/// A Node is an entry in our graph. A node has a set of connections that can be
/// read with `get_ports`. You can also see if a node accesses a given net with
/// `port_for_net`
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Input(GalInput),
    Sop(GalSop),
    Olmc(GalOLMC),
}

impl Node {
    /* These functions are a little odd. We invert the input/output bits. CHIP inputs, are viewed
     * as outputs internally, likewise we view the outputs of the chip as inputs (to be driven
     * internally. So get_outputs returns values on input_cells, since those are driven already.
     * Likewise we reverse this for get_inputs, since the chip's outputs are our inputs.
     */
    // Returns the hashmap of String (connection name) to a list of nets.
    pub fn get_connections(&self) -> HashMap<String, Vec<Net>> {
        match self {
            Self::Olmc(ol) => ol.connections.clone(),
            Self::Input(i) => i.connections.clone(),
            Self::Sop(s) => s.connections.clone(),
        }
    }

    /// Returns the connection that contains this net, if any.
    pub fn port_for_net(&self, net: &Net) -> Option<String> {
        for (port, nets) in self.get_connections() {
            if nets.contains(net) {
                return Some(port.to_string());
            }
        }
        None
    }

    /// Get every net that this node uses.
    pub fn get_nets(&self) -> Vec<Net> {
        self.get_connections()
            .iter()
            .flat_map(|(_, nets)| nets.clone())
            .collect()
    }
}

#[derive(Default, Debug)]
pub struct Graph {
    pub nodelist: Vec<Node>,
    pub adjlist: Vec<NetAdjPair>,
    pub ports: Vec<NamedPort>,
}

// For each node, for each port, for each net
// find all other nodes that have this net.

impl Graph {
    /// re-generate the adjacency set for this graph.
    pub fn generate_adjacency(&mut self) {
        self.adjlist.clear();
        for (idx1, node1) in self.nodelist.iter().enumerate() {
            for net in node1.get_nets() {
                if !matches!(net, Net::N(_)) {
                    info!("skipping global nets");
                    continue;
                }
                let connected_nodes: Vec<_> = self
                    .nodelist
                    .iter()
                    .enumerate()
                    .filter(|&(idx2, node2)| {
                        if idx1 == idx2 {
                            return false;
                        }
                        node2.get_nets().contains(&net)
                    })
                    .collect();

                for (idx2, node2) in connected_nodes {
                    // get the port for node1
                    let node1_port = node1.port_for_net(&net).expect("how");
                    let node2_port = node2.port_for_net(&net).expect("how");
                    let adj = NetAdjPair {
                        net: net.clone(),
                        idx1: NodeIdx(idx1),
                        idx2: NodeIdx(idx2),
                        port1: node1_port,
                        port2: node2_port,
                    };
                    if !self.adjlist.contains(&adj) {
                        // if this fails, it means that the opposing pair is already present.
                        self.adjlist.push(adj);
                    }
                }
            }
        }
    }
    /// Find all nodes that are attached to this net in any way.
    /// Note that this is an expensive operation since it can't currently use the adjlist.
    pub fn find_nodes_on_net(&self, net: &Net) -> Vec<NodeIdx> {
        let mut res = Vec::new();
        for (idx, node) in self.nodelist.iter().enumerate() {
            if node.get_nets().contains(net) {
                res.push(NodeIdx(idx));
            }
        }
        res
    }

    /// Retrieve a node from the node index
    /// TODO: make a newtype for the index.
    pub fn get_node(&self, idx: &NodeIdx) -> Option<&Node> {
        self.nodelist.get(idx.0)
    }

    // find the connections from the given node/port ONLY WORKS FOR NON_PORT DEVICES.
    pub fn get_node_port_conns(&self, nodeidx: &NodeIdx, port: &str) -> Vec<&NetAdjPair> {
        self.adjlist
            .iter()
            .filter(|adj| adj.uses_nodeport(nodeidx, port))
            .collect()
    }

    // TODO: get rid of this or refactor somehow?????
    // VERY BAD
    pub fn get_olmc_idx(&self) -> Vec<NodeIdx> {
        self.nodelist
            .iter()
            .enumerate()
            .filter_map(|(idx, node)| match node {
                Node::Olmc(_) => Some(NodeIdx(idx)),
                _ => None,
            })
            .collect()
    }

    /// find the port that uses the current net, if any.
    /// Ports are the input/output of a module. They are handled separately.
    pub fn find_port(&self, net: &Net) -> Option<&NamedPort> {
        match net {
            Net::N(_) => self.ports.iter().find(|p| p.net == *net),
            _ => None,
        }
    }

    pub fn get_olmc(&self) -> Vec<&Node> {
        self.nodelist
            .iter()
            .filter_map(|node| match node {
                Node::Olmc(_) => Some(node),
                _ => None,
            })
            .collect()
    }

    /// Validate that the graph has valid invariants.
    /// This function does not guarantee a mapping, but it does mean that the output produced
    /// by the yosys script is what we expected. Mainly a tool for debugging the Yosys outputs.
    pub fn validate(&self) -> Result<(), &str> {
        info!("Checking OLMC blocks");
        let olmc = self.nodelist.iter().filter_map(|node| match node {
            Node::Olmc(o) => Some(o),
            _ => None,
        });
        let olmc_clock = olmc.filter_map(|o| o.connections.get("C"));
        let test = olmc_clock.clone().all(|v| v.len() == 1);
        if !test {
            return Err("OLMC has more than one clock input!");
        }
        // assert that all olmc C nets are either not connected or to a net
        let test = olmc_clock
            .clone()
            .flatten()
            .all(|net| matches!(net, Net::NotConnected) || matches!(net, Net::N(_)));
        if !test {
            return Err("invalid clock pin");
        }
        // for the ones connected to a net, extract the net number so we can make sure they're all
        // the same clock.
        let olmc_clocked: Vec<u32> = olmc_clock
            .clone()
            .flatten()
            .filter_map(|net| match net {
                Net::N(x) => Some(*x),
                _ => None,
            })
            .collect();
        let test = olmc_clocked.windows(2).all(|w| w[0] == w[1]);
        if !test {
            return Err("clock pin is not shared amongst all OLMCs");
        }

        Ok(())
    }
}

const TECHMAP_NAMES: [&str; 5] = ["DFF_P", "GAL_INPUT", "GAL_SOP", "GAL_OLMC", "GAL_1SOP"];

impl From<YosysDoc> for Graph {
    fn from(value: YosysDoc) -> Self {
        let mut g = Graph::default();
        for (mod_name, module) in value.modules {
            info!("Processing module {}", mod_name);
            if TECHMAP_NAMES.contains(&mod_name.as_str()) {
                info!("Skipping module as it is a techmap module");
                continue;
            }
            for (cell_name, cell) in module.cells {
                info!("Processing cell {}", cell_name);
                let newcell = match cell {
                    YosysCell::Input(d) => Node::Input(d),
                    YosysCell::Sop(s) => Node::Sop(s),
                    YosysCell::OLMC(n) => Node::Olmc(n),
                };
                g.nodelist.push(newcell);
            }
            for (port_name, port) in module.ports {
                info!("Processing port {}", port_name);
                let new_ports: Vec<NamedPort> = match port {
                    Port::Output(o) => NamedPort::new_split(&port_name, o, PortDirection::Output),
                    Port::Input(i) => NamedPort::new_split(&port_name, i, PortDirection::Input),
                    Port::InOut(io) => NamedPort::new_split(&port_name, io, PortDirection::Inout),
                };
                g.ports.extend(new_ports);
            }
        }
        g.generate_adjacency();
        g
    }
}

pub enum CellType {
    Input,
    Sop,
    OLMC,
}
type Connections = HashMap<String, Vec<Net>>;

type Parameters = HashMap<String, String>;

pub trait Cell {
    fn name(&self) -> &str;
    fn ctype(&self) -> CellType;

    fn get_connection(&self, conn: &str) -> Option<&Vec<Net>>;
    fn get_param(&self, param: &str) -> Option<&String>;
    fn nets(&self) -> Vec<Net>;
    fn uses_net(&self, net: Net) -> bool;
}

pub struct GalCell {
    name: Option<String>,
    ctype: CellType,
    connections: Connections,
    params: Parameters,
}

impl GalCell {
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or("unnamed".to_string())
    }

    pub fn get_connection(&self, conn: &str) -> Option<&Vec<Net>> {
        self.connections.get(conn)
    }

    /// Access the parameter from the cell. Note that this does not 
    /// have.
    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.params.get(param)
    }

    /// Get all of the nets that this cell uses
    pub fn nets(&self) -> Vec<&Net> {
        self.connections.iter().flat_map(|x| x.1).collect()
    }

    /// Returns true if the net is used at all by the cell
    pub fn uses_net(&self, net: &Net) -> bool {
        self.nets().contains(&net)
    }

    /// Returns the port that uses the given net, if any.
    pub fn get_port_for_net(&self, net: &Net) -> Option<&str> {
        for (port, nets) in self.connections.iter() {
            if nets.contains(net) {
                return Some(port)
            }
        }
        None
    }

    /// Returns the underlying Cell type, used to differentiate available
    /// connections
    pub fn ctype(&self) -> &CellType {
        &self.ctype
    }
}


/*
 * This graph is too general as it stands. we cannot map individual input pins
 * like we can with our output pins. Also, i have no way of finding a specific
 * node in the graph.
 */

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json::from_str;

    #[test]
    fn test_netspecial_n() -> Result<()> {
        let netstring = "23";
        let data: Net = from_str(netstring)?;
        assert_eq!(data, Net::N(23));
        Ok(())
    }
    #[test]
    fn test_netspecial_x() -> Result<()> {
        let netstring = "\"x\"";
        let data: Net = from_str(netstring)?;
        assert_eq!(data, Net::NotConnected);
        Ok(())
    }
    #[test]
    fn test_netspecial_zero() -> Result<()> {
        let netstring = "[\"0\"]";
        let data: Vec<Net> = from_str(netstring)?;
        assert_eq!(data[0], Net::LiteralZero);
        Ok(())
    }
    #[test]
    fn test_netspecial_one() -> Result<()> {
        let netstring = "\"1\"";
        let data: Net = from_str(netstring)?;
        assert_eq!(data, Net::LiteralOne);
        Ok(())
    }
}
