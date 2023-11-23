# OxideCore
A Simple Unix-like Kernel Written in Rust

## Tool Chain
On MacOS:
```bash
brew install rustup
rustup default nightly
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Usage
```bash
cd os
make run
```
