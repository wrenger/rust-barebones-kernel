#![allow(dead_code)] // < This sample doesn't use them, but you might :)

// NOTE: This code uses "{dx}N" as a register specifier. I _believe_ the N means (8-bit immediate)

use core::arch::asm;

/// Write a byte to the specified port
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb dx", in("dx") port, in("al") val);
}

/// Read a single byte from the specified port
pub unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    asm!("inb dx", in("dx") port, out("al") ret);
    return ret;
}

/// Write a word (16-bits) to the specified port
pub unsafe fn outw(port: u16, val: u16) {
    asm!("outw dx, ax", in("dx") port, in("ax") val);
}

/// Read a word (16-bits) from the specified port
pub unsafe fn inw(port: u16) -> u16 {
    let mut ret: u16;
    asm!("inb dx, ax", in("dx") port, out("ax") ret);
    return ret;
}

/// Write a long/double-word (32-bits) to the specified port
pub unsafe fn outl(port: u16, val: u32) {
    asm!("outw dx, eax", in("dx") port, in("eax") val);
}

/// Read a long/double-word (32-bits) from the specified port
pub unsafe fn inl(port: u16) -> u32 {
    let mut ret: u32;
    asm!("inb dx, eax", in("dx") port, out("eax") ret);
    return ret;
}
