`timescale 1ns / 1ps
// Note that top level testbench module does not need any IO ports and
// hence can be empty and is usually called "tb" or "tb_top", but it can be
// named anything.

module tb_latch;

// All testbench code goes inside this module
reg     d;             // To drive input "d" of the DUT
reg     en;         // To drive input "en" of the DUT
reg     rstn;         // To drive input "rstn" of the DUT

reg     prev_q;     // To ensure q has not changed when en=0

wire     q;             // To tap output "q" from DUT

d_latch u0 (
	.d    (d),
	.en  (en),
	.rstn (rstn),
	.q    (q)
);

task init();
	begin
		d <= 0;
		en <= 0;
		rstn <= 0;
	end
endtask

task reset_release();
	// 2. Release reset
	#10 rstn <= 1;
endtask

task checker (input d, en, rstn, q);
	begin
		#1;
		if (!rstn) begin
			if (q != 0)
				$error("Q is not 0 during resetn !");
		end else begin
			if (en) begin
				if (q != d)
					$error ("Q does not follow D when EN is high !");
			end else begin
				if (q != prev_q)
					$error ("Q does not get latched !");
			end
		end
	end
endtask

task test_1();
	integer i;
	integer delay;
	integer delay2;
	// 3. Randomly change d and enable
	for (i = 0; i < 5; i=i+1) begin
		delay = $random;
		delay2 = $random;
		#(delay2) en <= ~en;
		#(delay) d <= i;

		// Check output value for given inputs
		checker(d, en, rstn, q);
		prev_q <= q;
	end
endtask

initial begin
	// As shown in step 4
	init();

	reset_release();
	test_1();
end

endmodule
