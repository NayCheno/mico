# T006: Direct CDC Without Adapter

## Goal

Reject a direct cross-domain stream connection from `Dma@Aclk` to `Aes@Bclk`.

## Expected behavior

The compiler should reject the task with domain/interface mismatch diagnostics. A valid solution must use an explicit CDC adapter.
