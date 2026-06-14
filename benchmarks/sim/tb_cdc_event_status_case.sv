`timescale 1ns/1ps

module tb_cdc_event_status_case;
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

    if (dut.source_tx__caseasyncfifo32_0_in_payload !== 32'h0000_0e11) begin
      $fatal(1, "event payload did not reach CDC adapter input");
    end
    if (dut.caseasyncfifo32_0__sink_rx_payload !== 32'h0000_0e11) begin
      $fatal(1, "CDC adapter payload did not reach status sink");
    end
    if (dut.source_tx__caseasyncfifo32_0_in_valid !== 1'b1 ||
        dut.source_tx__caseasyncfifo32_0_in_ready !== 1'b1 ||
        dut.caseasyncfifo32_0__sink_rx_valid !== 1'b1 ||
        dut.caseasyncfifo32_0__sink_rx_ready !== 1'b1) begin
      $fatal(1, "CDC smoke handshake did not assert");
    end

    $display("SIM PASS T065_cdc_event_status_case");
    $finish;
  end
endmodule
