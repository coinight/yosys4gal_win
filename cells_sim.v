module GAL_SOP (A, Y);

parameter WIDTH = 0;
parameter DEPTH = 0;
parameter TABLE = 0;

input [WIDTH-1:0] A;
output reg Y;

integer i, j;
reg match;

always @* begin
	Y = 0;
	for (i = 0; i < DEPTH; i=i+1) begin
		match = 1;
		for (j = 0; j < WIDTH; j=j+1) begin
			if (TABLE[2*WIDTH*i + 2*j + 0] && A[j]) match = 0;
			if (TABLE[2*WIDTH*i + 2*j + 1] && !A[j]) match = 0;
		end
		if (match) Y = 1;
	end
end

endmodule

/*module DFF_P (C, D, Q);

input C, D;
output reg Q;

always @ (posedge C)
	Q <= D;

endmodule

module NDFF_P (C, D, Q);

input C, D;
output reg Q;

always @ (posedge C)
	Q <= !D;

endmodule*/

module GAL_INPUT (A, Y);

input A;
output Y;

assign Y = A;

endmodule

module GAL_OLMC (C, A, Y);

parameter REGISTERED = 0;
parameter INVERTED = 0;

input C, A;
inout reg Y;

generate
	if (REGISTERED == 1) begin
		always @ (posedge C) begin
			Y <= (INVERTED == 0) ? A : !A;
		end
	end else begin
		always @ (*) begin
			Y <= (INVERTED == 0) ? A : !A;
		end
	end
endgenerate

endmodule
