module REG_OUT_P (C, A, Y);
	input C, A;
	output Y;

	DFF_P dff_p_inst (.C(C), .D(A), .Q(Y));
	GAL_OUTPUT gal_output_inst (.A(Y));
endmodule

module REG_OUT_N (C, A, Y);
	input C, A;
	output Y;

	NDFF_P dff_p_inst (.C(C), .D(A), .Q(Y));
	GAL_OUTPUT gal_output_inst (.A(X));
endmodule
