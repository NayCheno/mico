`timescale 1ns/1ps

module tb_width_protocol_bridge_case;
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

    if (dut.sensor_tx__casewiden32to64_0_in_payload !== 32'h1234_5678) begin
      $fatal(1, "sensor payload did not reach width adapter");
    end
    if (dut.casewiden32to64_0__accel_rx_payload !== 64'h0000_0000_1234_5678) begin
      $fatal(1, "width adapter did not zero-extend sensor payload");
    end
    if (dut.accel_tx__host_rx_payload !== 64'h0000_0000_1234_6678) begin
      $fatal(1, "accelerator result did not reach host sink");
    end
    if (dut.sensor_tx__casewiden32to64_0_in_valid !== 1'b1 ||
        dut.sensor_tx__casewiden32to64_0_in_ready !== 1'b1) begin
      $fatal(1, "sensor/adapter handshake did not assert");
    end
    if (dut.casewiden32to64_0__accel_rx_valid !== 1'b1 ||
        dut.casewiden32to64_0__accel_rx_ready !== 1'b1) begin
      $fatal(1, "adapter/accelerator handshake did not assert");
    end
    if (dut.accel_tx__host_rx_valid !== 1'b1 || dut.accel_tx__host_rx_ready !== 1'b1) begin
      $fatal(1, "accelerator/host handshake did not assert");
    end

    $display("SIM PASS T059_width_protocol_bridge_case");
    $finish;
  end
endmodule
