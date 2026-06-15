`default_nettype none

module register_status_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] cmd_payload,
  input wire        cmd_valid,
  input wire        cmd_ready,
  input wire [31:0] status_payload,
  input wire        status_valid,
  input wire        status_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({cmd_payload, cmd_valid, cmd_ready, status_payload, status_valid, status_ready}));

    if (rst) begin
      assert (cmd_valid == 1'b0);
      assert (status_valid == 1'b0);
    end else begin
      assert (cmd_payload == 32'h0000_0003);
      assert (status_payload == 32'h8000_0003);
      assert (cmd_valid == 1'b1);
      assert (cmd_ready == 1'b1);
      assert (status_valid == 1'b1);
      assert (status_ready == 1'b1);
    end
  end
endmodule

bind Top register_status_case_formal_monitor mico_register_status_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .cmd_payload(cmd_req__regs_write_payload),
  .cmd_valid(cmd_req__regs_write_valid),
  .cmd_ready(cmd_req__regs_write_ready),
  .status_payload(regs_status__sink_status_payload),
  .status_valid(regs_status__sink_status_valid),
  .status_ready(regs_status__sink_status_ready)
);

`default_nettype wire
