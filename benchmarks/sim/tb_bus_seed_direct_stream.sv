`timescale 1ns/1ps

module tb_bus_seed_direct_stream;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.register_source_tx__register_sink_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "register source payload did not reach sink");
    end
    if (dut.register_source_tx__register_sink_rx_valid !== 1'b1 ||
        dut.register_source_tx__register_sink_rx_ready !== 1'b1) begin
      $fatal(1, "register direct ready-valid handshake did not assert");
    end

    $display("SIM PASS T031_bus_seed_direct_stream");
    $finish;
  end
endmodule
