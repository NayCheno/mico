# T066 APB Direct Protocol Mismatch Case

Negative held-out case paired with `T063_axi_apb_wrapper_case`. It attempts to
connect an AXI-lite-like command stream directly to an APB-like request port
without the declared bridge module.
