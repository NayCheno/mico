`timescale 1ns/1ps

module tb_fifo_chain_contractless;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.p_tx__f_input_payload !== 32'h0000_0001) begin
      $fatal(1, "producer payload did not reach contractless fifo");
    end
    if (dut.f_output__c_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "contractless fifo payload did not reach consumer");
    end
    if (dut.p_tx__f_input_valid !== 1'b1 || dut.p_tx__f_input_ready !== 1'b1 ||
        dut.f_output__c_rx_valid !== 1'b1 || dut.f_output__c_rx_ready !== 1'b1) begin
      $fatal(1, "contractless fifo handshake did not assert");
    end

    $display("SIM PASS T018_fifo_chain_contractless");
    $finish;
  end
endmodule
