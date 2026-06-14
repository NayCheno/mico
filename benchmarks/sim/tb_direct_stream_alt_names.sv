`timescale 1ns/1ps

module tb_direct_stream_alt_names;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.prod_tx__cons_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "direct alt payload mismatch");
    end
    if (dut.prod_tx__cons_rx_valid !== 1'b1 || dut.prod_tx__cons_rx_ready !== 1'b1) begin
      $fatal(1, "direct alt handshake did not assert");
    end

    $display("SIM PASS T013_direct_stream_alt_names");
    $finish;
  end
endmodule
