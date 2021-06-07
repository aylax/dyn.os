# lynn

```bash

# build basic Rust Env
rustup target list | grep riscv64
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview

# build lynn os
cargo build

# gen bin file
rust-objcopy \
    --binary-architecture=riscv64 \
    target/riscv64gc-unknown-none-elf/debug/lynn \
    --strip-all -O binary \
    target/riscv64gc-unknown-none-elf/debug/lynn.bin

# exec
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/debug/lynn.bin,addr=0x80200000
```
