`timescale 1ns/1ps

module tb_latency_seed_fifo_chain;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.producer0_tx__fifo0_input_payload !== 32'h0000_0001) begin
      $fatal(1, "latency producer payload did not reach fifo");
    end
    if (dut.fifo0_output__consumer0_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "latency fifo payload did not reach consumer");
    end
    if (dut.producer0_tx__fifo0_input_valid !== 1'b1 || dut.producer0_tx__fifo0_input_ready !== 1'b1 ||
        dut.fifo0_output__consumer0_rx_valid !== 1'b1 || dut.fifo0_output__consumer0_rx_ready !== 1'b1) begin
      $fatal(1, "latency fifo handshake did not assert");
    end

    $display("SIM PASS T019_latency_seed_fifo_chain");
    $finish;
  end
endmodule
