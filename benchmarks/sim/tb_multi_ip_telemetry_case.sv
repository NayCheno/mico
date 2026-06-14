`timescale 1ns/1ps

module tb_multi_ip_telemetry_case;
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

    if (dut.source_tx__filter_input_payload !== 32'h1357_2468) begin
      $fatal(1, "telemetry source payload did not reach filter");
    end
    if (dut.filter_output__casewiden32to64_1_in_payload !== 32'h13a8_2497) begin
      $fatal(1, "telemetry filter payload did not reach width adapter");
    end
    if (dut.casewiden32to64_1__accum_rx_payload !== 64'h0000_0000_13a8_2497) begin
      $fatal(1, "telemetry widened payload did not reach accumulator");
    end
    if (dut.accum_tx__host_rx_payload !== 64'h0000_0000_24b9_2497) begin
      $fatal(1, "telemetry accumulator payload did not reach host");
    end
    if (dut.source_tx__filter_input_valid !== 1'b1 || dut.source_tx__filter_input_ready !== 1'b1) begin
      $fatal(1, "source/filter handshake did not assert");
    end
    if (dut.filter_output__casewiden32to64_1_in_valid !== 1'b1 || dut.filter_output__casewiden32to64_1_in_ready !== 1'b1) begin
      $fatal(1, "filter/adapter handshake did not assert");
    end
    if (dut.casewiden32to64_1__accum_rx_valid !== 1'b1 || dut.casewiden32to64_1__accum_rx_ready !== 1'b1) begin
      $fatal(1, "adapter/accumulator handshake did not assert");
    end
    if (dut.accum_tx__host_rx_valid !== 1'b1 || dut.accum_tx__host_rx_ready !== 1'b1) begin
      $fatal(1, "accumulator/host handshake did not assert");
    end

    $display("SIM PASS T062_multi_ip_telemetry_case");
    $finish;
  end
endmodule
