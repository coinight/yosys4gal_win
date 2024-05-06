# `ver2gal`: Yosys Netlist-to-JEDEC compiler

This program converts a Yosys JSON netlist that has been techmapped with the GAL techlib to a JEDEC fuse file to be flashed to hardware.

## Build and Install

You'll need a relatively recent version of Rust and `cargo`, the package manager. If the version included in your distribution is too old, install `rustup` to install
a recent stable release.

Build with `cargo build`. This will produce a `ver2gal` binary in `target/debug`. The program can be installed globally with `cargo install --path .`.

## Usage

The primary mode is the `synth` subcommand:
```
Usage: ver2gal synth [OPTIONS] <NETLIST> <CONSTRAINTS>

Arguments:
  <NETLIST>
  <CONSTRAINTS>

Options:
      --chip <CHIP>  [default: gal16v8] [possible values: gal16v8, gal22v10]
  -h, --help         Print help
```

When provided a netlist JSON file and a PCF constraints file it will produce a `.jed` fuse file called `output.jed`.

**Important Note**: This program should be run in the same directory as the `shrink_sop.tcl` script, as it will automatically call the script
if a SOP needs to be split. This should only be necessary in the `gal22v10` mode. 
