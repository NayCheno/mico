`default_nettype none

module Top(
  input logic clk,
  input logic rst
);
  logic [31:0] axi_req__bridge_axi_payload;
  logic axi_req__bridge_axi_ready;
  logic axi_req__bridge_axi_valid;
  logic [31:0] bridge_apb__periph_req_payload;
  logic bridge_apb__periph_req_ready;
  logic bridge_apb__periph_req_valid;

  CaseAxiLiteSource axi (
    .clk(clk),
    .rst(rst),
    .req_payload(axi_req__bridge_axi_payload),
    .req_valid(axi_req__bridge_axi_valid),
    .req_ready(axi_req__bridge_axi_ready)
  );

  CaseAxiToApbBridge bridge (
    .clk(clk),
    .rst(rst),
    .axi_payload(axi_req__bridge_axi_payload),
    .axi_valid(axi_req__bridge_axi_valid),
    .axi_ready(axi_req__bridge_axi_ready),
    .apb_payload(bridge_apb__periph_req_payload),
    .apb_valid(bridge_apb__periph_req_valid),
    .apb_ready(bridge_apb__periph_req_ready)
  );

  CaseApbPeripheralSink periph (
    .clk(clk),
    .rst(rst),
    .req_payload(bridge_apb__periph_req_payload),
    .req_valid(bridge_apb__periph_req_valid),
    .req_ready(bridge_apb__periph_req_ready)
  );
endmodule

`default_nettype wire
