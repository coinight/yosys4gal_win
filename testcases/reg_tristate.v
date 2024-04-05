module reg_tristate (
	input clk,
	input a, b, c,
	output y
);

reg x;

assign y = c && b ? x : 1'bz;

always @ (posedge clk)
	x <= a && b;

endmodule
