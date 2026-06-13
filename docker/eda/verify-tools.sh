#!/usr/bin/env bash
set -euo pipefail

echo "== Rust =="
rustc --version
cargo --version
rustup --version 2>/dev/null

echo "== RTL / EDA tools =="
yosys -V
verilator --version
iverilog -V 2>/dev/null | head -n 1
ghdl --version | head -n 1
tclsh <<'TCL'
puts "tclsh [info patchlevel]"
TCL
z3 --version

for tool in boolector yosys-abc nextpnr-ice40 nextpnr-ecp5 icepack openocd; do
    if command -v "${tool}" >/dev/null 2>&1; then
        if [[ "${tool}" == "yosys-abc" ]]; then
            version_line="installed at $(command -v "${tool}")"
        else
            set +e
            version_line="$(${tool} --version 2>&1 | head -n 1)"
            set -e
            if [[ -z "${version_line}" ]]; then
                version_line="installed at $(command -v "${tool}")"
            fi
        fi
        echo "optional ${tool}: ${version_line}"
    else
        echo "optional ${tool}: not installed"
    fi
done

if command -v gtkwave >/dev/null 2>&1; then
    echo "optional gtkwave: installed at $(command -v gtkwave)"
else
    echo "optional gtkwave: not installed"
fi

echo "== Python EDA packages =="
python3 --version
python3 -m pip list --disable-pip-version-check --format=freeze 2>/dev/null \
    | awk -F'==' '/^(cocotb|edalize|fusesoc|openai|pytest|PyYAML)==/ {print $1, $2}'
