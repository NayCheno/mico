`default_nettype none

module axis_packetizer_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] word_payload,
  input wire        word_valid,
  input wire        word_ready,
  input wire [63:0] packet_payload,
  input wire        packet_valid,
  input wire        packet_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({word_payload, word_valid, word_ready, packet_payload, packet_valid, packet_ready}));

    if (rst) begin
      assert (word_valid == 1'b0);
      assert (packet_valid == 1'b0);
    end else begin
      assert (word_payload == 32'h0000_005a);
      assert (packet_payload == 64'hca5e_0001_0000_005a);
      assert (word_valid == 1'b1);
      assert (word_ready == 1'b1);
      assert (packet_valid == 1'b1);
      assert (packet_ready == 1'b1);
    end
  end
endmodule

bind Top axis_packetizer_case_formal_monitor mico_axis_packetizer_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .word_payload(word_source_tx__packetizer_word_payload),
  .word_valid(word_source_tx__packetizer_word_valid),
  .word_ready(word_source_tx__packetizer_word_ready),
  .packet_payload(packetizer_packet__packet_sink_rx_payload),
  .packet_valid(packetizer_packet__packet_sink_rx_valid),
  .packet_ready(packetizer_packet__packet_sink_rx_ready)
);

`default_nettype wire
