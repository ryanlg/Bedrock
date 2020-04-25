#/usr/bin/env bash

apt-get update
apt-get -y install build-essential qemu nasm xorriso

# We need llvm
bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"

# Set timezone and sync time with NTP
rm /etc/localtime
ln -s /usr/share/zoneinfo/America/Chicago /etc/localtime
timedatectl set-ntp off
timedatectl set-ntp on
