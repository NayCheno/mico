`default_nettype none

module mmio_control_data_path_case_formal_monitor (
  input wire        clk,
  input wire        rst,
  input wire [31:0] ctrl_payload,
  input wire        ctrl_valid,
  input wire        ctrl_ready,
  input wire [63:0] cfg_payload,
  input wire        cfg_valid,
  input wire        cfg_ready,
  input wire [63:0] data_payload,
  input wire        data_valid,
  input wire        data_ready
);
  reg past_valid = 1'b0;

  always @(posedge clk) begin
    if (!past_valid) begin
      assume (rst == 1'b1);
    end else begin
      assume (rst == 1'b0);
    end
    past_valid <= 1'b1;

    assert (!$isunknown({
      ctrl_payload, ctrl_valid, ctrl_ready,
      cfg_payload, cfg_valid, cfg_ready,
      data_payload, data_valid, data_ready
    }));

    if (rst) begin
      assert (ctrl_valid == 1'b0);
      assert (cfg_valid == 1'b0);
      assert (data_valid == 1'b0);
    end else begin
      assert (ctrl_payload == 32'h0000_00c0);
      assert (cfg_payload == 64'h0000_0000_0000_00c0);
      assert (data_payload == 64'h0000_0000_0000_0fcf);
      assert (ctrl_valid == 1'b1);
      assert (ctrl_ready == 1'b1);
      assert (cfg_valid == 1'b1);
      assert (cfg_ready == 1'b1);
      assert (data_valid == 1'b1);
      assert (data_ready == 1'b1);
    end
  end
endmodule

bind Top mmio_control_data_path_case_formal_monitor mico_mmio_control_data_path_case_formal_monitor (
  .clk(clk),
  .rst(rst),
  .ctrl_payload(ctrl_tx__casewiden32to64_0_in_payload),
  .ctrl_valid(ctrl_tx__casewiden32to64_0_in_valid),
  .ctrl_ready(ctrl_tx__casewiden32to64_0_in_ready),
  .cfg_payload(casewiden32to64_0__datapath_cfg_payload),
  .cfg_valid(casewiden32to64_0__datapath_cfg_valid),
  .cfg_ready(casewiden32to64_0__datapath_cfg_ready),
  .data_payload(datapath_data__host_rx_payload),
  .data_valid(datapath_data__host_rx_valid),
  .data_ready(datapath_data__host_rx_ready)
);

`default_nettype wire
