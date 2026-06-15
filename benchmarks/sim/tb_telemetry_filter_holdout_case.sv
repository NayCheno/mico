`timescale 1ns/1ps

module tb_telemetry_filter_holdout_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.telemetry_src_tx__telemetry_filter_input_payload !== 32'h1357_2468) begin
      $fatal(1, "held-out telemetry source payload did not reach filter");
    end
    if (dut.telemetry_filter_output__casewiden32to64_1_in_payload !== 32'h13a8_2497) begin
      $fatal(1, "held-out telemetry filter payload did not reach adapter");
    end
    if (dut.casewiden32to64_1__telemetry_accum_rx_payload !== 64'h0000_0000_13a8_2497) begin
      $fatal(1, "held-out telemetry widened payload did not reach accumulator");
    end
    if (dut.telemetry_accum_tx__host_sink_rx_payload !== 64'h0000_0000_24b9_2497) begin
      $fatal(1, "held-out telemetry accumulator payload did not reach host");
    end
    if (dut.telemetry_src_tx__telemetry_filter_input_valid !== 1'b1 ||
        dut.telemetry_src_tx__telemetry_filter_input_ready !== 1'b1 ||
        dut.telemetry_filter_output__casewiden32to64_1_in_valid !== 1'b1 ||
        dut.telemetry_filter_output__casewiden32to64_1_in_ready !== 1'b1 ||
        dut.casewiden32to64_1__telemetry_accum_rx_valid !== 1'b1 ||
        dut.casewiden32to64_1__telemetry_accum_rx_ready !== 1'b1 ||
        dut.telemetry_accum_tx__host_sink_rx_valid !== 1'b1 ||
        dut.telemetry_accum_tx__host_sink_rx_ready !== 1'b1) begin
      $fatal(1, "held-out telemetry ready-valid chain did not assert");
    end

    $display("SIM PASS T069_telemetry_filter_holdout_case");
    $finish;
  end
endmodule
