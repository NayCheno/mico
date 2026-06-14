`timescale 1ns/1ps

module tb_width_with_latency_attr;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.s0_tx__widen32to64_0_in_payload !== 32'h0000_0020) begin
      $fatal(1, "latency-attr narrow payload did not reach adapter");
    end
    if (dut.widen32to64_0__t0_rx_payload !== 64'h0000_0000_0000_0020) begin
      $fatal(1, "latency-attr width adapter output mismatch");
    end
    if (dut.s0_tx__widen32to64_0_in_valid !== 1'b1 || dut.s0_tx__widen32to64_0_in_ready !== 1'b1 ||
        dut.widen32to64_0__t0_rx_valid !== 1'b1 || dut.widen32to64_0__t0_rx_ready !== 1'b1) begin
      $fatal(1, "latency-attr width handshake did not assert");
    end

    $display("SIM PASS T022_width_with_latency_attr");
    $finish;
  end
endmodule
