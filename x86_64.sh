cargo build --target=src/arch/x86_64/x86_64-unknown-kernel.json --release
# sadly we can only boot 32bit elfs
objcopy target/x86_64-unknown-kernel/release/rust-barebones-kernel -F elf32-i386 target/x86_64-unknown-kernel/release/rust-barebones-kernel.elf32
qemu-system-x86_64 -serial stdio -gdb tcp::1234 -no-shutdown -no-reboot -kernel target/x86_64-unknown-kernel/release/rust-barebones-kernel.elf32
