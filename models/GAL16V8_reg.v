`default_nettype none
module GAL16V8_reg (
	input wire       clk,
	input wire [7:0] in,
	input wire       oe_n,
	inout wire [7:0] io
);
	// Read in binary JEDEC file
	reg [7:0] jed_bin_file [0:278];
	initial $readmemh("GAL16V8_reg.hex", jed_bin_file);

	// Linearize to a fuse map
	wire [2193:0] fuses;
	genvar i, j;
	generate
		for (i = 0; i < 274; i = i + 1) begin
			for (j = 0; j < 8; j = j + 1) begin
				assign fuses[8*i + j] = jed_bin_file[i + 4][j];
			end
		end
		// Last couple bits
		for (j = 0; j < 2; j = j + 1) begin
			assign fuses[8*274 + j] = jed_bin_file[274 + 4][j];
		end
	endgenerate

	// Extract useful fuses
	wire syn_fuse, ac0_fuse;
	assign syn_fuse = fuses[2192];
	assign ac0_fuse = fuses[2193];

	wire [7:0] xor_fuses;
	assign xor_fuses = fuses[2055:2048];

	wire [7:0] ac1_fuses;
	assign ac1_fuses = fuses[2127:2120];

	wire [255:0] sop_fuses [0:7];
	wire [7:0] ptd_fuses [0:7];
	generate
		for (i = 0; i < 8; i = i + 1) begin
			assign sop_fuses[i] = fuses[256*i +: 256];
			assign ptd_fuses[i] = fuses[2128 + 8*i +: 8];
		end
	endgenerate

	// Interleave in and feedback for SOP inputs
	wire [7:0] feedback;
	wire [15:0] interleaved;
	generate
		for (i = 0; i < 8; i = i + 1) begin
			assign interleaved[2*i +: 2] = {feedback[i], in[i]};
		end
	endgenerate

	// Generate GAL elements
	generate
		for (i = 0; i < 8; i = i + 1) begin
			wire one_sop_out, sop_out;

			// 1SOP
			sop #(
				.NUM_PRODUCTS(1),
				.NUM_INPUTS(16)
			) one_sop_inst (
				.sop_fuses(sop_fuses[i][255:224]),
				.ptd_fuses(ptd_fuses[i][0]),
				.in(interleaved),
				.out(one_sop_out)
			);

			// SOP
			sop #(
				.NUM_PRODUCTS(7),
				.NUM_INPUTS(16)
			) sop_inst (
				.sop_fuses(sop_fuses[i][223:0]),
				.ptd_fuses(ptd_fuses[i][7:1]),
				.in(interleaved),
				.out(sop_out)
			);

			// OLMC
			olmc olmc_inst (
				.xor_fuse(xor_fuses[i]),
				.ac1_fuse(ac1_fuses[i]),
				.sop(sop_out),
				.one_sop(one_sop_out),
				.clk(clk),
				.oe_n(oe_n),
				.io(io[i]),
				.feedback(feedback[i])
			);
		end
	endgenerate

	// Simulation printing
	initial begin
		#1;
		$display("SYN: %d, AC0: %d", syn_fuse, ac0_fuse);
		$display("XOR: %b, AC1: %b", xor_fuses, ac1_fuses);
		$display("Fuses: %x", fuses);

		$display("sop0 %x", sop_fuses[0]);
		$display("ptd0 %x", ptd_fuses[0]);

		$display("sop1 %x", sop_fuses[1]);
		$display("ptd1 %x", ptd_fuses[1]);
		$display("sop2 %x", sop_fuses[2]);
		$display("ptd2 %x", ptd_fuses[2]);
		$display("sop3 %x", sop_fuses[3]);
		$display("ptd3 %x", ptd_fuses[3]);
		$display("sop4 %x", sop_fuses[4]);
		$display("ptd4 %x", ptd_fuses[4]);
		$display("sop5 %x", sop_fuses[5]);
		$display("ptd5 %x", ptd_fuses[5]);
		$display("sop6 %x", sop_fuses[6]);
		$display("ptd6 %x", ptd_fuses[6]);
		$display("sop7 %x", sop_fuses[7]);
		$display("ptd7 %x", ptd_fuses[7]);
	end
endmodule

module sop #(
	parameter NUM_PRODUCTS = 7,
	parameter NUM_INPUTS = 16
)(
	input  wire [2*NUM_INPUTS*NUM_PRODUCTS-1:0] sop_fuses,
	input  wire [NUM_PRODUCTS-1:0] ptd_fuses,
	input  wire [NUM_INPUTS-1:0] in,
	output reg  out
);
	integer i, j;
	reg match;

	always @ (*) begin
		out = 0;
		for (i = 0; i < NUM_PRODUCTS; i = i + 1) begin
			match = 1;
			for (j = 0; j < NUM_INPUTS; j = j + 1) begin
				if (!sop_fuses[2*NUM_INPUTS*i + 2*j + 0] && !in[j]) match = 0;
				if (!sop_fuses[2*NUM_INPUTS*i + 2*j + 1] &&  in[j]) match = 0;
			end
			if (match && ptd_fuses[i]) out = 1;
		end
	end
endmodule

module olmc (
	input wire xor_fuse,
	input wire ac1_fuse,

	input wire sop,
	input wire one_sop,

	input wire clk,
	input wire oe_n,

	inout  wire io,
	output wire feedback
);
	// Internal combined SOP output with optional inversion
	wire out;
	assign out = (ac1_fuse ? sop : (sop || one_sop)) ^ xor_fuse;

	reg reg_out;
	always @ (posedge clk) begin
		reg_out <= out;
	end

	assign feedback = ac1_fuse ? !reg_out : out;

	assign io = ac1_fuse ? (one_sop ? !out : 1'bz) : // Combinational
		(oe_n ? 1'bz : !reg_out); // Registered
endmodule
