`default_nettype none

// Hand-written reference wrapper for QoR comparison against generated Top.
module Top(
  input logic aclk,
  input logic arst_n,
  input logic bclk,
  input logic brst_n
);
  logic [31:0] dma_tx__asyncfifo32_0_in_payload;
  logic dma_tx__asyncfifo32_0_in_valid;
  logic dma_tx__asyncfifo32_0_in_ready;
  logic [31:0] asyncfifo32_0__aes_rx_payload;
  logic asyncfifo32_0__aes_rx_valid;
  logic asyncfifo32_0__aes_rx_ready;

  Dma dma (
    .clk(aclk),
    .rst(arst_n),
    .tx_payload(dma_tx__asyncfifo32_0_in_payload),
    .tx_valid(dma_tx__asyncfifo32_0_in_valid),
    .tx_ready(dma_tx__asyncfifo32_0_in_ready)
  );

  Aes aes (
    .clk(bclk),
    .rst(brst_n),
    .rx_payload(asyncfifo32_0__aes_rx_payload),
    .rx_valid(asyncfifo32_0__aes_rx_valid),
    .rx_ready(asyncfifo32_0__aes_rx_ready)
  );

  AsyncFifo32 asyncfifo32_0 (
    .src_clk(aclk),
    .src_rst(arst_n),
    .dst_clk(bclk),
    .dst_rst(brst_n),
    .in_payload(dma_tx__asyncfifo32_0_in_payload),
    .in_valid(dma_tx__asyncfifo32_0_in_valid),
    .in_ready(dma_tx__asyncfifo32_0_in_ready),
    .out_payload(asyncfifo32_0__aes_rx_payload),
    .out_valid(asyncfifo32_0__aes_rx_valid),
    .out_ready(asyncfifo32_0__aes_rx_ready)
  );
endmodule

`default_nettype wire
