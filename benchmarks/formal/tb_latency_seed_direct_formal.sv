`default_nettype none

module latency_seed_direct_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] payload,
  input wire        valid,
  input wire        ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({payload, valid, ready}));

    if (rst) begin
      assert (valid == 1'b0);
      assert (ready == 1'b0);
    end else begin
      assert (payload == 32'h0000_0001);
      assert (valid == 1'b1);
      assert (ready == 1'b1);
    end

    if (past_valid && $past(valid && !ready)) begin
      assert (payload == $past(payload));
    end
  end
endmodule

bind Top latency_seed_direct_formal_monitor mico_latency_direct_formal_monitor (
  .clk(clk),
  .rst(rst),
  .payload(src0_tx__dst0_rx_payload),
  .valid(src0_tx__dst0_rx_valid),
  .ready(src0_tx__dst0_rx_ready)
);

`default_nettype wire
