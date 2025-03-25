(* techmap_celltype = "GAL_SOP" *)
module _80_GAL_SOP (A, Y);
	parameter WIDTH = 0;
	parameter DEPTH = 0;
	parameter TABLE = 0;

	input [WIDTH-1:0] A;
	output Y;

	generate
		wire internal;

		GAL_OLMC #(
			.REGISTERED(1'b0),
			.INVERTED(1'b0)
		) olmc_inst (
			.C(1'bX),
			.E(1'b1),
			.A(internal),
			.Y(Y)
		);

		GAL_SOP #(
			.WIDTH(WIDTH),
			.DEPTH(DEPTH),
			.TABLE(TABLE)
		) _TECHMAP_REPLACE_ (
			.A(A),
			.Y(internal)
		);
	endgenerate
endmodule
