`timescale 1ns/1ps

module tb_control_status_width_seed;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.ctrl_source_tx__widen32to64_0_in_payload !== 32'h0000_0020) begin
      $fatal(1, "control payload did not reach width adapter input");
    end
    if (dut.widen32to64_0__status_sink_rx_payload !== 64'h0000_0000_0000_0020) begin
      $fatal(1, "status payload was not zero-extended");
    end
    if (dut.ctrl_source_tx__widen32to64_0_in_valid !== 1'b1 ||
        dut.ctrl_source_tx__widen32to64_0_in_ready !== 1'b1 ||
        dut.widen32to64_0__status_sink_rx_valid !== 1'b1 ||
        dut.widen32to64_0__status_sink_rx_ready !== 1'b1) begin
      $fatal(1, "control/status width ready-valid handshake did not assert");
    end

    $display("SIM PASS T057_control_status_width_seed");
    $finish;
  end
endmodule
