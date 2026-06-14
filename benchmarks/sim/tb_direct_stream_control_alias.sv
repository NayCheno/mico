`timescale 1ns/1ps

module tb_direct_stream_control_alias;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.producer_a_tx__consumer_a_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "control payload did not reach consumer");
    end
    if (dut.producer_a_tx__consumer_a_rx_valid !== 1'b1 ||
        dut.producer_a_tx__consumer_a_rx_ready !== 1'b1) begin
      $fatal(1, "control ready-valid handshake did not assert");
    end

    $display("SIM PASS T051_direct_stream_control_alias");
    $finish;
  end
endmodule
