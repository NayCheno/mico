`default_nettype none

module register_seed_fifo_chain_formal_monitor (
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

    if (past_valid && $past(in_valid && !in_ready)) begin
      assert (in_payload == $past(in_payload));
    end
    if (past_valid && $past(out_valid && !out_ready)) begin
      assert (out_payload == $past(out_payload));
    end
  end
endmodule

bind Top register_seed_fifo_chain_formal_monitor mico_register_seed_fifo_formal_monitor (
  .clk(clk),
  .rst(rst),
  .in_payload(reg_write_source_tx__reg_buffer_input_payload),
  .in_valid(reg_write_source_tx__reg_buffer_input_valid),
  .in_ready(reg_write_source_tx__reg_buffer_input_ready),
  .out_payload(reg_buffer_output__reg_status_sink_rx_payload),
  .out_valid(reg_buffer_output__reg_status_sink_rx_valid),
  .out_ready(reg_buffer_output__reg_status_sink_rx_ready)
);

`default_nettype wire
