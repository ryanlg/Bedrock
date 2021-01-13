# Project Bedrock
Bedrock is an x86/x86_64 chain loader written in Rust and Assembly.

## Goal
This project is a work-in-progress. The main goal is for me to learn about OSDev, and ultimately to boot a Linux kernel under x86_64 arch.

## Getting Started
### Prerequisite
- Rust (nightly) with `rust-src`
- QEMU (`qemu-system-x86`)
- NASM

### Setup
> macOS only.
- `brew install llvm qemu bochs nasm x86_64-elf-binutils`
