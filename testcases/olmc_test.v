module olmc_test (clk, A, B, AND, NAND, REG_AND, REG_NAND);

input clk, A, B;
output AND, NAND;
output reg REG_AND, REG_NAND;

assign AND = A && B;
assign NAND = !(A && B);

always @ (posedge clk) begin
	REG_AND <= A && B;
	REG_NAND <= !(A && B);
end

endmodule
