#!/usr/bin/env -S yosys -c
yosys -import

if { $argc != 1 } {
	puts "USAGE: $argv0 -- <VERILOG FILE>"
	exit
}

set fbasename [file rootname [file tail [lindex $argv 0]]]
puts $fbasename

exec rm -rf output
exec mkdir output

## Read Verilog/Liberty file
read_verilog [lindex $argv 0]
hierarchy -auto-top
read_verilog -lib cells_sim.v
read_liberty -lib techmaps/gal_dff.lib

## First pass synthesis
tribuf
synth
design -save preop

# Map IO pins (and undo port removal for the output)
iopadmap -bits -inpad GAL_INPUT Y:A -toutpad GAL_OUTPUT E:A:Y -outpad GAL_OUTPUT A
expose */t:GAL_OUTPUT "%x:+\[A\]" */t:GAL_OUTPUT %d

## DFF/SOP mapping
dfflibmap -liberty techmaps/gal_dff.lib

# Get count of non-clock inputs and registers
set num_inputs [regexp -inline {\d+} [tee -s result.string select -count t:GAL_INPUT]]
set num_regs [regexp -inline {\d+} [tee -s result.string select -count t:DFF_P]]
set num_inputs_regs [expr $num_inputs + $num_regs]
if {$num_regs > 0} { set num_inputs_regs [expr $num_inputs_regs - 1] }

#abc -sop -I $num_inputs_regs -P 256
#abc -sop -I 8 -P 8
#abc -script "+strash;,dretime;,collapse;,write_pla,test.pla" -sop
# Force one-level SOP
abc -script "abc.script" -sop

# Resynth all too big SOPs together in multi-level SOP
#select "t:\$sop" r:DEPTH>8 %i
#techmap -autoproc -map sop.v
#yosys proc
#techmap
#select *
#abc -sop -I $num_inputs_regs -P 8

opt
clean -purge

show -width

## Tech mapping
# Logic
techmap -map techmaps/pla.v -D PLA_MAX_PRODUCTS=10000

# Sequential OLMC 
extract -map extractions/ndff.v
extract -constports -map extractions/olmc.v
techmap -map techmaps/olmc_seq.v

# Combinational OLMC
iopadmap -bits -outpad GAL_COMB_OUTPUT_P A:Y */t:GAL_SOP "%x:+\[Y\]" */t:GAL_SOP %d o:* %i
techmap -map techmaps/olmc_comb.v o:* %x o:* %d

clean -purge

## Write output files and graph
write_verilog "output/synth_${fbasename}.v"
write_json "output/synth_${fbasename}.json"
write_table "output/synth_${fbasename}.txt"
write_blif "output/synth_${fbasename}.blif"
write_rtlil "output/synth_${fbasename}.rtlil"

## Verify equivalence
# Backup and make gold and gate modules
design -stash postop
design -copy-from preop -as gold A:top
design -copy-from postop -as gate A:top

# Inverse tech map into primatives
techmap -autoproc -map cells_sim.v -autoproc
clean -purge

# Verify
equiv_make gold gate equiv
tribuf -formal equiv
equiv_induct equiv
equiv_status -assert equiv

# Get LTP from inverse tech map so FF cells are recognized
ltp -noff

# Restore backup
design -load postop

## Print final stats
show -width -signed -enum

stat

shell
