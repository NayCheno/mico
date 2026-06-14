# T005: Invalid Width Without Adapter

## Goal

Reject a direct `StreamU32 -> StreamU64` connection that omits the explicit width adapter.

## Expected behavior

The compiler should reject the task with `InterfaceMismatch`; no RTL lint, simulation, or formal stage should run for this negative case.
