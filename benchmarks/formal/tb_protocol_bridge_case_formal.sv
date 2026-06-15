`default_nettype none

module protocol_bridge_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] req_payload,
  input wire        req_valid,
  input wire        req_ready,
  input wire [31:0] rsp_payload,
  input wire        rsp_valid,
  input wire        rsp_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({req_payload, req_valid, req_ready, rsp_payload, rsp_valid, rsp_ready}));

    if (rst) begin
      assert (req_valid == 1'b0);
      assert (rsp_valid == 1'b0);
    end else begin
      assert (req_payload == 32'h0000_00a5);
      assert (rsp_payload == 32'h0000_a55a);
      assert (req_valid == 1'b1);
      assert (req_ready == 1'b1);
      assert (rsp_valid == 1'b1);
      assert (rsp_ready == 1'b1);
    end
  end
endmodule

bind Top protocol_bridge_case_formal_monitor mico_protocol_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .req_payload(source_req__bridge_req_payload),
  .req_valid(source_req__bridge_req_valid),
  .req_ready(source_req__bridge_req_ready),
  .rsp_payload(bridge_rsp__sink_rsp_payload),
  .rsp_valid(bridge_rsp__sink_rsp_valid),
  .rsp_ready(bridge_rsp__sink_rsp_ready)
);

`default_nettype wire
