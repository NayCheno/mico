`timescale 1ns/1ps

module tb_axis_packetizer_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.word_source_tx__packetizer_word_payload !== 32'h0000_005a) begin
      $fatal(1, "AXI-stream word did not reach packetizer");
    end
    if (dut.packetizer_packet__packet_sink_rx_payload !== 64'hca5e_0001_0000_005a) begin
      $fatal(1, "packetized payload did not reach packet sink");
    end
    if (dut.word_source_tx__packetizer_word_valid !== 1'b1 ||
        dut.word_source_tx__packetizer_word_ready !== 1'b1 ||
        dut.packetizer_packet__packet_sink_rx_valid !== 1'b1 ||
        dut.packetizer_packet__packet_sink_rx_ready !== 1'b1) begin
      $fatal(1, "AXI-stream packetizer ready-valid handshake did not assert");
    end

    $display("SIM PASS T079_axis_packetizer_case");
    $finish;
  end
endmodule
