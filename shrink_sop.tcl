#!/usr/bin/env -S yosys -c
yosys -import

# Check arguments
if { $argc < 3 } {
	puts "USAGE: $argv0 -- <JSON_FILE> <SOP_NAME> <MAX_SIZE>"
	exit
}

read_json [lindex $argv 0]
hierarchy -auto-top
read_verilog -lib cells_sim.v
read_liberty -lib techmaps/gal_dff.lib

# PLAs
chtype -set "\$__sop" [lindex $argv 1]
techmap -map techmaps/pla.v -D PLA_MAX_PRODUCTS=[lindex $argv 2] [lindex $argv 1]
#chtype -map "\$__sop" "GAL_SOP" *

# Add OLMC for internal GAL_SOPs
techmap -max_iter 1 -map techmaps/pla_olmc_int.v */t:GAL_SOP %co1 */w:* %i */t:GAL_SOP %ci1 */w:* %i %i %c %ci1 %D

clean -purge

write_json [lindex $argv 0]
#write_json test.json
