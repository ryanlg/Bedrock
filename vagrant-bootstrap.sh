#!/usr/bin/env bash

apt-get update

# Get rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup override add nightly
