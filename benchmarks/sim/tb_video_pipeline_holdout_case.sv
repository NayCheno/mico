`timescale 1ns/1ps

module tb_video_pipeline_holdout_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.pixel_ingress_tx__line_stage_input_payload !== 32'h0000_0033) begin
      $fatal(1, "held-out pixel payload did not reach line stage");
    end
    if (dut.line_stage_output__threshold_stage_input_payload !== 32'h0000_0034) begin
      $fatal(1, "held-out line stage payload did not reach threshold stage");
    end
    if (dut.threshold_stage_output__frame_sink_rx_payload !== 32'h8000_0034) begin
      $fatal(1, "held-out threshold payload did not reach frame sink");
    end
    if (dut.pixel_ingress_tx__line_stage_input_valid !== 1'b1 ||
        dut.pixel_ingress_tx__line_stage_input_ready !== 1'b1 ||
        dut.line_stage_output__threshold_stage_input_valid !== 1'b1 ||
        dut.line_stage_output__threshold_stage_input_ready !== 1'b1 ||
        dut.threshold_stage_output__frame_sink_rx_valid !== 1'b1 ||
        dut.threshold_stage_output__frame_sink_rx_ready !== 1'b1) begin
      $fatal(1, "held-out video ready-valid chain did not assert");
    end

    $display("SIM PASS T075_video_pipeline_holdout_case");
    $finish;
  end
endmodule
