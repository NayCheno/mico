`timescale 1ns/1ps

module tb_width_alt_stream_names_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.pixels_in_tx__widen32to64_0_in_payload !== 32'h0000_0020) begin
      $fatal(1, "pixel payload did not reach width adapter input");
    end
    if (dut.widen32to64_0__pixels_out_rx_payload !== 64'h0000_0000_0000_0020) begin
      $fatal(1, "pixel width adapter did not zero-extend payload");
    end
    if (dut.pixels_in_tx__widen32to64_0_in_valid !== 1'b1 ||
        dut.pixels_in_tx__widen32to64_0_in_ready !== 1'b1 ||
        dut.widen32to64_0__pixels_out_rx_valid !== 1'b1 ||
        dut.widen32to64_0__pixels_out_rx_ready !== 1'b1) begin
      $fatal(1, "pixel width ready-valid handshake did not assert");
    end

    $display("SIM PASS T026_width_alt_stream_names");
    $finish;
  end
endmodule
