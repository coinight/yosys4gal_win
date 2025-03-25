#!/usr/bin/env -S yosys -c
# 重命名原生 exec 为 tcl_exec
# 仅在原生 exec 存在且未重命名时执行操作
if {[llength [info commands exec]] > 0} {    ;# 检查原生 exec 是否存在
    if {[llength [info commands tcl_exec]] == 0} {  ;# 避免重复重命名
		rename exec tcl_exec
        puts "TCL exec renamed to tcl_exec."
    } else {
        puts "tcl_exec already exists, skip renaming."
    }
} else {
    puts "Original TCL exec not found. Possible conflict?"
}

yosys -import

## Check arguments
if { $argc != 1 && $argc != 2 } {
	puts "USAGE: $argv0 -- <VERILOG FILE> \[CHIP\]"
	exit
}

set fbasename [file rootname [file tail [lindex $argv 0]]]
puts $fbasename

# 检查是否存在 output 文件夹，若存在则强制删除
if {[file exists "output"] && [file isdirectory "output"]} {
    exec -- rmdir /s /q output
    puts "Deleted existing output directory."
}
#exec rm -rf output
exec -- mkdir output

## Set target chip (default to GAL16V8)
set target [expr {$argc == 2 ? [lindex $argv 1] : "GAL16V8"}]
if {$target == "GAL16V8"} {
	set num_max_products 7
} elseif {$target == "GAL22V10"} {
	set num_max_products 16
} else {
	puts "Invalid target chip: GAL16V8 and GAL22V10 available"
	puts "Set Chip GAL16V8"
	set num_max_products 7
}

## Read Verilog/Liberty file
read_verilog [lindex $argv 0]
hierarchy -auto-top
read_verilog -lib cells_sim.v

read_liberty -lib GAL_LIB/techmaps/gal_dff.lib

## First pass synthesis
tribuf
synth
design -save preop

# Map inputs and tristate pins
iopadmap -bits -inpad GAL_INPUT Y:A -toutpad GAL_TRI E:A:Y -tinoutpad GAL_TRI E:Y:A

## DFF/SOP mapping
dfflibmap -liberty GAL_LIB/techmaps/gal_dff.lib

# Get count of non-clock inputs and registers
set num_inputs [regexp -inline {\d+} [tee -s result.string select -count t:GAL_INPUT]]
set num_regs [regexp -inline {\d+} [tee -s result.string select -count t:DFF_P]]
set num_inputs_regs [expr $num_inputs + $num_regs]
if {$num_regs > 0} { set num_inputs_regs [expr $num_inputs_regs - 1] }

#abc -script "+strash;,dretime;,collapse;,write_pla,test.pla" -sop
# Force one-level SOP
#abc -script "abc.script" -sop

# Resynth all too big SOPs together in multi-level SOP
#select "t:\$sop" r:DEPTH>8 %i
#techmap -autoproc -map sop.v
#yosys proc
#techmap
#select *

#abc -sop -I $num_inputs_regs -P $num_max_products
# Use infinite inputs since ABC is dumb and this usually gets around it (shouldn't cause problems)
abc -sop -I 100000 -P $num_max_products

opt
clean -purge
puts "#####test#####"
## Tech mapping
# PLAs
techmap -map GAL_LIB/techmaps/pla.v -D PLA_MAX_PRODUCTS=$num_max_products
techmap -max_iter 1 -map GAL_LIB/techmaps/trivial_sop.v

# Sequential OLMC 
extract -constports -map GAL_LIB/extractions/ndff.v
extract -constports -map GAL_LIB/extractions/tristate.v
techmap -map GAL_LIB/techmaps/olmc_seq.v

# Make 1SOPs for combinational tristates
techmap -max_iter 1 -map GAL_LIB/techmaps/one_sop.v */t:GAL_TRI "%x:+\[E\]" */t:GAL_TRI %d %ci1 */t:GAL_SOP %i
techmap -max_iter 1 -map GAL_LIB/techmaps/one_sop.v */t:GAL_TRI_N "%x:+\[E\]" */t:GAL_TRI_N %d %ci1 */t:GAL_SOP %i

# Add OLMC for internal GAL_SOPs
#techmap -max_iter 1 -map GAL_LIB/techmaps/pla_olmc_int.v */t:GAL_OLMC %ci2 */t:GAL_SOP %i */t:GAL_SOP %D
techmap -max_iter 1 -map GAL_LIB/techmaps/pla_olmc_int.v */t:GAL_SOP %co1 */w:* %i */t:GAL_SOP %ci1 */w:* %i %i %c %ci1 %D

# Add OLMC for internal GAL_SOPs attached to enable lines
techmap -max_iter 1 -map GAL_LIB/techmaps/pla_olmc_int.v */t:GAL_SOP %co1 */w:* %i */t:GAL_OLMC "%ci1:+\[E\]" */w:* %i %i %c %ci1 %D

# Combinational OLMC
iopadmap -bits -outpad GAL_COMB_OUTPUT_P A:Y */t:GAL_SOP "%x:+\[Y\]" */t:GAL_SOP %d o:* %i
techmap -map GAL_LIB/techmaps/olmc_comb.v

# Add trivial SOPs between directly connected OLMCs
techmap -max_iter 1 -map GAL_LIB/techmaps/trivial_sop_olmc.v */t:GAL_OLMC "%ci1:+\[A\]" */w:* %i */t:GAL_SOP %co1 */w:* %i %i %c %co1 %D */t:GAL_OLMC %D

# Add trivial SOPs between directly connected OLMCs and INPUTs
techmap -max_iter 1 -map GAL_LIB/techmaps/trivial_sop_olmc.v */t:GAL_OLMC "%ci1:+\[A\]" */w:* %i */t:GAL_INPUT %co1 */w:* %i %i %c %co1 %D

# Add trivial 1SOPs between directly connected OLMC enables and INPUTs
techmap -max_iter 1 -map GAL_LIB/techmaps/trivial_1sop_olmc.v */t:GAL_OLMC "%ci1:+\[E\]" */w:* %i */t:GAL_INPUT %co1 */w:* %i %i %c %co1 %D

clean -purge

## Write output files
write_verilog "output/synth_${fbasename}.v"
write_json "output/synth_${fbasename}.json"

## Verify equivalence
# Backup and make gold and gate modules
design -stash postop
design -copy-from preop -as gold A:top
design -copy-from postop -as gate A:top

# Inverse tech map into primatives
techmap -autoproc -map GAL_LIB/techmaps/cells_sim.v
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

## Print final stats and show graph
show -width -signed
exec -- move show.dot output
stat

shell
exit