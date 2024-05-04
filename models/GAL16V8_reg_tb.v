
module GAL16V8_reg_tb(
	output reg        clk = 0,
	output reg  [7:0] in,
	output reg        oe_n,
	inout  wire [7:0] io
);

GAL16V8_reg GAL16V8_reg_inst (
	.clk(clk),
	.in(in),
	.oe_n(oe_n),
	.io(io)
);

always #5 clk = !clk;

initial begin
	$dumpfile("dump.vcd");
	$dumpvars(0, GAL16V8_reg_tb);
	#3;
	in = 8'b0000_1100;
	oe_n = 0;
	#10;
	in = 8'b0000_1001;
	#10;
	in = 8'b0000_0110;
	#10;
	in = 8'b0000_0011;
	#10;
	in = 8'b0000_1100;
	#30;
	oe_n = 1;
	#10;
	$finish;
end

endmodule
