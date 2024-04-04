module test (
	input clk,

	output reg [7:0] counter
);

always @ (posedge clk) begin
	counter <= counter + 1;
end

endmodule
