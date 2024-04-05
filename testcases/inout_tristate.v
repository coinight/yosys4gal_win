module inout_tristate (
	input a, b, c,
	inout y,
	output z
);

assign y = c && b ? a && b : 1'bz;
assign z = c ? a || b : y;

endmodule
