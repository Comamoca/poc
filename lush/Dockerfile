FROM nickblah/lua:5.4.6-luarocks-ubuntu

RUN apt-get update
RUN apt-get install -y curl build-essential pkg-config liblua5.4

WORKDIR /setup

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
  chmod +x install.sh && \
  ./install.sh -y

WORKDIR /lush

COPY . .

ENTRYPOINT ["bash"]
