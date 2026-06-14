`timescale 1ns/1ps

module tb_fifo_chain_fire_contract;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.p0_tx__f0_input_payload !== 32'h0000_0001) begin
      $fatal(1, "producer payload did not reach fifo");
    end
    if (dut.f0_output__c0_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "fifo payload did not reach consumer");
    end
    if (dut.p0_tx__f0_input_valid !== 1'b1 || dut.p0_tx__f0_input_ready !== 1'b1 ||
        dut.f0_output__c0_rx_valid !== 1'b1 || dut.f0_output__c0_rx_ready !== 1'b1) begin
      $fatal(1, "fifo fire handshake did not assert");
    end

    $display("SIM PASS T016_fifo_chain_fire_contract");
    $finish;
  end
endmodule
