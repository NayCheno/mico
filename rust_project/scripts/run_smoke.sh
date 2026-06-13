#!/usr/bin/env bash
set -euo pipefail
cargo run -p mico_cli -- check examples/stream_fifo.mico
cargo run -p mico_cli -- dump-ir examples/stream_fifo.mico
cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico > /tmp/mico_top.sv
cargo run -p mico_cli -- check examples/invalid_width.mico && {
  echo "expected invalid_width.mico to fail" >&2
  exit 1
} || true
