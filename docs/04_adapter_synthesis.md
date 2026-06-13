# Adapter Synthesis Plan

## Adapter classes

### Structural adapters

- field rename;
- bundle flatten/unflatten;
- constant tie-off;
- optional signal defaulting.

These can be auto-inserted if semantics are lossless and reported.

### Width adapters

- truncate: unsafe unless explicitly allowed;
- zero/sign extend: safe only with declared numeric semantics;
- pack/unpack: may change transfer granularity.

### Protocol adapters

- valid-only to valid-ready;
- ready-valid skid buffer;
- pulse to level;
- interrupt synchronizer.

### Domain adapters

- two-flop synchronizer for single-bit controls;
- async FIFO for streams;
- reset synchronizer;
- request/ack handshake synchronizer.

## Contract-guided formulation

```text
Find adapter A such that:
  Source.guarantee ; A.guarantee |= Sink.assumption
subject to:
  domain constraints,
  width constraints,
  latency bound,
  no_drop/no_duplicate/order constraints,
  synthesizability.
```

## v0 strategy

Do not synthesize arbitrary adapters. Provide an adapter library plus legality checker.

## v1 strategy

Search within a small adapter grammar:

```text
Adapter ::= Rename | TieOff | Extend | Slice | SkidBuffer | Pipeline(N) | AsyncFifo(D) | Bridge(P)
```

## v2 strategy

Generate proof obligations and use formal tools to validate adapter correctness.
