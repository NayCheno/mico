`timescale 1ns/1ps

module tb_fifo_chain_status_alias;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.status_src_tx__status_fifo_input_payload !== 32'h0000_0001) begin
      $fatal(1, "status payload did not reach fifo");
    end
    if (dut.status_fifo_output__status_sink_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "status fifo payload did not reach sink");
    end
    if (dut.status_src_tx__status_fifo_input_valid !== 1'b1 ||
        dut.status_src_tx__status_fifo_input_ready !== 1'b1 ||
        dut.status_fifo_output__status_sink_rx_valid !== 1'b1 ||
        dut.status_fifo_output__status_sink_rx_ready !== 1'b1) begin
      $fatal(1, "status fifo ready-valid handshake did not assert");
    end

    $display("SIM PASS T052_fifo_chain_status_alias");
    $finish;
  end
endmodule
