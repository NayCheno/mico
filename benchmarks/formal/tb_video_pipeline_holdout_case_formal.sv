`default_nettype none

module video_pipeline_holdout_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] source_payload,
  input wire        source_valid,
  input wire        source_ready,
  input wire [31:0] line_payload,
  input wire        line_valid,
  input wire        line_ready,
  input wire [31:0] threshold_payload,
  input wire        threshold_valid,
  input wire        threshold_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({
      source_payload, source_valid, source_ready,
      line_payload, line_valid, line_ready,
      threshold_payload, threshold_valid, threshold_ready
    }));

    if (rst) begin
      assert (source_valid == 1'b0);
      assert (line_valid == 1'b0);
      assert (threshold_valid == 1'b0);
    end else begin
      assert (source_payload == 32'h0000_0033);
      assert (line_payload == 32'h0000_0034);
      assert (threshold_payload == 32'h8000_0034);
      assert (source_valid == 1'b1);
      assert (source_ready == 1'b1);
      assert (line_valid == 1'b1);
      assert (line_ready == 1'b1);
      assert (threshold_valid == 1'b1);
      assert (threshold_ready == 1'b1);
    end
  end
endmodule

bind Top video_pipeline_holdout_case_formal_monitor mico_video_holdout_formal_monitor (
  .clk(clk),
  .rst(rst),
  .source_payload(pixel_ingress_tx__line_stage_input_payload),
  .source_valid(pixel_ingress_tx__line_stage_input_valid),
  .source_ready(pixel_ingress_tx__line_stage_input_ready),
  .line_payload(line_stage_output__threshold_stage_input_payload),
  .line_valid(line_stage_output__threshold_stage_input_valid),
  .line_ready(line_stage_output__threshold_stage_input_ready),
  .threshold_payload(threshold_stage_output__frame_sink_rx_payload),
  .threshold_valid(threshold_stage_output__frame_sink_rx_valid),
  .threshold_ready(threshold_stage_output__frame_sink_rx_ready)
);

`default_nettype wire
