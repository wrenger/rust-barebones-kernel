use core::fmt;

/// A primitive lock for the logging output
pub struct Writer;


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // If the lock is owned by this instance, then we can safely write to the output
		unsafe { crate::arch::debug::puts(s) };
        Ok(())
    }
}

/// A very primitive logging macro
///
/// Obtains a logger instance (locking the log channel) with the current module name passed
/// then passes the standard format! arguments to it
macro_rules! log{
	( $($arg:tt)* ) => ({
		use core::fmt::Write;
		let _ = writeln!(&mut crate::logging::Writer, "[{}]: {}", module_path!(), format_args!($($arg)*));
	})
}
