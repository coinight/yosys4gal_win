module REG_OUT_P (
	input C, A,
	output Y
);


DFF_P dff_p_inst (.C(C), .D(A), .Q(Y));
GAL_OUTPUT gal_output_inst (.A(Y));

endmodule

module REG_OUT_N (
	input C, A,
	output Y
);

NDFF_P dff_p_inst (.C(C), .D(A), .Q(Y));
GAL_OUTPUT gal_output_inst (.A(X));

endmodule
