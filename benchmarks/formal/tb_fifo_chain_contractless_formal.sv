`default_nettype none

module fifo_chain_contractless_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] in_payload,
  input wire        in_valid,
  input wire        in_ready,
  input wire [31:0] out_payload,
  input wire        out_valid,
  input wire        out_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({in_payload, in_valid, in_ready, out_payload, out_valid, out_ready}));

    if (rst) begin
      assert (in_valid == 1'b0);
      assert (in_ready == 1'b0);
      assert (out_valid == 1'b0);
      assert (out_ready == 1'b0);
    end else begin
      assert (in_payload == 32'h0000_0001);
      assert (out_payload == 32'h0000_0001);
      assert (in_valid == 1'b1);
      assert (in_ready == 1'b1);
      assert (out_valid == 1'b1);
      assert (out_ready == 1'b1);
    end
  end
endmodule

bind Top fifo_chain_contractless_formal_monitor mico_fifo_raw_formal_monitor (
  .clk(clk),
  .rst(rst),
  .in_payload(p_tx__f_input_payload),
  .in_valid(p_tx__f_input_valid),
  .in_ready(p_tx__f_input_ready),
  .out_payload(f_output__c_rx_payload),
  .out_valid(f_output__c_rx_valid),
  .out_ready(f_output__c_rx_ready)
);

`default_nettype wire
