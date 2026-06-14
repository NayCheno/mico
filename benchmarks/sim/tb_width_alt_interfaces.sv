`timescale 1ns/1ps

module tb_width_alt_interfaces;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.src_tx__widen32to64_0_in_payload !== 32'h0000_0020) begin
      $fatal(1, "narrow payload did not reach width adapter");
    end
    if (dut.widen32to64_0__dst_rx_payload !== 64'h0000_0000_0000_0020) begin
      $fatal(1, "width adapter did not zero-extend alt payload");
    end
    if (dut.src_tx__widen32to64_0_in_valid !== 1'b1 || dut.src_tx__widen32to64_0_in_ready !== 1'b1 ||
        dut.widen32to64_0__dst_rx_valid !== 1'b1 || dut.widen32to64_0__dst_rx_ready !== 1'b1) begin
      $fatal(1, "width alt handshake did not assert");
    end

    $display("SIM PASS T021_width_alt_interfaces");
    $finish;
  end
endmodule
