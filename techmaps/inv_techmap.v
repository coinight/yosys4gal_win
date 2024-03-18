module GAL_SOP (A, Y);

parameter WIDTH = 0;
parameter DEPTH = 0;
parameter TABLE = 0;

input [WIDTH-1:0] A;
output reg Y;

\$sop #(
	.WIDTH(WIDTH),
	.DEPTH(DEPTH),
	.TABLE(TABLE)
) sop_partial (
	.A(A),
	.Y(Y)
);

endmodule

module DFF_P (C, D, Q);

input C, D;
output Q;

\$_DFF_P_ dff_inst (.C(C), .D(D), .Q(Q));

endmodule

module NDFF_P (C, D, Q);

input C, D;
output Q;

wire Y;

\$_NOT_ not_inst (.A(D), .Y(Y));
\$_DFF_P_ dff_inst (.C(C), .D(Y), .Q(Q));

endmodule
