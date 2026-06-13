# T003: Width Adapter Required

## Goal

Connect a `StreamU32` source to a `StreamU64` sink using an explicit packer adapter.

## Expected behavior

Direct connect must fail with `InterfaceMismatch`. A valid solution must use a declared width adapter.
