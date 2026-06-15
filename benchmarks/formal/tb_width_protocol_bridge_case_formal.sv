`default_nettype none

module width_protocol_bridge_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] sensor_payload,
  input wire        sensor_valid,
  input wire        sensor_ready,
  input wire [63:0] widen_payload,
  input wire        widen_valid,
  input wire        widen_ready,
  input wire [63:0] accel_payload,
  input wire        accel_valid,
  input wire        accel_ready
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
      sensor_payload, sensor_valid, sensor_ready,
      widen_payload, widen_valid, widen_ready,
      accel_payload, accel_valid, accel_ready
    }));

    if (rst) begin
      assert (sensor_valid == 1'b0);
      assert (widen_valid == 1'b0);
      assert (accel_valid == 1'b0);
    end else begin
      assert (sensor_payload == 32'h1234_5678);
      assert (widen_payload == 64'h0000_0000_1234_5678);
      assert (accel_payload == 64'h0000_0000_1234_6678);
      assert (sensor_valid == 1'b1);
      assert (sensor_ready == 1'b1);
      assert (widen_valid == 1'b1);
      assert (widen_ready == 1'b1);
      assert (accel_valid == 1'b1);
      assert (accel_ready == 1'b1);
    end
  end
endmodule

bind Top width_protocol_bridge_case_formal_monitor mico_width_protocol_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .sensor_payload(sensor_tx__casewiden32to64_0_in_payload),
  .sensor_valid(sensor_tx__casewiden32to64_0_in_valid),
  .sensor_ready(sensor_tx__casewiden32to64_0_in_ready),
  .widen_payload(casewiden32to64_0__accel_rx_payload),
  .widen_valid(casewiden32to64_0__accel_rx_valid),
  .widen_ready(casewiden32to64_0__accel_rx_ready),
  .accel_payload(accel_tx__host_rx_payload),
  .accel_valid(accel_tx__host_rx_valid),
  .accel_ready(accel_tx__host_rx_ready)
);

`default_nettype wire
