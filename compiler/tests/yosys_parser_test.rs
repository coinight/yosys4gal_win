use ver2gal::yosys_parser::*;
use std::error::Error;
use std::fs;
use serde_json::from_slice;

#[test]
fn test_load() -> Result<(), Box<dyn Error>> {
    let f = fs::read("testcases/json/synth_olmc_test.json")?;

    let data: YosysDoc = from_slice(f.as_slice())?;
    println!("{:?}", data);

    Ok(())
}

#[test]
fn test_graph() -> Result<(), Box<dyn Error>> {
    let f = fs::read("testcases/json/synth_olmc_test.json")?;

    let data: YosysDoc = from_slice(f.as_slice())?;

    let g = Graph::from(data);
    println!("{:?}", g);
    g.validate()?;
    Ok(())
}
