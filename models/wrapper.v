module wrapper (
	input wire clk,
	input wire [5:0] in,
	output wire [4:0] out
);

wire [7:0] in_int;
wire [7:0] io_int;
wire oe_n;

assign oe_n = 0;
assign in_int = {3'b0, in};
assign out = {io_int[3], io_int[4], io_int[5], io_int[6], io_int[7]};

GAL16V8_reg GAL16V8_reg_inst (
	.clk(clk),
	.in(in_int),
	.oe_n(oe_n),
	.io(io_int)
);

endmodule
