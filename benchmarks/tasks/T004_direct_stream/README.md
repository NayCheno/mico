# T004: Direct Stream

## Goal

Connect `Producer -> Consumer` directly with a same-domain `StreamU32` ready/valid interface.

## Expected behavior

The compiler should accept the direct connection, and the generated wrapper should lint and elaborate against `rtl/examples/mico_example_leafs.sv`.
