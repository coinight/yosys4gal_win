module big_xor (A, Y);

input [7:0] A;
output Y;

assign Y = ^A;

endmodule
