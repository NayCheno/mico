`timescale 1ns/1ps

module tb_protocol_bridge_case;
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

    if (dut.source_req__bridge_req_payload !== 32'h0000_00a5) begin
      $fatal(1, "protocol source payload did not reach bridge");
    end
    if (dut.bridge_rsp__sink_rsp_payload !== 32'h0000_a55a) begin
      $fatal(1, "protocol bridge response payload did not reach sink");
    end
    if (dut.source_req__bridge_req_valid !== 1'b1 || dut.source_req__bridge_req_ready !== 1'b1) begin
      $fatal(1, "source/bridge handshake did not assert");
    end
    if (dut.bridge_rsp__sink_rsp_valid !== 1'b1 || dut.bridge_rsp__sink_rsp_ready !== 1'b1) begin
      $fatal(1, "bridge/sink handshake did not assert");
    end

    $display("SIM PASS T061_protocol_bridge_case");
    $finish;
  end
endmodule
