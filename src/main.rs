#![feature(panic_info_message)] //< Panic handling
#![feature(llvm_asm)] //< As a kernel, we need inline assembly
#![feature(global_asm)]
#![no_std]
#![no_main]

#[macro_use]
mod logging;

// Achitecture-specific modules
#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;
#[cfg(target_arch = "x86")]
#[path = "arch/x86/mod.rs"]
pub mod arch;

// Startup code
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("arch/x86_64/start.s"));
#[cfg(target_arch = "x86")]
global_asm!(include_str!("arch/x86/start.s"));

/// Exception handling (panic)
pub mod unwind;

// Kernel entrypoint (called by arch/<foo>/start.s)
#[no_mangle]
pub fn kmain() {
	log!("Hello world! 1={}", 1);

	loop {}
}
