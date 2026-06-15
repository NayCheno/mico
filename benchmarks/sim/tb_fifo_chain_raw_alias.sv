`timescale 1ns/1ps

module tb_fifo_chain_raw_alias;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.raw_src_tx__raw_fifo_input_payload !== 32'h0000_0001) begin
      $fatal(1, "raw payload did not reach fifo");
    end
    if (dut.raw_fifo_output__raw_sink_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "raw fifo payload did not reach sink");
    end
    if (dut.raw_src_tx__raw_fifo_input_valid !== 1'b1 ||
        dut.raw_src_tx__raw_fifo_input_ready !== 1'b1 ||
        dut.raw_fifo_output__raw_sink_rx_valid !== 1'b1 ||
        dut.raw_fifo_output__raw_sink_rx_ready !== 1'b1) begin
      $fatal(1, "raw fifo ready-valid handshake did not assert");
    end

    $display("SIM PASS T054_fifo_chain_raw_alias");
    $finish;
  end
endmodule
