`ifndef PLA_MAX_PRODUCTS
$fatal(1, "Macro PLA_MAX_PRODUCTS must be defined");
`endif

(* techmap_celltype = "$sop $__sop" *)
module _80_sop (A, Y);
	parameter WIDTH = 0;
	parameter DEPTH = 0;
	parameter TABLE = 0;

	input [WIDTH-1:0] A;
	output reg Y;

	// Add a blank variable to TABLE
	function [2*(WIDTH+1)*DEPTH-1:0] add_var_table;
		integer i, j;
		for (i = 0; i < DEPTH; i=i+1) begin
			for (j = 0; j < WIDTH + 1; j=j+1) begin
				if (j < WIDTH) begin
					add_var_table[2*(WIDTH+1)*i + 2*j + 0] = TABLE[2*WIDTH*i + 2*j + 0];
					add_var_table[2*(WIDTH+1)*i + 2*j + 1] = TABLE[2*WIDTH*i + 2*j + 1];
				end else begin
					add_var_table[2*(WIDTH+1)*i + 2*j + 0] = 1'b0;
					add_var_table[2*(WIDTH+1)*i + 2*j + 1] = 1'b0;
				end
			end
		end
	endfunction

	generate
		genvar i, j;
		if (DEPTH <= `PLA_MAX_PRODUCTS) begin // Convert to GAL_SOP object if it fits
			GAL_SOP #(
				.WIDTH(WIDTH),
				.DEPTH(DEPTH),
				.TABLE(TABLE)
			) _TECHMAP_REPLACE_ (
				.A(A),
				.Y(Y)
			);
		end else begin // Otherwise split into two new SOP objects
			wire partial;
			\$__sop #(
				.WIDTH(WIDTH),
				.DEPTH(`PLA_MAX_PRODUCTS),
				.TABLE(TABLE[2*WIDTH*`PLA_MAX_PRODUCTS-1:0])
			) sop_partial (
				.A(A),
				.Y(partial)
			);

			localparam EXTRA_VAR_TABLE = add_var_table();
			\$__sop #(
				.WIDTH(WIDTH+1),
				.DEPTH(DEPTH-`PLA_MAX_PRODUCTS+1),
				.TABLE({EXTRA_VAR_TABLE[2*(WIDTH+1)*DEPTH-1:2*(WIDTH+1)*`PLA_MAX_PRODUCTS], {2'b10, {{WIDTH}{2'b00}}}})
			) sop_rest (
				.A({partial, A}),
				.Y(Y)
			);
		end
	endgenerate
endmodule
