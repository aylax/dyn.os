# lynn

```bash
rustup target list | grep riscv64
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/lynn; echo $?
```
