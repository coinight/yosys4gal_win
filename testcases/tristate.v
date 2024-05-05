module tristate (
	input a, b, c, d,
	output x, y
);

assign x = !d;
assign y = c ? a && b : 1'bz;

endmodule
