#!/usr/bin/env bash

# Get rustup
# This assumes the current user is vagrant
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
echo 'export PATH=$HOME/.cargo/bin:$PATH' >> ~/.bashrc

# Setup project specific things
rustup override add nightly
cargo install cargo-xbuild      # Need this to compile
rustup component add rust-src
