#/usr/bin/env bash

apt-get update
apt-get -y install build-essential qemu

# We need llvm
bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
