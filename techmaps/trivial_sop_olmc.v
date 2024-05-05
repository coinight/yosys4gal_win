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
			.E(E),
			.A(int),
			.Y(Y)
		);

		GAL_SOP #(
			.WIDTH(1),
			.DEPTH(1),
			.TABLE(2'b10)
		) trivial_sop (
			.A(A),
			.Y(int),
		);
	endgenerate
endmodule
