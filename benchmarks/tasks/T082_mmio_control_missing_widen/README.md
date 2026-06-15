# T082 MMIO Control Missing Widen

Reject a memory-mapped control/data path that connects a 32-bit control stream
directly into a 64-bit data path without the declared width adapter. This is
the paired unsafe variant for T081.
