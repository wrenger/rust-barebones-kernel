#![feature(panic_info_message)] //< Panic handling
#![no_std]
#![no_main]

use core::arch::{asm, global_asm};

#[macro_use]
mod logging;

// Achitecture-specific modules
#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;
#[cfg(target_arch = "x86")]
#[path = "arch/x86/mod.rs"]
pub mod arch;
#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
pub mod arch;

// Startup code
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("arch/x86_64/start.s"), options(att_syntax));
#[cfg(target_arch = "x86")]
global_asm!(include_str!("arch/x86/start.s"), options(att_syntax));
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("arch/aarch64/start.s"));

/// Exception handling (panic)
pub mod unwind;

// Kernel entrypoint (called by arch/<foo>/start.s)
#[no_mangle]
pub fn kmain() -> ! {
    unsafe { arch::debug::puts("hello world 0\n") };
    unsafe { asm!("nop") };

    log!("hello world 1");

    loop {}
}
