`timescale 1ns/1ps

module tb_mmio_control_data_path_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.ctrl_tx__casewiden32to64_0_in_payload !== 32'h0000_00c0) begin
      $fatal(1, "MMIO control word did not reach width adapter");
    end
    if (dut.casewiden32to64_0__datapath_cfg_payload !== 64'h0000_0000_0000_00c0) begin
      $fatal(1, "MMIO control word was not zero-extended");
    end
    if (dut.datapath_data__host_rx_payload !== 64'h0000_0000_0000_0fcf) begin
      $fatal(1, "MMIO data path payload did not reach host");
    end
    if (dut.ctrl_tx__casewiden32to64_0_in_valid !== 1'b1 ||
        dut.ctrl_tx__casewiden32to64_0_in_ready !== 1'b1 ||
        dut.casewiden32to64_0__datapath_cfg_valid !== 1'b1 ||
        dut.casewiden32to64_0__datapath_cfg_ready !== 1'b1 ||
        dut.datapath_data__host_rx_valid !== 1'b1 ||
        dut.datapath_data__host_rx_ready !== 1'b1) begin
      $fatal(1, "MMIO control/data ready-valid handshake did not assert");
    end

    $display("SIM PASS T081_mmio_control_data_path_case");
    $finish;
  end
endmodule
