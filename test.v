module test (
	input clk,

	output reg [0:7] counter
);

always @ (posedge clk) begin
	counter <= counter - 1;
end

endmodule

/*module test (
	input [1:0] a, b,
	output [2:0] y
);

assign y = a + b;

endmodule*/
