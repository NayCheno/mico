`default_nettype none

module Producer (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h0000_0001;
    tx_valid = !rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module Fifo (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] input_payload,
  input  logic        input_valid,
  output logic        input_ready,
  output logic [31:0] output_payload,
  output logic        output_valid,
  input  logic        output_ready
);
  always_comb begin
    output_payload = input_payload;
    output_valid = input_valid;
    input_ready = output_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module Consumer (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] rx_payload,
  input  logic        rx_valid,
  output logic        rx_ready
);
  always_comb begin
    rx_ready = !rst;
  end

  wire unused_inputs = clk ^ rx_valid ^ rx_payload[0];
endmodule

module Dma (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h0000_00d0;
    tx_valid = rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module Aes (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] rx_payload,
  input  logic        rx_valid,
  output logic        rx_ready
);
  always_comb begin
    rx_ready = rst;
  end

  wire unused_inputs = clk ^ rx_valid ^ rx_payload[0];
endmodule

// Smoke-only CDC adapter stub. It is not a CDC correctness proof.
module AsyncFifo32 (
  input  logic        src_clk,
  input  logic        src_rst,
  input  logic        dst_clk,
  input  logic        dst_rst,
  input  logic [31:0] in_payload,
  input  logic        in_valid,
  output logic        in_ready,
  output logic [31:0] out_payload,
  output logic        out_valid,
  input  logic        out_ready
);
  always_comb begin
    out_payload = in_payload;
    out_valid = in_valid;
    in_ready = out_ready;
  end

  wire unused_inputs = src_clk ^ src_rst ^ dst_clk ^ dst_rst;
endmodule

module Source32 (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h0000_0020;
    tx_valid = !rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module Sink64 (
  input  logic        clk,
  input  logic        rst,
  input  logic [63:0] rx_payload,
  input  logic        rx_valid,
  output logic        rx_ready
);
  always_comb begin
    rx_ready = !rst;
  end

  wire unused_inputs = clk ^ rx_valid ^ rx_payload[0];
endmodule

module Widen32To64 (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] in_payload,
  input  logic        in_valid,
  output logic        in_ready,
  output logic [63:0] out_payload,
  output logic        out_valid,
  input  logic        out_ready
);
  always_comb begin
    out_payload = {32'h0000_0000, in_payload};
    out_valid = in_valid;
    in_ready = out_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

`default_nettype wire
