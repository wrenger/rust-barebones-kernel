/// Debug output for rust panics.
#[panic_handler]
pub fn panic_implementation(info: &::core::panic::PanicInfo) -> ! {
    let (file, line) = match info.location() {
        Some(loc) => (loc.file(), loc.line()),
        None => ("", 0),
    };
    if let Some(m) = info.message() {
        log!("PANIC file='{file}', line={line} :: {m}");
    } else if let Some(m) = info.payload().downcast_ref::<&str>() {
        log!("PANIC file='{file}', line={line} :: {m}");
    } else {
        log!("PANIC file='{file}', line={line} :: ?");
    }
    loop {}
}
