module GAL22V10_reg_tb(
	output reg  [12:0] in = 13'bx_xxxx_xxxx_xxx1,
	inout  wire [9:0] io
);

GAL22V10_reg GAL22V10_reg_inst (
	.in(in),
	.io(io)
);

always #5 in[0] = !in[0];

initial begin
	$dumpfile("dump.vcd");
	$dumpvars(0, GAL22V10_reg_tb);
	#3;
	in[12:1] = 12'bxxxx_xxxx_x00x;
	#10;
	in[12:1] = 12'bxxxx_xxxx_x01x;
	#10;
	in[12:1] = 12'bxxxx_xxxx_x10x;
	#10;
	in[12:1] = 12'bxxxx_xxxx_x11x;
	#10;
	in[12:1] = 12'bxxxx_xxxx_x00x;
	#10;
	$finish;
end

endmodule
