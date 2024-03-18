module SOP_DFF (C, A, Y);

parameter WIDTH = 4;
parameter DEPTH = 0;
parameter TABLE = 0;

input C;
input [3:0] A;
output Y;

wire sop;

GAL_SOP #(
	.WIDTH(WIDTH),
	.DEPTH(DEPTH),
	.TABLE(TABLE))
gal_sop_inst (
	.A(A),
	.Y(sop)
);

DFF_P dff_inst(.C(C), .D(sop), .Q(Y));

endmodule
