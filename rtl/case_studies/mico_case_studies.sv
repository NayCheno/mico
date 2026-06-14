`default_nettype none

module CaseDmaSource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'hcafe_0011;
    tx_valid = !rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module CaseSkidBuffer (
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

module CaseXorFilter (
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
    output_payload = input_payload ^ 32'h0000_00ff;
    output_valid = input_valid;
    input_ready = output_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseResultSink (
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

module CaseSensor32 (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h1234_5678;
    tx_valid = !rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module CaseWiden32To64 (
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

module CaseAccel64 (
  input  logic        clk,
  input  logic        rst,
  input  logic [63:0] rx_payload,
  input  logic        rx_valid,
  output logic        rx_ready,
  output logic [63:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = rx_payload + 64'h0000_0000_0000_1000;
    tx_valid = rx_valid;
    rx_ready = tx_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseHostSink64 (
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

module CaseApbCommandSource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] req_payload,
  output logic        req_valid,
  input  logic        req_ready
);
  always_comb begin
    req_payload = 32'h0000_0003;
    req_valid = !rst;
  end

  wire unused_inputs = clk ^ req_ready;
endmodule

module CaseRegisterFile (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] write_payload,
  input  logic        write_valid,
  output logic        write_ready,
  output logic [31:0] status_payload,
  output logic        status_valid,
  input  logic        status_ready
);
  always_comb begin
    status_payload = write_payload | 32'h8000_0000;
    status_valid = write_valid;
    write_ready = status_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseStatusSink (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] status_payload,
  input  logic        status_valid,
  output logic        status_ready
);
  always_comb begin
    status_ready = !rst;
  end

  wire unused_inputs = clk ^ status_valid ^ status_payload[0];
endmodule

module CaseProtocolSource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] req_payload,
  output logic        req_valid,
  input  logic        req_ready
);
  always_comb begin
    req_payload = 32'h0000_00a5;
    req_valid = !rst;
  end

  wire unused_inputs = clk ^ req_ready;
endmodule

module CaseProtocolBridge (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] req_payload,
  input  logic        req_valid,
  output logic        req_ready,
  output logic [31:0] rsp_payload,
  output logic        rsp_valid,
  input  logic        rsp_ready
);
  always_comb begin
    rsp_payload = {req_payload[23:0], 8'h5a};
    rsp_valid = req_valid;
    req_ready = rsp_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseProtocolSink (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] rsp_payload,
  input  logic        rsp_valid,
  output logic        rsp_ready
);
  always_comb begin
    rsp_ready = !rst;
  end

  wire unused_inputs = clk ^ rsp_valid ^ rsp_payload[0];
endmodule

module CaseTelemetrySource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h1357_2468;
    tx_valid = !rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module CaseTelemetryFilter (
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
    output_payload = input_payload ^ 32'h00ff_00ff;
    output_valid = input_valid;
    input_ready = output_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseTelemetryAccumulator64 (
  input  logic        clk,
  input  logic        rst,
  input  logic [63:0] rx_payload,
  input  logic        rx_valid,
  output logic        rx_ready,
  output logic [63:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = rx_payload + 64'h0000_0000_1111_0000;
    tx_valid = rx_valid;
    rx_ready = tx_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseAxiLiteSource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] req_payload,
  output logic        req_valid,
  input  logic        req_ready
);
  always_comb begin
    req_payload = 32'h0000_0044;
    req_valid = !rst;
  end

  wire unused_inputs = clk ^ req_ready;
endmodule

module CaseAxiToApbBridge (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] axi_payload,
  input  logic        axi_valid,
  output logic        axi_ready,
  output logic [31:0] apb_payload,
  output logic        apb_valid,
  input  logic        apb_ready
);
  always_comb begin
    apb_payload = axi_payload | 32'h0000_1000;
    apb_valid = axi_valid;
    axi_ready = apb_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseApbPeripheralSink (
  input  logic        clk,
  input  logic        rst,
  input  logic [31:0] req_payload,
  input  logic        req_valid,
  output logic        req_ready
);
  always_comb begin
    req_ready = !rst;
  end

  wire unused_inputs = clk ^ req_valid ^ req_payload[0];
endmodule

module CasePixelSource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h0000_0033;
    tx_valid = !rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module CaseLineBuffer (
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
    output_payload = input_payload + 32'h0000_0001;
    output_valid = input_valid;
    input_ready = output_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseThresholdFilter (
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
    output_payload = input_payload | 32'h8000_0000;
    output_valid = input_valid;
    input_ready = output_ready;
  end

  wire unused_inputs = clk ^ rst;
endmodule

module CaseFrameSink (
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

module CaseEventSource (
  input  logic        clk,
  input  logic        rst,
  output logic [31:0] tx_payload,
  output logic        tx_valid,
  input  logic        tx_ready
);
  always_comb begin
    tx_payload = 32'h0000_0e11;
    tx_valid = rst;
  end

  wire unused_inputs = clk ^ tx_ready;
endmodule

module CaseEventStatusSink (
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
module CaseAsyncFifo32 (
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

`default_nettype wire
