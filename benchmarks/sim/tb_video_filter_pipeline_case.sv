`timescale 1ns/1ps

module tb_video_filter_pipeline_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (
    .clk(clk),
    .rst(rst)
  );

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.source_tx__linebuf_input_payload !== 32'h0000_0033) begin
      $fatal(1, "pixel source payload did not reach line buffer");
    end
    if (dut.linebuf_output__filter_input_payload !== 32'h0000_0034) begin
      $fatal(1, "line buffer payload did not reach filter");
    end
    if (dut.filter_output__sink_rx_payload !== 32'h8000_0034) begin
      $fatal(1, "threshold filter payload did not reach frame sink");
    end
    if (dut.source_tx__linebuf_input_valid !== 1'b1 || dut.source_tx__linebuf_input_ready !== 1'b1) begin
      $fatal(1, "source/line buffer handshake did not assert");
    end
    if (dut.linebuf_output__filter_input_valid !== 1'b1 || dut.linebuf_output__filter_input_ready !== 1'b1) begin
      $fatal(1, "line buffer/filter handshake did not assert");
    end
    if (dut.filter_output__sink_rx_valid !== 1'b1 || dut.filter_output__sink_rx_ready !== 1'b1) begin
      $fatal(1, "filter/sink handshake did not assert");
    end

    $display("SIM PASS T064_video_filter_pipeline_case");
    $finish;
  end
endmodule
