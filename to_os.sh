cargo build
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/debug/lynn --strip-all -O binary target/riscv64gc-unknown-none-elf/debug/lynn.bin
