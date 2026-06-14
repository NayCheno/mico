`default_nettype none

module width_alt_stream_names_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] in_payload,
  input wire        in_valid,
  input wire        in_ready,
  input wire [63:0] out_payload,
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
      assert (out_valid == 1'b0);
      assert (in_ready == 1'b0);
      assert (out_ready == 1'b0);
    end else begin
      assert (out_payload[31:0] == in_payload);
      assert (out_payload[63:32] == 32'h0000_0000);
      assert (out_valid == in_valid);
      assert (in_ready == out_ready);
    end
  end
endmodule

bind Top width_alt_stream_names_case_formal_monitor mico_width_alt_names_formal_monitor (
  .clk(clk),
  .rst(rst),
  .in_payload(pixels_in_tx__widen32to64_0_in_payload),
  .in_valid(pixels_in_tx__widen32to64_0_in_valid),
  .in_ready(pixels_in_tx__widen32to64_0_in_ready),
  .out_payload(widen32to64_0__pixels_out_rx_payload),
  .out_valid(widen32to64_0__pixels_out_rx_valid),
  .out_ready(widen32to64_0__pixels_out_rx_ready)
);

`default_nettype wire
