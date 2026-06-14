`timescale 1ns/1ps

module tb_subsystem_seed_fifo_chain;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.ingress_tx__buffer0_input_payload !== 32'h0000_0001) begin
      $fatal(1, "ingress payload did not reach subsystem buffer");
    end
    if (dut.buffer0_output__egress_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "subsystem buffer payload did not reach egress");
    end
    if (dut.ingress_tx__buffer0_input_valid !== 1'b1 ||
        dut.ingress_tx__buffer0_input_ready !== 1'b1 ||
        dut.buffer0_output__egress_rx_valid !== 1'b1 ||
        dut.buffer0_output__egress_rx_ready !== 1'b1) begin
      $fatal(1, "subsystem fifo ready-valid handshake did not assert");
    end

    $display("SIM PASS T032_subsystem_seed_fifo_chain");
    $finish;
  end
endmodule
