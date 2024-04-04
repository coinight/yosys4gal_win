(* techmap_celltype = "GAL_COMB_OUTPUT_P" *)
module _80_GAL_COMB_OUTPUT_P (A, Y);
	input A, Y;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b0),
			.INVERTED(1'b0)
		) _TECHMAP_REPLACE_ (
			.C(1'bX),
			.E(1'b1),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule

(* techmap_celltype = "$_NOT_" *)
module _80_NOT (A, Y);
	input A, Y;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b0),
			.INVERTED(1'b1)
		) _TECHMAP_REPLACE_ (
			.C(1'bX),
			.E(1'b1),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule
