(* techmap_celltype = "GAL_OLMC" *)
module _80_GAL_OLMC (C, E, A, Y);
	parameter REGISTERED = 0;
	parameter INVERTED = 0;

	input C, E, A;
	inout Y;

	wire int;

	generate
		GAL_OLMC #(
			.REGISTERED(REGISTERED),
			.INVERTED(INVERTED)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.E(int),
			.A(A),
			.Y(Y)
		);

		GAL_1SOP #(
			.WIDTH(1),
			.DEPTH(1),
			.TABLE(2'b10)
		) trivial_1sop_olmc (
			.A(E),
			.Y(int),
		);
	endgenerate
endmodule
