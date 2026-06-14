`default_nettype none

module Top(
  input logic clk,
  input logic rst
);
  logic [31:0] cmd_req__regs_write_payload;
  logic cmd_req__regs_write_ready;
  logic cmd_req__regs_write_valid;
  logic [31:0] regs_status__sink_status_payload;
  logic regs_status__sink_status_ready;
  logic regs_status__sink_status_valid;

  CaseApbCommandSource cmd (
    .clk(clk),
    .rst(rst),
    .req_payload(cmd_req__regs_write_payload),
    .req_valid(cmd_req__regs_write_valid),
    .req_ready(cmd_req__regs_write_ready)
  );

  CaseRegisterFile regs (
    .clk(clk),
    .rst(rst),
    .write_payload(cmd_req__regs_write_payload),
    .write_valid(cmd_req__regs_write_valid),
    .write_ready(cmd_req__regs_write_ready),
    .status_payload(regs_status__sink_status_payload),
    .status_valid(regs_status__sink_status_valid),
    .status_ready(regs_status__sink_status_ready)
  );

  CaseStatusSink sink (
    .clk(clk),
    .rst(rst),
    .status_payload(regs_status__sink_status_payload),
    .status_valid(regs_status__sink_status_valid),
    .status_ready(regs_status__sink_status_ready)
  );
endmodule

`default_nettype wire
