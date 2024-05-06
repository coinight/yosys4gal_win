`default_nettype none
module __wrapper (
	input wire       __clk,  // Pin 1
	input wire [7:0] __in,   // Pin {9, 8, 7, 6, 5, 4, 3, 2}
	input wire       __oe_n, // Pin 11
	inout wire [7:0] __io    // Pin {12, 13, 14, 15, 16, 17, 18, 19}
);

GAL16V8_reg GAL16V8_reg_inst (
	.clk(__clk),
	.in(__in),
	.oe_n(__oe_n),
	.io(__io)
);

endmodule
