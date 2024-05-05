module tiny_xor (A, Y);

input [4:0] A;
output Y;

assign Y = ^A;

endmodule
