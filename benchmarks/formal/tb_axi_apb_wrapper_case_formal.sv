`default_nettype none

module axi_apb_wrapper_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] axi_payload,
  input wire        axi_valid,
  input wire        axi_ready,
  input wire [31:0] apb_payload,
  input wire        apb_valid,
  input wire        apb_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({axi_payload, axi_valid, axi_ready, apb_payload, apb_valid, apb_ready}));

    if (rst) begin
      assert (axi_valid == 1'b0);
      assert (apb_valid == 1'b0);
    end else begin
      assert (axi_payload == 32'h0000_0044);
      assert (apb_payload == 32'h0000_1044);
      assert (axi_valid == 1'b1);
      assert (axi_ready == 1'b1);
      assert (apb_valid == 1'b1);
      assert (apb_ready == 1'b1);
    end
  end
endmodule

bind Top axi_apb_wrapper_case_formal_monitor mico_axi_apb_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .axi_payload(axi_req__bridge_axi_payload),
  .axi_valid(axi_req__bridge_axi_valid),
  .axi_ready(axi_req__bridge_axi_ready),
  .apb_payload(bridge_apb__periph_req_payload),
  .apb_valid(bridge_apb__periph_req_valid),
  .apb_ready(bridge_apb__periph_req_ready)
);

`default_nettype wire
