#!/usr/bin/env bash
set -euo pipefail

missing_required=0

require_tool() {
    local tool="$1"
    if ! command -v "${tool}" >/dev/null 2>&1; then
        echo "required ${tool}: not installed"
        missing_required=1
        return 1
    fi
    return 0
}

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
if require_tool yosys-smtbmc; then
    yosys-smtbmc -h 2>/dev/null | head -n 1
fi
if require_tool sby; then
    set +e
    sby_version="$(sby --version 2>&1 | head -n 1)"
    sby_status=$?
    set -e
    if [[ ${sby_status} -ne 0 || -z "${sby_version}" ]]; then
        sby_version="installed at $(command -v sby)"
    fi
    echo "sby ${sby_version}"
fi

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

if [[ ${missing_required} -ne 0 ]]; then
    echo "ERROR: required Docker EDA tools are missing; rebuild the image with MICO_EDA_REBUILD=1." >&2
    exit 1
fi
