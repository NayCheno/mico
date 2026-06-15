`default_nettype none

module multi_ip_telemetry_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] source_payload,
  input wire        source_valid,
  input wire        source_ready,
  input wire [31:0] filter_payload,
  input wire        filter_valid,
  input wire        filter_ready,
  input wire [63:0] widen_payload,
  input wire        widen_valid,
  input wire        widen_ready,
  input wire [63:0] accum_payload,
  input wire        accum_valid,
  input wire        accum_ready
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
      filter_payload, filter_valid, filter_ready,
      widen_payload, widen_valid, widen_ready,
      accum_payload, accum_valid, accum_ready
    }));

    if (rst) begin
      assert (source_valid == 1'b0);
      assert (filter_valid == 1'b0);
      assert (widen_valid == 1'b0);
      assert (accum_valid == 1'b0);
    end else begin
      assert (source_payload == 32'h1357_2468);
      assert (filter_payload == 32'h13a8_2497);
      assert (widen_payload == 64'h0000_0000_13a8_2497);
      assert (accum_payload == 64'h0000_0000_24b9_2497);
      assert (source_valid == 1'b1);
      assert (source_ready == 1'b1);
      assert (filter_valid == 1'b1);
      assert (filter_ready == 1'b1);
      assert (widen_valid == 1'b1);
      assert (widen_ready == 1'b1);
      assert (accum_valid == 1'b1);
      assert (accum_ready == 1'b1);
    end
  end
endmodule

bind Top multi_ip_telemetry_case_formal_monitor mico_multi_ip_telemetry_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .source_payload(source_tx__filter_input_payload),
  .source_valid(source_tx__filter_input_valid),
  .source_ready(source_tx__filter_input_ready),
  .filter_payload(filter_output__casewiden32to64_1_in_payload),
  .filter_valid(filter_output__casewiden32to64_1_in_valid),
  .filter_ready(filter_output__casewiden32to64_1_in_ready),
  .widen_payload(casewiden32to64_1__accum_rx_payload),
  .widen_valid(casewiden32to64_1__accum_rx_valid),
  .widen_ready(casewiden32to64_1__accum_rx_ready),
  .accum_payload(accum_tx__host_rx_payload),
  .accum_valid(accum_tx__host_rx_valid),
  .accum_ready(accum_tx__host_rx_ready)
);

`default_nettype wire
