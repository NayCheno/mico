`default_nettype none

// Hand-written reference wrapper for QoR comparison against generated Top.
module Top(
  input logic clk,
  input logic rst
);
  logic [31:0] p_tx__f_input_payload;
  logic p_tx__f_input_valid;
  logic p_tx__f_input_ready;
  logic [31:0] f_output__c_rx_payload;
  logic f_output__c_rx_valid;
  logic f_output__c_rx_ready;

  Producer p (
    .clk(clk),
    .rst(rst),
    .tx_payload(p_tx__f_input_payload),
    .tx_valid(p_tx__f_input_valid),
    .tx_ready(p_tx__f_input_ready)
  );

  Fifo f (
    .clk(clk),
    .rst(rst),
    .input_payload(p_tx__f_input_payload),
    .input_valid(p_tx__f_input_valid),
    .input_ready(p_tx__f_input_ready),
    .output_payload(f_output__c_rx_payload),
    .output_valid(f_output__c_rx_valid),
    .output_ready(f_output__c_rx_ready)
  );

  Consumer c (
    .clk(clk),
    .rst(rst),
    .rx_payload(f_output__c_rx_payload),
    .rx_valid(f_output__c_rx_valid),
    .rx_ready(f_output__c_rx_ready)
  );
endmodule

`default_nettype wire
