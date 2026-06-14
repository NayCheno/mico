`default_nettype none

module subsystem_seed_fifo_chain_formal_monitor (
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

bind Top subsystem_seed_fifo_chain_formal_monitor mico_subsystem_seed_formal_monitor (
  .clk(clk),
  .rst(rst),
  .in_payload(ingress_tx__buffer0_input_payload),
  .in_valid(ingress_tx__buffer0_input_valid),
  .in_ready(ingress_tx__buffer0_input_ready),
  .out_payload(buffer0_output__egress_rx_payload),
  .out_valid(buffer0_output__egress_rx_valid),
  .out_ready(buffer0_output__egress_rx_ready)
);

`default_nettype wire
