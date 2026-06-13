# T001: Stream FIFO Direct Composition

## Goal

Connect `Producer -> Fifo -> Consumer` using `StreamU32` in the same clock domain.

## Expected MICO

```mico
compose Top @Sys {
  inst p: Producer;
  inst f: Fifo;
  inst c: Consumer;
  connect p.tx -> f.input;
  connect f.output -> c.rx;
}
```

## Negative variants

- Reverse `f.input -> p.tx`.
- Connect `p.tx -> c.rx` while leaving FIFO unused.
- Use a non-existing port name.
