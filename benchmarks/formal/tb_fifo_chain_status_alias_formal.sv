`default_nettype none

module fifo_chain_status_alias_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] in_payload,
  input wire        in_valid,
  input wire        in_ready,
  input wire [31:0] out_payload,
  input wire        out_valid,
  input wire        out_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({in_payload, in_valid, in_ready, out_payload, out_valid, out_ready}));

    if (rst) begin
      assert (in_valid == 1'b0);
      assert (in_ready == 1'b0);
      assert (out_valid == 1'b0);
      assert (out_ready == 1'b0);
    end else begin
      assert (in_payload == 32'h0000_0001);
      assert (out_payload == 32'h0000_0001);
      assert (in_valid == 1'b1);
      assert (in_ready == 1'b1);
      assert (out_valid == 1'b1);
      assert (out_ready == 1'b1);
    end
  end
endmodule

bind Top fifo_chain_status_alias_formal_monitor mico_status_alias_formal_monitor (
  .clk(clk),
  .rst(rst),
  .in_payload(status_src_tx__status_fifo_input_payload),
  .in_valid(status_src_tx__status_fifo_input_valid),
  .in_ready(status_src_tx__status_fifo_input_ready),
  .out_payload(status_fifo_output__status_sink_rx_payload),
  .out_valid(status_fifo_output__status_sink_rx_valid),
  .out_ready(status_fifo_output__status_sink_rx_ready)
);

`default_nettype wire
