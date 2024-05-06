Model-Based Verification Checking
=================================

To assist with verification checking, _yosys4gal_ includes Verilog models of
the supported GAL16V8 and GAL22V10 chips. These Verilog read in the
tool-generated fuse map and the behavior should match that of the original
Verilog. The provided script performs this check using yosys' SAT solver:
```
./prove_equiv.tcl -- <JEDEC FILE> <PCF CONSTRAINTS> <VERILOG FILES...>
```

Limitations
-----------
The models only support the functionality of the GAL chips which is supported
in _yosys4gal_. I.e. the GAL16V8 model only supports registered mode and the
GAL22V10 model doesn't support asynchronous set/reset. Also note that a failure
to prove equivalence does not imply inequivalence. In particular, yosys can
struggle to synthesize the GAL22V10 model correctly. The false positive rate,
however, should be zero.

Dependencies
------------
- `yosys` 0.38 or higher
- `jedutil` from MAME utilities
- `xxd` from Vim
