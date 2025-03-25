Techmaps
========

All the yosys techmapping libraries/Verilog files used for mapping to the GAL
structure. See the `synth_gal.tcl` file for details on how they're used.
A summary is below (in the order they're used):

- `gal_dff.lib` Liberty library for supported FFs (only positive edge-triggered
  DFFs). Used to prevent yosys from using fancy flip-flops
- `pla.v` splits SOPs into a chain of SOPs with a specified size
- `trivial_sop.v` replaces SOPs which are just buffers/NOT gates with
  buffer/NOT cells
- `olmc_seq.v` converts all existing sequential elements into GAL_OLMCs this
  includes special merged types from other techmap passes/extractions
- `one_sop.v` converts GAL_SOPs with only one product into GAL_1SOPs. Used on
  enabled (tristate) lines
- `pla_olmc_int.v` adds a combinational GAL_OLMC after a GAL_SOP. Used to
  insert GAL_OLMC between GAL_SOPs or enable lines
- `olmc_comb.v` creates GAL_OLMCs for combinational/tristate output pads
- `trivial_sop_olmc.v` adds a buffer GAL_SOP before a GAL_OLMC. Used to add
  GAL_SOPs between directly connect GAL_OLMCs/GAL_INPUTs
- `trivial_1sop_olmc.v` same as above but for enable lines
