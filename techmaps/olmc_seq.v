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

(* techmap_celltype = "TRI_DFF_P" *)
module _80_TRI_DFF_P (C, E, D, Q);
	input C, E, D;
	inout Q;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b1),
			.INVERTED(1'b0)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.E(E),
			.A(D),
			.Y(Q)
		);
	endgenerate
endmodule

(* techmap_celltype = "TRI_NDFF_P" *)
module _81_TRI_NDFF_P (C, E, D, Q);
	input C, E, D;
	inout Q;

	generate
		GAL_OLMC #(
			.REGISTERED(1'b1),
			.INVERTED(1'b1)
		) _TECHMAP_REPLACE_ (
			.C(C),
			.E(E),
			.A(D),
			.Y(Q)
		);
	endgenerate
endmodule
