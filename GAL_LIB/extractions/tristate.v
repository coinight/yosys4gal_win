module TRI_DFF_P (C, E, D, Q);
	input C, E, D;
	inout Q;

	wire X;

	GAL_TRI gal_tri_inst (.A(X), .E(E), .Y(Q));
	DFF_P dff_inst (.D(D), .C(C), .Q(X));
endmodule

module TRI_NDFF_P (C, E, D, Q);
	input C, E, D;
	inout Q;

	wire X;

	GAL_TRI gal_tri_inst (.A(X), .E(E), .Y(Q));
	NDFF_P dff_inst (.D(D), .C(C), .Q(X));
endmodule

module GAL_TRI_N (C, E, D, Q);
	input C, E, D;
	inout Q;

	wire X;

	GAL_TRI gal_tri_inst (.A(X), .E(E), .Y(Q));
	$_NOT_ not_inst (.A(D), .Y(X));
endmodule
