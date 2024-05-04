module complex_single_sop (clk, A, B, C,D, Y);

input A, B, C, D;
input clk;
output reg Y;

always @(posedge clk) begin
    Y <= (A && !B && !C && D) || (!A && B && !C && D) || (!A && !B && C && D) || (A && B && C && D);
end

endmodule
