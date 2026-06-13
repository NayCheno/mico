#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image="${MICO_EDA_IMAGE:-mico-eda:ubuntu24.04}"
cargo_registry_volume="${MICO_CARGO_REGISTRY_VOLUME:-mico-cargo-registry}"
cargo_git_volume="${MICO_CARGO_GIT_VOLUME:-mico-cargo-git}"

if [[ "${MICO_EDA_REBUILD:-0}" == "1" ]] || ! docker image inspect "${image}" >/dev/null 2>&1; then
    docker build -f "${repo_root}/docker/eda/Dockerfile" -t "${image}" "${repo_root}"
fi

tty_args=(-i)
if [[ -t 0 && -t 1 ]]; then
    tty_args=(-it)
fi

if [[ $# -eq 0 ]]; then
    set -- bash
fi

docker run --rm "${tty_args[@]}" \
    -v "${repo_root}:/workspace" \
    -v "${cargo_registry_volume}:/opt/rust/cargo/registry" \
    -v "${cargo_git_volume}:/opt/rust/cargo/git" \
    -w /workspace \
    "${image}" \
    "$@"
