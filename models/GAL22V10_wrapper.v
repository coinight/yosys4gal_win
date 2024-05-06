`default_nettype none
module __wrapper (
	input wire [12:0] __in, // Pin {13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1}, 1 is clk
	inout wire [9:0] __io   // Pin {14, 15, 16, 17, 18, 19, 20, 21, 22, 23}
);

GAL22V10_reg GAL22V10_reg_inst (
	.in(__in),
	.io(__io)
);

endmodule
