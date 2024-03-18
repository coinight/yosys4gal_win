module NDFF_P (
	input C, D,
	output Q
);

wire Y;

$_NOT_ not_inst (.A(D), .Y(Y));
DFF_P dff_inst (.D(Y), .C(C), .Q(Q));

endmodule

/*module GAL_MACROCELL #(
	parameter ACTIVE_HIGH = 0,
	parameter REGISTERED = 0,
)(
	input clk,

	input data,

	input in,
	output out,
);

$_NOT_ not_inst (.A(D), .Y(Y));
DFF_P dff_inst (.D(Y), .C(C), .Q(Q));

endmodule*/
