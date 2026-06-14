`timescale 1ns/1ps

module tb_latency_seed_direct;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.src0_tx__dst0_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "latency direct payload mismatch");
    end
    if (dut.src0_tx__dst0_rx_valid !== 1'b1 || dut.src0_tx__dst0_rx_ready !== 1'b1) begin
      $fatal(1, "latency direct handshake did not assert");
    end

    $display("SIM PASS T020_latency_seed_direct");
    $finish;
  end
endmodule
