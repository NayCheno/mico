`timescale 1ns/1ps

module tb_direct_stream_monitor_contract;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.monitor_src_tx__monitor_sink_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "monitor payload did not reach sink");
    end
    if (dut.monitor_src_tx__monitor_sink_rx_valid !== 1'b1 ||
        dut.monitor_src_tx__monitor_sink_rx_ready !== 1'b1) begin
      $fatal(1, "monitor ready-valid handshake did not assert");
    end

    $display("SIM PASS T053_direct_stream_monitor_contract");
    $finish;
  end
endmodule
