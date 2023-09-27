FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update
# RUN apt-get install -y curl build-essential pkg-config liblua5.4
RUN apt-get install -y lua5.4 liblua5.4

# WORKDIR /setup

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
#   chmod +x install.sh && \
#   ./install.sh -y

WORKDIR /lush

COPY .. .
