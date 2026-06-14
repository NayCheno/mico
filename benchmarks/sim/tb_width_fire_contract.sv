`timescale 1ns/1ps

module tb_width_fire_contract;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.source32_tx__widen32to64_0_in_payload !== 32'h0000_0020) begin
      $fatal(1, "source payload did not reach fire-contract width adapter input");
    end
    if (dut.widen32to64_0__sink64_rx_payload !== 64'h0000_0000_0000_0020) begin
      $fatal(1, "fire-contract width adapter did not zero-extend payload");
    end
    if (dut.source32_tx__widen32to64_0_in_valid !== 1'b1 ||
        dut.source32_tx__widen32to64_0_in_ready !== 1'b1 ||
        dut.widen32to64_0__sink64_rx_valid !== 1'b1 ||
        dut.widen32to64_0__sink64_rx_ready !== 1'b1) begin
      $fatal(1, "fire-contract width ready-valid handshake did not assert");
    end

    $display("SIM PASS T025_width_fire_contract");
    $finish;
  end
endmodule
