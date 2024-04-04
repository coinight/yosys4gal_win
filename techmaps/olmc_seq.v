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
			.E(1'b1),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule

(* techmap_celltype = "DFF_P" *)
module _80_DFF_P (C, D, Q);
	input C, D;
	output Q;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b1),
			.INVERTED(1'b0)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.E(1'b1),
			.A(D),
			.Y(Q)
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
			.E(1'b1),
			.A(A),
			.Y(Y)
		);
	endgenerate
endmodule

(* techmap_celltype = "NDFF_P" *)
module _81_NDFF_P (C, D, Q);
	input C, D;
	output Q;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b1),
			.INVERTED(1'b1)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.E(1'b1),
			.A(D),
			.Y(Q)
		);
	endgenerate
endmodule

(* techmap_celltype = "GAL_OUTPUT" *)
module _82_GAL_OUTPUT (A);
	input A;

	// Delete
endmodule
