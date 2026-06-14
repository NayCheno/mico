`timescale 1ns/1ps

module tb_fifo_chain_alt_names;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.source_tx__queue_input_payload !== 32'h0000_0001) begin
      $fatal(1, "source payload did not reach queue");
    end
    if (dut.queue_output__sink_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "queue payload did not reach sink");
    end
    if (dut.source_tx__queue_input_valid !== 1'b1 || dut.source_tx__queue_input_ready !== 1'b1 ||
        dut.queue_output__sink_rx_valid !== 1'b1 || dut.queue_output__sink_rx_ready !== 1'b1) begin
      $fatal(1, "fifo alt handshake did not assert");
    end

    $display("SIM PASS T015_fifo_chain_alt_names");
    $finish;
  end
endmodule
