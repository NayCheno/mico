# T002: CDC FIFO Required

## Goal

Connect `Dma@Aclk` to `Aes@Bclk` through an explicit async FIFO adapter.

## Expected behavior

Direct connect must fail with `ClockDomainMismatch`. A valid solution must use `adapt dma.tx -> AsyncFifo32 -> aes.rx`.
