#!/usr/bin/env -S yosys -c
yosys -import

if { $argc != 1 } {
	puts "USAGE: $argv0 -- <VERILOG FILE>"
	exit
}

exec rm -rf output
exec mkdir output

## Read Verilog
read_verilog [lindex $argv 0]
hierarchy -auto-top

## First pass synthesis
synth
design -save preop

## DFF/SOP mapping
dfflibmap -liberty techmaps/gal_dff.lib

# Get count of non-clock inputs and registers (TODO make finding clk more robust)
set num_inputs_regs [regexp -inline {\d+} [tee -s result.string select -count t:DFF_P i:* */clk %d]]

abc -sop -I $num_inputs_regs -P 256
#abc -sop -I 8 -P 256
opt
clean -purge

## Tech mapping
techmap -map techmaps/pla.v -D PLA_MAX_PRODUCTS=8
extract -map extractions/ndff.v

clean -purge

## Write output files and graph
write_verilog output/synth.v
write_json output/synth.json
write_table output/synth.txt
write_blif output/synth.blif
write_rtlil output/synth.rtlil

show -width -signed -enum

## Verify equivalence
# Backup and make gold and gate modules
design -stash postop
design -copy-from preop -as gold A:top
design -copy-from postop -as gate A:top

# Reverse tech map into primatives
#chtype -map GAL_SOP $sop gate
#chtype -map DFF_P $_DFF_P_
techmap -map techmaps/inv_techmap.v

# Verify
equiv_make gold gate equiv
equiv_induct equiv
equiv_simple equiv

# Restore backup
design -load postop

## Print final stats
ltp -noff
stat

shell
