_yosys4gal_: Verilog Flow for the GAL16V8 and GAL22V10
======================================================

A Verilog flow for GAL16V8 and GAL22V10 logic chips
(and pin-compatible alternatives like the ATF16V8 and ATF22V10).
It leverages [Yosys](https://www.github.com/YosysHQ/yosys)
and [Galette](https://www.github.com/simon-frankau/galette).

Usage
-----
In Windows
------
1.install oss-cad-suite  
2.copy folder "GAL_LIB" to "oss-cad-suite" folder inside  
3.open start.bat  
4.in the shell window:Input below  
```
yosys -c .\GAL_LIB\synth_gal.tcl -- {YourFile.V}  
#eg. yosys -c .\GAL_LIB\synth_gal.tcl C:\Users\admin\Downloads\yosys4gal-master\testcases\test.v  
```
Old : Fork From yosys4gal only on linux 
------
To synthesize a Verilog file:
```
./synth_gal.tcl -- <VERILOG_FILE> [CHIP]
```
Where `[CHIP]` is either `GAL16V8` (default) or `GAL22V10`. The synthesized
JSON netlist will be put in `output/`.

To fit the synthesized design and generate the JEDEC file used for programming,
first build the Rust compiler `ver2gal` (see the `compiler/` directory) and run:
```
./ver2gal synth <JSON NETLIST> <PCF_CONSTRAINTS> --chip <CHIP>
```
Where `<CHIP>` is either `gal16v8` or `gal22v10`. The generate JEDEC file will
be generated in the current directory as `output.jed`. Note this program _must_
be run in the same directory as the `shrink_sop.tcl` script.

This JEDEC file can be optionally be verified programmatically using the
scripts and Verilog models found the `models/` directory. 

The JEDEC file can then be flashed to the GAL chips. A convenience flashing
script is provided for the cheap/common TL866 family of programmers. This works
around the verification bugs in the upstream `minipro` programming software:
```
./flash_minipro.sh <JEDEC FILE> <CHIP>
```
Where `<CHIP>` is `GAL16V8`, `GAL22V10`, `ATF16V8B`, etc.

Limitations
-----------
The GAL16V8 mode only supports the "Registered" mode and does not handle
tristate Verilog for registered outputs (since they're globally shared). The
GAL22V10 mode does not support the asynchronous set/reset signals for the
registers. Additionally, in both modes, there is no guarantee that the mapping
will be the most efficient (especially for timing). While fairly well tested,
there is no guarantee of correctness either. Use at your own risk.

Dependencies
------------
- `yosys` 0.38 or higher
- Rust
- `jedutil` from MAME utilities for model-based verification checking
- `xxd` from Vim for model-based verification checking
- `minipro` for the provided convenience flashing script
