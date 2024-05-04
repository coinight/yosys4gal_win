module and_gate (clk, A, B, Y);

input A, B;
output Y;

always @(posedge clk) begin
    Y <= A && B;
end

endmodule
