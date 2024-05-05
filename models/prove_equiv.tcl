#!/usr/bin/env -S yosys -c
yosys -import

# Parse arguments
if { $argc < 3 } {
	puts "USAGE: $argv0 -- <JEDEC_FILE> <PCF_FILE> <VERILOG FILES> ..."
	exit
}

set jedec_file [lindex $argv 0]
set pcf_file [lindex $argv 1]
set verilog_files [lrange $argv 2 end]

# Convert JEDEC file to hex for Verilog model
exec jedutil -convert $jedec_file GAL16V8_reg.bin
exec xxd -ps -c 1 GAL16V8_reg.bin GAL16V8_reg.hex

# Read and synthesize original Verilog
read_verilog $verilog_files
hierarchy -auto-top
flatten
synth
splitnets -ports
yosys rename -top __original
select -module __original

# Process PCF file and rename ports
set used [list]
set pin_mapping [dict create 1 "clk" 2 "in\[0\]" 3 "in\[1\]" 4 "in\[2\]" 5 "in\[3\]" 6 "in\[4\]" 7 "in\[5\]" 8 "in\[6\]" 9 "in\[7\]" 11 "oe_n" 12 "io\[7\]" 13 "io\[6\]" 14 "io\[5\]" 15 "io\[4\]" 16 "io\[3\]" 17 "io\[2\]" 18 "io\[1\]" 19 "io\[0\]"]

set pcf_fp [open $pcf_file r]
foreach line [split [read $pcf_fp] "\n"] {
	puts $line
	if {[regexp {set_io\s+(.*)\s+([0-9]+)} $line -> net pin]} {
		# Rename nets to match GAL model
		yosys rename $net __[dict get $pin_mapping $pin]

		# Mark as used
		lappend used [dict get $pin_mapping $pin]
	}
}

select -clear
design -stash __original

# Read and synthesize GAL model
read_verilog wrapper.v GAL16V8_reg.v
flatten
synth
splitnets -ports
select -module __wrapper

# Delete extra "unused" ports
foreach pin_name [dict values $pin_mapping] {
	if {[lsearch -exact $used $pin_name] >= 0} {
		puts "$pin_name is used"
	} elseif {$pin_name == "oe_n"} {
		puts "$pin_name is not used"
		# Enable registered outputs if net unused
		connect -set __oe_n '0
		delete -port __$pin_name
	} else {
		puts "$pin_name is not used"
		delete -port __$pin_name
	}
}
select -clear

# Make and check equivalence circuit
design -copy-from __original -as __original A:top

equiv_make __original __wrapper equiv
equiv_induct equiv
equiv_status -assert equiv
