#!/usr/bin/env -S yosys -c
yosys -import

read_verilog original.v
flatten
synth

read_verilog wrapper.v GAL16V8_reg.v
flatten
synth

equiv_make original wrapper equiv
equiv_induct equiv
equiv_status -assert equiv
