`default_nettype none

module video_filter_pipeline_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] source_payload,
  input wire        source_valid,
  input wire        source_ready,
  input wire [31:0] line_payload,
  input wire        line_valid,
  input wire        line_ready,
  input wire [31:0] filter_payload,
  input wire        filter_valid,
  input wire        filter_ready
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
      filter_payload, filter_valid, filter_ready
    }));

    if (rst) begin
      assert (source_valid == 1'b0);
      assert (line_valid == 1'b0);
      assert (filter_valid == 1'b0);
    end else begin
      assert (source_payload == 32'h0000_0033);
      assert (line_payload == 32'h0000_0034);
      assert (filter_payload == 32'h8000_0034);
      assert (source_valid == 1'b1);
      assert (source_ready == 1'b1);
      assert (line_valid == 1'b1);
      assert (line_ready == 1'b1);
      assert (filter_valid == 1'b1);
      assert (filter_ready == 1'b1);
    end
  end
endmodule

bind Top video_filter_pipeline_case_formal_monitor mico_video_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .source_payload(source_tx__linebuf_input_payload),
  .source_valid(source_tx__linebuf_input_valid),
  .source_ready(source_tx__linebuf_input_ready),
  .line_payload(linebuf_output__filter_input_payload),
  .line_valid(linebuf_output__filter_input_valid),
  .line_ready(linebuf_output__filter_input_ready),
  .filter_payload(filter_output__sink_rx_payload),
  .filter_valid(filter_output__sink_rx_valid),
  .filter_ready(filter_output__sink_rx_ready)
);

`default_nettype wire
