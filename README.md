# lynn

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/lynn; echo $?
```
