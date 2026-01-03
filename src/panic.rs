use crate::debug::dprintf;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    // NOTE: temporary
    dprintf("PANIC: ");
    dprintf(info.message().as_str().unwrap_or_default());

    // write!(DEBUG_OUTPUT, "panicked: {}", info.message());
    loop {}
}

pub fn panic_no_impl(msg: &str) -> ! {
    dprintf(msg);
    panic!("Not yet implemented!");
}
