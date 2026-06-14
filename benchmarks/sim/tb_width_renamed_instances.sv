`timescale 1ns/1ps

module tb_width_renamed_instances;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.narrow_source_tx__widen32to64_0_in_payload !== 32'h0000_0020) begin
      $fatal(1, "renamed narrow payload did not reach adapter");
    end
    if (dut.widen32to64_0__wide_sink_rx_payload !== 64'h0000_0000_0000_0020) begin
      $fatal(1, "renamed width adapter output mismatch");
    end
    if (dut.narrow_source_tx__widen32to64_0_in_valid !== 1'b1 || dut.narrow_source_tx__widen32to64_0_in_ready !== 1'b1 ||
        dut.widen32to64_0__wide_sink_rx_valid !== 1'b1 || dut.widen32to64_0__wide_sink_rx_ready !== 1'b1) begin
      $fatal(1, "renamed width handshake did not assert");
    end

    $display("SIM PASS T023_width_renamed_instances");
    $finish;
  end
endmodule
