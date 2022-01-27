cargo build --target src/arch/aarch64/aarch64-unknown-none.json --release
qemu-system-aarch64 -M virt -cpu cortex-a57 -m 1G -serial stdio -gdb tcp::1234 -no-shutdown -no-reboot -kernel target/aarch64-unknown-none/release/rust-barebones-kernel
