#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

build_dir="${MICO_EDA_SMOKE_BUILD_DIR:-build/eda-smoke}"
mkdir -p "${build_dir}"

if ! command -v sby >/dev/null 2>&1; then
  echo "ERROR: SymbiYosys driver 'sby' is required for eda-smoke; rebuild the Docker image with MICO_EDA_REBUILD=1." >&2
  exit 1
fi

leafs="rtl/examples/mico_example_leafs.sv"
examples=(
  stream_fifo
  cdc_fifo
  width_adapter
)

for example in "${examples[@]}"; do
  wrapper="${build_dir}/${example}_top.sv"
  sva="${build_dir}/${example}_sva.sv"
  vvp="${build_dir}/${example}_top.vvp"
  echo "== MICO EDA smoke: ${example} =="
  (
    cd rust_project
    cargo run -q -p mico_cli -- emit-sv "examples/${example}.mico"
  ) > "${wrapper}"
  (
    cd rust_project
    cargo run -q -p mico_cli -- emit-sva "examples/${example}.mico"
  ) > "${sva}"

  verilator --lint-only -Wall \
    -Wno-DECLFILENAME \
    -Wno-UNUSEDSIGNAL \
    --top-module Top \
    "${leafs}" \
    "${wrapper}"

  verilator --lint-only -Wall \
    -Wno-DECLFILENAME \
    -Wno-UNUSEDSIGNAL \
    --top-module mico_sva_Top \
    "${sva}"

  iverilog -g2012 -s Top -o "${vvp}" "${leafs}" "${wrapper}"

  yosys -q -p "read_verilog -sv ${leafs} ${wrapper}; hierarchy -check -top Top; proc; opt; stat"
done

formal_dir="${build_dir}/formal"
mkdir -p "${formal_dir}"
cat > "${formal_dir}/mico_sby_smoke.sv" <<'SV'
module mico_sby_smoke;
  always @* begin
    assert (1'b1);
  end
endmodule
SV
cat > "${formal_dir}/mico_sby_smoke.sby" <<'SBY'
[options]
mode prove
depth 1

[engines]
smtbmc z3

[script]
read -formal mico_sby_smoke.sv
prep -top mico_sby_smoke

[files]
mico_sby_smoke.sv
SBY
(
  cd "${formal_dir}"
  sby -f mico_sby_smoke.sby
)

echo "MICO EDA smoke passed"
