module and_gate (clk, A, B, Y);

input A, B;
input clk;
output reg Y;

always @(posedge clk) begin
    Y <= A && B;
end

endmodule
