`timescale 1ns/1ps

module tb_register_seed_fifo_chain;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.reg_write_source_tx__reg_buffer_input_payload !== 32'h0000_0001) begin
      $fatal(1, "register write payload did not reach buffer");
    end
    if (dut.reg_buffer_output__reg_status_sink_rx_payload !== 32'h0000_0001) begin
      $fatal(1, "register buffer payload did not reach status sink");
    end
    if (dut.reg_write_source_tx__reg_buffer_input_valid !== 1'b1 ||
        dut.reg_write_source_tx__reg_buffer_input_ready !== 1'b1 ||
        dut.reg_buffer_output__reg_status_sink_rx_valid !== 1'b1 ||
        dut.reg_buffer_output__reg_status_sink_rx_ready !== 1'b1) begin
      $fatal(1, "register fifo ready-valid handshake did not assert");
    end

    $display("SIM PASS T056_register_seed_fifo_chain");
    $finish;
  end
endmodule
