`default_nettype none

module streaming_accelerator_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] dma_payload,
  input wire        dma_valid,
  input wire        dma_ready,
  input wire [31:0] skid_payload,
  input wire        skid_valid,
  input wire        skid_ready,
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
      dma_payload, dma_valid, dma_ready,
      skid_payload, skid_valid, skid_ready,
      filter_payload, filter_valid, filter_ready
    }));

    if (rst) begin
      assert (dma_valid == 1'b0);
      assert (skid_valid == 1'b0);
      assert (filter_valid == 1'b0);
      assert (filter_ready == 1'b0);
    end else begin
      assert (dma_payload == 32'hcafe_0011);
      assert (skid_payload == 32'hcafe_0011);
      assert (filter_payload == 32'hcafe_00ee);
      assert (dma_valid == 1'b1);
      assert (skid_valid == 1'b1);
      assert (filter_valid == 1'b1);
      assert (dma_ready == 1'b1);
      assert (skid_ready == 1'b1);
      assert (filter_ready == 1'b1);
    end

    if (past_valid && $past(filter_valid && !filter_ready)) begin
      assert (filter_payload == $past(filter_payload));
    end
  end
endmodule

bind Top streaming_accelerator_case_formal_monitor mico_streaming_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .dma_payload(dma_tx__skid_input_payload),
  .dma_valid(dma_tx__skid_input_valid),
  .dma_ready(dma_tx__skid_input_ready),
  .skid_payload(skid_output__filter_input_payload),
  .skid_valid(skid_output__filter_input_valid),
  .skid_ready(skid_output__filter_input_ready),
  .filter_payload(filter_output__sink_rx_payload),
  .filter_valid(filter_output__sink_rx_valid),
  .filter_ready(filter_output__sink_rx_ready)
);

`default_nettype wire
