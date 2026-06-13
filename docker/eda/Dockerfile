# syntax=docker/dockerfile:1.7

FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG UBUNTU_MIRROR=http://mirrors.tuna.tsinghua.edu.cn/ubuntu

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN sed -i "s|http://archive.ubuntu.com/ubuntu|${UBUNTU_MIRROR}|g; s|http://security.ubuntu.com/ubuntu|${UBUNTU_MIRROR}|g" /etc/apt/sources.list.d/ubuntu.sources \
    && rm -f /etc/apt/apt.conf.d/docker-clean \
    && echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' >/etc/apt/apt.conf.d/keep-cache

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update \
    && apt-get install -y --no-install-recommends \
        autoconf \
        automake \
        bash \
        bison \
        build-essential \
        ca-certificates \
        cmake \
        curl \
        flex \
        gawk \
        ghdl \
        git \
        graphviz \
        iverilog \
        less \
        libffi-dev \
        libreadline-dev \
        libssl-dev \
        libtool \
        locales \
        make \
        ninja-build \
        perl \
        pkg-config \
        python3 \
        python3-click \
        python3-jinja2 \
        python3-pip \
        python3-setuptools \
        python3-venv \
        python3-yaml \
        tcl \
        tcl-dev \
        verilator \
        yosys \
        z3 \
        zlib1g-dev \
    && for pkg in \
        berkeley-abc \
        boolector \
        dfu-util \
        fpga-icestorm \
        ghdl-yosys-plugin \
        gtkwave \
        nextpnr-ecp5 \
        nextpnr-ice40 \
        openocd \
        prjtrellis \
    ; do \
        if apt-cache show "${pkg}" >/dev/null 2>&1; then \
            apt-get install -y --no-install-recommends "${pkg}"; \
        else \
            echo "Skipping optional package not available in Ubuntu 24.04 apt: ${pkg}"; \
        fi; \
    done

ARG RUST_TOOLCHAIN=1.96.0
ARG RUSTUP_DIST_SERVER=https://rsproxy.cn
ARG RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup
ARG RUSTUP_INIT_URL=https://rsproxy.cn/rustup-init.sh

ENV RUSTUP_HOME=/opt/rust/rustup
ENV CARGO_HOME=/opt/rust/cargo
ENV PATH=/opt/rust/cargo/bin:/usr/local/bin:/usr/local/sbin:/usr/sbin:/usr/bin:/sbin:/bin
ENV RUSTUP_DIST_SERVER=${RUSTUP_DIST_SERVER}
ENV RUSTUP_UPDATE_ROOT=${RUSTUP_UPDATE_ROOT}
ENV PIP_INDEX_URL=https://pypi.tuna.tsinghua.edu.cn/simple
ENV PIP_DISABLE_PIP_VERSION_CHECK=1

RUN mkdir -p "${CARGO_HOME}" \
    && printf '%s\n' \
        '[source.crates-io]' \
        'replace-with = "rsproxy-sparse"' \
        '' \
        '[source.rsproxy]' \
        'registry = "https://rsproxy.cn/crates.io-index"' \
        '' \
        '[source.rsproxy-sparse]' \
        'registry = "sparse+https://rsproxy.cn/index/"' \
        '' \
        '[registries.rsproxy]' \
        'index = "https://rsproxy.cn/crates.io-index"' \
        '' \
        '[registries.rsproxy-sparse]' \
        'index = "sparse+https://rsproxy.cn/index/"' \
        '' \
        '[net]' \
        'git-fetch-with-cli = true' \
        > "${CARGO_HOME}/config.toml"

RUN curl --proto '=https' --tlsv1.2 -sSf "${RUSTUP_INIT_URL}" \
        | sh -s -- -y --profile default --default-toolchain "${RUST_TOOLCHAIN}" \
    && rustup component add rustfmt clippy \
    && chmod -R a+rwX /opt/rust

RUN python3 -m pip install --break-system-packages --no-cache-dir \
        cocotb \
        edalize \
        fusesoc \
        jinja2 \
        openai \
        pyaml \
        pytest \
        pytest-xdist \
        pyyaml

COPY docker/eda/verify-tools.sh /usr/local/bin/mico-verify-tools
RUN chmod +x /usr/local/bin/mico-verify-tools

WORKDIR /workspace
CMD ["/bin/bash"]
