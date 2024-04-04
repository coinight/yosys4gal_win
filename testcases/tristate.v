module tristate (
	input a, b, c,
	output y
);

assign y = c ? a && b : 1'bz;

endmodule
