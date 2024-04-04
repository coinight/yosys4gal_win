(* techmap_celltype = "GAL_SOP" *)
module _80_GAL_SOP (A, Y);
	parameter WIDTH = 0;
	parameter DEPTH = 0;
	parameter TABLE = 0;

	input [WIDTH-1:0] A;
	output reg Y;

	generate
		if (WIDTH == 1 && DEPTH == 1 && TABLE == 01) begin
			$_NOT_ _TECHMAP_REPLACE_ (.A(A), .Y(Y));
		end else if (WIDTH == 1 && DEPTH == 1 && TABLE == 10) begin
			$_BUF_ _TECHMAP_REPLACE_ (.A(A), .Y(Y));
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
