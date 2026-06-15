`timescale 1ns/1ps

module tb_dma_register_map_case;
  logic clk = 1'b0;
  logic rst = 1'b1;

  Top dut (.clk(clk), .rst(rst));

  always #5 clk = ~clk;

  initial begin
    repeat (2) @(posedge clk);
    rst = 1'b0;
    repeat (2) @(posedge clk);
    #1;

    if (dut.dma_cmd_tx__regs_write_payload !== 32'h0000_0015) begin
      $fatal(1, "DMA register command did not reach register file");
    end
    if (dut.regs_status__irq_status_status_payload !== 32'h8000_0015) begin
      $fatal(1, "register-map status did not reach interrupt/status sink");
    end
    if (dut.dma_cmd_tx__regs_write_valid !== 1'b1 ||
        dut.dma_cmd_tx__regs_write_ready !== 1'b1 ||
        dut.regs_status__irq_status_status_valid !== 1'b1 ||
        dut.regs_status__irq_status_status_ready !== 1'b1) begin
      $fatal(1, "DMA register-map ready-valid handshake did not assert");
    end

    $display("SIM PASS T077_dma_register_map_case");
    $finish;
  end
endmodule
