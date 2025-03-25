Extractions
===========

All the yosys extraction pass Verilog file used for mapping to the GAL
structure. See the `synth_gal.tcl` file for details on how they're used. A
summary is below (in the order they're used):

- `ndff.v` merges NOT gates and DFFs into a single cell. Used to set the active
  high/low state of the GAL_OLMCs later
- `tristate.v` merges tristate cells and NOT gates or tristate cells and DFFs.
  Used to combine tristate functionality into the GAL_OLMCs later
