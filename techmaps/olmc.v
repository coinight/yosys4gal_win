(* techmap_celltype = "REG_OUT_P" *)
module _80_REG_OUT_P (C, A, Y);
	input C, A;
	output Y;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b1),
			.INVERTED(1'b0)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule

(* techmap_celltype = "REG_OUT_N" *)
module _81_REG_OUT_N (C, A, Y);
	input C, A;
	output Y;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b1),
			.INVERTED(1'b1)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule

(* techmap_celltype = "GAL_OUTPUT" *)
module _82_GAL_OUTPUT (A);
	input A;

	// Delete
endmodule

(* techmap_celltype = "GAL_COMB_OUTPUT_P" *)
module _82_GAL_COMB_OUTPUT_P (A, Y);
	input A, Y;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b0),
			.INVERTED(1'b0)
		) _TECHMAP_REPLACE_ (
			.C(1'bX),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule
