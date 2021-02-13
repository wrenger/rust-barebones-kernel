# Rust Bare-Bones Kernel

This is designed to be a rust equivalent of the OSDev.org Bare\_Bones article, presenting the bare minimum you need to get started.


## Requirements

* `rustup` and `cargo`: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
* An up to date `nightly-i686-unknown-linux-gnu` toolchain
* The `rust-src` component, because we have a custom target:
    * `rustup component add rust-src`
* QEMU for i386 or x86_64

## Features

* x86 and x86\_64 (amd64) "ports"
* Initial paging for both (with higher-half)
* Links with libcore
* Serial output using the classic PC serial port, formatted using `core::fmt`

## Building

### 32-bit

Build with cargo:
```bash
# configure the toolchain to be used (only necessary on the first time)
rustup override set nightly-i686-unknown-linux-gnu
# build for the custom i686 target
cargo build --target src/arch/i686/i686-unknown-kernel.json
```

Run with qemu:
```bash
qemu-system-i386 -serial stdio -kernel target/i686-unknown-kernel/debug/rust-barebones-kernel
```

You should see a

```text
[rust_barebones_kernel] Hello world! 1=1
```

print to the console.

### 64-bit

Build with cargo:
```bash
# configure the toolchain to be used (only necessary on the first time)
rustup override set nightly-x86_64-unknown-linux-gnu
# build for the custom x86_64 target
cargo build --target src/arch/x86_64/x86_64-unknown-kernel.json
# Convert to 32bit elf so that the bios can load it
objcopy target/x86_64-unknown-kernel/debug/rust-barebones-kernel -F elf32-i386 target/x86_64-unknown-kernel/debug/rust-barebones-kernel.elf32
```

Run with qemu:
```bash
qemu-system-x86_64 -serial stdio -kernel target/x86_64-unknown-kernel/debug/rust-barebones-kernel.elf32
```

You should see a

```text
[rust_barebones_kernel] Hello world! 1=1
```

print to the console.

## Debugging

This rust kernel can be debugged like any other c/c++ kernel using qemu's debugging api and gdb or lldb to connect to it.

```
# run kernel
qemu-system-i386 -kernel target/i686-unknown-kernel/release/rust-barebones-kernel -serial stdio -gdb tcp::1234 -no-shutdown -no-reboot -S
# open gdb (in another terminal)
gdb target/i686-unknown-kernel/release/rust-barebones-kernel
(gdb) target remote :1234
```

> For 64-bit substitute `i386` / `i686` with `x86_64`.
