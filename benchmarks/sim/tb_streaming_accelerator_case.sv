`timescale 1ns/1ps

module tb_streaming_accelerator_case;
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

    if (dut.dma_tx__skid_input_payload !== 32'hcafe_0011) begin
      $fatal(1, "DMA payload did not reach skid buffer");
    end
    if (dut.skid_output__filter_input_payload !== 32'hcafe_0011) begin
      $fatal(1, "skid output did not reach filter");
    end
    if (dut.filter_output__sink_rx_payload !== 32'hcafe_00ee) begin
      $fatal(1, "filter result did not reach sink");
    end
    if (dut.dma_tx__skid_input_valid !== 1'b1 || dut.dma_tx__skid_input_ready !== 1'b1) begin
      $fatal(1, "DMA/skid handshake did not assert");
    end
    if (dut.skid_output__filter_input_valid !== 1'b1 || dut.skid_output__filter_input_ready !== 1'b1) begin
      $fatal(1, "skid/filter handshake did not assert");
    end
    if (dut.filter_output__sink_rx_valid !== 1'b1 || dut.filter_output__sink_rx_ready !== 1'b1) begin
      $fatal(1, "filter/sink handshake did not assert");
    end

    $display("SIM PASS T058_streaming_accelerator_case");
    $finish;
  end
endmodule
