FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# [Optional] Uncomment this section to install additional packages.
RUN export DEBIAN_FRONTEND=noninteractive && apt-get -y upgrade && apt-get -y update

# Cargo tools for nightly must be installed with vscode user
USER vscode
RUN rustup toolchain install nightly && \
    rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu && \
    rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu