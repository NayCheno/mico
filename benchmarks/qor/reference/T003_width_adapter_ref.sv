`default_nettype none

// Hand-written reference wrapper for QoR comparison against generated Top.
module Top(
  input logic clk,
  input logic rst
);
  logic [31:0] s_tx__widen32to64_0_in_payload;
  logic s_tx__widen32to64_0_in_valid;
  logic s_tx__widen32to64_0_in_ready;
  logic [63:0] widen32to64_0__t_rx_payload;
  logic widen32to64_0__t_rx_valid;
  logic widen32to64_0__t_rx_ready;

  Source32 s (
    .clk(clk),
    .rst(rst),
    .tx_payload(s_tx__widen32to64_0_in_payload),
    .tx_valid(s_tx__widen32to64_0_in_valid),
    .tx_ready(s_tx__widen32to64_0_in_ready)
  );

  Sink64 t (
    .clk(clk),
    .rst(rst),
    .rx_payload(widen32to64_0__t_rx_payload),
    .rx_valid(widen32to64_0__t_rx_valid),
    .rx_ready(widen32to64_0__t_rx_ready)
  );

  Widen32To64 widen32to64_0 (
    .clk(clk),
    .rst(rst),
    .in_payload(s_tx__widen32to64_0_in_payload),
    .in_valid(s_tx__widen32to64_0_in_valid),
    .in_ready(s_tx__widen32to64_0_in_ready),
    .out_payload(widen32to64_0__t_rx_payload),
    .out_valid(widen32to64_0__t_rx_valid),
    .out_ready(widen32to64_0__t_rx_ready)
  );
endmodule

`default_nettype wire
