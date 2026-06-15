`timescale 1ns/1ps

module tb_protocol_bridge_holdout_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.req_source_req__req_rsp_bridge_req_payload !== 32'h0000_00a5) begin
      $fatal(1, "held-out protocol request did not reach bridge");
    end
    if (dut.req_rsp_bridge_rsp__rsp_sink_rsp_payload !== 32'h0000_a55a) begin
      $fatal(1, "held-out protocol response did not reach sink");
    end
    if (dut.req_source_req__req_rsp_bridge_req_valid !== 1'b1 ||
        dut.req_source_req__req_rsp_bridge_req_ready !== 1'b1 ||
        dut.req_rsp_bridge_rsp__rsp_sink_rsp_valid !== 1'b1 ||
        dut.req_rsp_bridge_rsp__rsp_sink_rsp_ready !== 1'b1) begin
      $fatal(1, "held-out protocol ready-valid chain did not assert");
    end

    $display("SIM PASS T071_protocol_bridge_holdout_case");
    $finish;
  end
endmodule
