`default_nettype none

// Hand-written reference wrapper for QoR comparison against generated Top.
module Top(
  input logic clk,
  input logic rst
);
  logic [31:0] p_tx__c_rx_payload;
  logic p_tx__c_rx_valid;
  logic p_tx__c_rx_ready;

  Producer p (
    .clk(clk),
    .rst(rst),
    .tx_payload(p_tx__c_rx_payload),
    .tx_valid(p_tx__c_rx_valid),
    .tx_ready(p_tx__c_rx_ready)
  );

  Consumer c (
    .clk(clk),
    .rst(rst),
    .rx_payload(p_tx__c_rx_payload),
    .rx_valid(p_tx__c_rx_valid),
    .rx_ready(p_tx__c_rx_ready)
  );
endmodule

`default_nettype wire
