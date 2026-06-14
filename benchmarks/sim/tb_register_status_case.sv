`timescale 1ns/1ps

module tb_register_status_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (
    .clk(clk),
    .rst(rst)
  );

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.cmd_req__regs_write_payload !== 32'h0000_0003) begin
      $fatal(1, "command word did not reach register file");
    end
    if (dut.regs_status__sink_status_payload !== 32'h8000_0003) begin
      $fatal(1, "status word did not reach sink");
    end
    if (dut.cmd_req__regs_write_valid !== 1'b1 || dut.cmd_req__regs_write_ready !== 1'b1) begin
      $fatal(1, "command/register handshake did not assert");
    end
    if (dut.regs_status__sink_status_valid !== 1'b1 ||
        dut.regs_status__sink_status_ready !== 1'b1) begin
      $fatal(1, "register/status handshake did not assert");
    end

    $display("SIM PASS T060_register_status_case");
    $finish;
  end
endmodule
