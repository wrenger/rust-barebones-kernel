use core::ptr;

const UART0: *mut u8 = 0x0900_0000 as *mut u8;

/// Write a string to the output channel
pub unsafe fn puts(s: &str) {
    for b in s.bytes() {
        putb(b);
    }
}

/// Write a single byte to the output channel
pub unsafe fn putb(b: u8) {
    ptr::write_volatile(UART0, b);
}
