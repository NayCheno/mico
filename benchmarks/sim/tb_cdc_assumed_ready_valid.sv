`timescale 1ns/1ps

module tb_cdc_assumed_ready_valid;
  logic aclk = 1'b0;
  logic arst_n = 1'b0;
  logic bclk = 1'b0;
  logic brst_n = 1'b0;

  Top dut (
    .aclk(aclk),
    .arst_n(arst_n),
    .bclk(bclk),
    .brst_n(brst_n)
  );

  always #5 aclk = ~aclk;
  always #7 bclk = ~bclk;

  initial begin
    repeat (2) @(posedge aclk);
    arst_n = 1'b1;
    brst_n = 1'b1;
    repeat (3) @(posedge bclk);
    #1;

    if (dut.dma0_tx__asyncfifo32_0_in_payload !== 32'h0000_00d0) begin
      $fatal(1, "declared CDC guarantee payload did not reach adapter input");
    end
    if (dut.asyncfifo32_0__aes0_rx_payload !== 32'h0000_00d0) begin
      $fatal(1, "declared CDC guarantee payload did not reach sink");
    end
    if (dut.dma0_tx__asyncfifo32_0_in_valid !== 1'b1 ||
        dut.dma0_tx__asyncfifo32_0_in_ready !== 1'b1 ||
        dut.asyncfifo32_0__aes0_rx_valid !== 1'b1 ||
        dut.asyncfifo32_0__aes0_rx_ready !== 1'b1) begin
      $fatal(1, "declared CDC ready-valid handshake did not assert");
    end

    $display("SIM PASS T028_cdc_assumed_ready_valid");
    $finish;
  end
endmodule
