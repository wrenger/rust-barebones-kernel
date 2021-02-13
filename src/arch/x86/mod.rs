// x86 port IO
#[path = "../x86_common/io.rs"]
pub mod x86_io;

// Debug output channel (uses serial)
#[path = "../x86_common/debug.rs"]
pub mod debug;

const PRESENT: u32 = 0b1;
const RW: u32 = 0b10;

#[no_mangle]
pub fn x86_prep_page_table(buf: &mut [u32; 1024])
{
	// Mapping 4000KiB from 3Gib to 0GiB

	// PT contains physical mem 0x0-0x400000
	for i in 0u32 .. 1024
	{
		// Setup 4kib pages
		buf[i as usize] = i * 0x1000 | RW | PRESENT;
	}
}
