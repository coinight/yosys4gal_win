module GAL_INOUT_TRI(A, E, I, Y);
	input A, E;
	output I;
	inout Y;

	generate
		GAL_OLMC #(
			REGISTERED = 0,
			INVERTED = 0
		) _TECHMAP_REPLACE_ (
			.A(A),
			.C(1'bX),
			.E(E),
			.Y(E ? Y : I);
		);
	endgenerate
endmodule
