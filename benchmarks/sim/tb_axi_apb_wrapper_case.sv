`timescale 1ns/1ps

module tb_axi_apb_wrapper_case;
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

    if (dut.axi_req__bridge_axi_payload !== 32'h0000_0044) begin
      $fatal(1, "AXI command did not reach bridge");
    end
    if (dut.bridge_apb__periph_req_payload !== 32'h0000_1044) begin
      $fatal(1, "APB bridge transform did not reach peripheral");
    end
    if (dut.axi_req__bridge_axi_valid !== 1'b1 || dut.axi_req__bridge_axi_ready !== 1'b1) begin
      $fatal(1, "AXI/bridge handshake did not assert");
    end
    if (dut.bridge_apb__periph_req_valid !== 1'b1 || dut.bridge_apb__periph_req_ready !== 1'b1) begin
      $fatal(1, "bridge/peripheral handshake did not assert");
    end

    $display("SIM PASS T063_axi_apb_wrapper_case");
    $finish;
  end
endmodule
