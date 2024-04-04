#!/usr/bin/env -S yosys -c
yosys -import

set target "GAL16V8"

if {$target == "GAL16V8"} {
	set num_max_products 7
} elseif {$target == "GAL22V10"} {
	set num_max_products 11
} else {
	puts "Invalid target chip"
	exit
}

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
#abc -script "abc.script" -sop

# Resynth all too big SOPs together in multi-level SOP
#select "t:\$sop" r:DEPTH>8 %i
#techmap -autoproc -map sop.v
#yosys proc
#techmap
#select *

#abc -sop -I 100 -P $num_max_products
abc -sop -I $num_inputs_regs -P $num_max_products

opt
clean -purge

#show -width

## Tech mapping
# PLAs
techmap -map techmaps/pla.v -D PLA_MAX_PRODUCTS=$num_max_products
techmap -max_iter 1 -map techmaps/trivial_sop.v

# Sequential OLMC 
extract -constports -map extractions/ndff.v
extract -constports -map extractions/olmc.v
techmap -map techmaps/olmc_seq.v

# Add OLMC for internal GAL_SOPs
#techmap -max_iter 1 -map techmaps/pla_olmc_int.v */t:GAL_OLMC %ci2 */t:GAL_SOP %i */t:GAL_SOP %D
techmap -max_iter 1 -map techmaps/pla_olmc_int.v */t:GAL_SOP %co1 */w:* %i */t:GAL_SOP %ci1 */w:* %i %i %c %ci1 %D

# Combinational OLMC
iopadmap -bits -outpad GAL_COMB_OUTPUT_P A:Y */t:GAL_SOP "%x:+\[Y\]" */t:GAL_SOP %d o:* %i
techmap -map techmaps/olmc_comb.v

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
