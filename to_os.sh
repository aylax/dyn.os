cargo build
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/debug/dyn-os --strip-all -O binary target/riscv64gc-unknown-none-elf/debug/dyn-os.bin
