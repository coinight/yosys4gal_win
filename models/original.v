module original (
	input wire clk,
	input wire [5:0] in,
	output reg [4:0] out
);

always @(posedge clk) begin
	out[0] <= in[0] && in[1];
	out[1] <= in[2] || in[3];
	out[2] <= in[4] && !in[5] || !in[4] && in[5];
	out[3] <= in[0] && in[1] && in[2] && in[3] && in[4] && in[5];
	out[4] <= !(in[0] || in[1] || in[2] || in[3] || in[4] || in[5]);
end

endmodule
