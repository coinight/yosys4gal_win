module adder_downto_upto (A, B, C);

input [2:0] A;
input [0:2] B;
output [3:0] C;

assign C = A + B;

endmodule
