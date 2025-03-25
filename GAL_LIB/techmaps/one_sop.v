// Convert GAL_SOPs with only 1 product into a "1SOP"
// Intended for the enable product

(* techmap_celltype = "GAL_SOP" *)
module _80_GAL_SOP (A, Y);
	parameter WIDTH = 0;
	parameter DEPTH = 0;
	parameter TABLE = 0;

	input [WIDTH-1:0] A;
	output reg Y;

	generate
		if (DEPTH == 1) begin
			GAL_1SOP #(
				.WIDTH(WIDTH),
				.DEPTH(DEPTH),
				.TABLE(TABLE)
			) _TECHMAP_REPLACE_ (
				.A(A),
				.Y(Y)
			);
		end else begin // No-op
			GAL_SOP #(
				.WIDTH(WIDTH),
				.DEPTH(DEPTH),
				.TABLE(TABLE)
			) _TECHMAP_REPLACE_ (
				.A(A),
				.Y(Y)
			);
		end
	endgenerate
endmodule
