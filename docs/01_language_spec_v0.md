# MICO Language Spec v0

## Design philosophy

MICO is not a full HDL. It is a module-composition language for existing RTL/IP. Its source should describe:

- what interfaces exist;
- which modules expose which interfaces;
- which instances are composed;
- which interface endpoints are connected;
- which adapters are allowed;
- which contracts must hold.

## Lexical conventions

- Identifiers: `[A-Za-z_][A-Za-z0-9_]*`
- Statements end with `;`.
- Line comments start with `//`.
- Blocks use `{ ... }`.
- Built-in scalar types: `bool`, `u1`, `u8`, `u16`, `u32`, `u64`, `u128`.

## Top-level declarations

### Clock domain

```mico
clockdom Sys(clk, rst);
clockdom Aclk(aclk, arst_n);
```

### Interface

```mico
interface StreamU32 @Sys {
  producer payload:u32, valid:bool;
  consumer ready:bool;
  contract stable_payload: valid -> stable(payload) until ready;
  contract fire: valid & ready;
}
```

Each interface has:

- a name;
- a domain annotation;
- producer fields;
- consumer fields;
- optional contracts.

For a ready/valid stream:

- producer emits `payload` and `valid`;
- consumer emits `ready`;
- transfer event is usually `valid & ready`.

### External module

```mico
extern module Producer @Sys {
  out tx: StreamU32;
}

extern module Consumer @Sys {
  in rx: StreamU32;
}
```

`extern module` declares an existing RTL module whose implementation is not in MICO.

### Adapter

```mico
adapter AsyncFifo32 from StreamU32@Aclk to StreamU32@Bclk {
  kind cdc_fifo;
  depth 4;
  contract preserves_order;
}
```

Adapters are explicit unless declared as safe auto-adapters.

### Compose

```mico
compose Top @Sys {
  inst p: Producer;
  inst c: Consumer;
  connect p.tx -> c.rx;
}
```

`connect` links two interface endpoints. Legal direct connection requires:

1. both instances exist;
2. both ports exist;
3. source port direction is `out`;
4. sink port direction is `in`;
5. both ports use the same interface type;
6. both endpoints are in compatible clock/reset domains;
7. all contracts are compatible.

Cross-domain connections require `adapt`:

```mico
adapt dma.tx -> AsyncFifo32 -> aes.rx;
```

## Static errors

| Error | Meaning |
|---|---|
| `UnknownInstance` | Referenced instance does not exist. |
| `UnknownPort` | Referenced port does not exist on the instance module. |
| `DirectionMismatch` | Source is not `out` or sink is not `in`. |
| `InterfaceMismatch` | Direct connection uses incompatible interfaces. |
| `ClockDomainMismatch` | Direct connection crosses clock/reset domains. |
| `AdapterRequired` | A safe direct connection is impossible. |
| `AmbiguousConnect` | A shorthand connection maps to multiple candidates. |
| `ContractViolation` | Sink assumption is not satisfied by source+adapter guarantee. |

## Lowering contract

A checked MICO program must lower deterministically to:

- SystemVerilog wrapper/top module;
- optional SVA assertion file;
- optional JSON IR;
- future CIRCT HW/ESI/Verif/LTL IR.
