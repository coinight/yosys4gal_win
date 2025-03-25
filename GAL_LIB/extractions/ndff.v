module NDFF_P (C, D, Q);
	input C, D;
	output Q;

	wire Y;

	$_NOT_ not_inst (.A(D), .Y(Y));
	DFF_P dff_inst (.D(Y), .C(C), .Q(Q));
endmodule
