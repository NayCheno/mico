`timescale 1ns/1ps

module tb_register_status_holdout_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.apb_cmd_req__reg_file_write_payload !== 32'h0000_0003) begin
      $fatal(1, "held-out APB command did not reach register file");
    end
    if (dut.reg_file_status__status_sink_status_payload !== 32'h8000_0003) begin
      $fatal(1, "held-out register status did not reach sink");
    end
    if (dut.apb_cmd_req__reg_file_write_valid !== 1'b1 ||
        dut.apb_cmd_req__reg_file_write_ready !== 1'b1 ||
        dut.reg_file_status__status_sink_status_valid !== 1'b1 ||
        dut.reg_file_status__status_sink_status_ready !== 1'b1) begin
      $fatal(1, "held-out register/status ready-valid chain did not assert");
    end

    $display("SIM PASS T073_register_status_holdout_case");
    $finish;
  end
endmodule
