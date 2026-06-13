#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

build_dir="${MICO_EDA_SMOKE_BUILD_DIR:-build/eda-smoke}"
mkdir -p "${build_dir}"

leafs="rtl/examples/mico_example_leafs.sv"
examples=(
  stream_fifo
  cdc_fifo
  width_adapter
)

for example in "${examples[@]}"; do
  wrapper="${build_dir}/${example}_top.sv"
  echo "== MICO EDA smoke: ${example} =="
  (
    cd rust_project
    cargo run -q -p mico_cli -- emit-sv "examples/${example}.mico"
  ) > "${wrapper}"

  verilator --lint-only -Wall \
    -Wno-DECLFILENAME \
    -Wno-UNUSEDSIGNAL \
    --top-module Top \
    "${leafs}" \
    "${wrapper}"

  yosys -q -p "read_verilog -sv ${leafs} ${wrapper}; hierarchy -check -top Top; proc; opt; stat"
done

echo "MICO EDA smoke passed"
