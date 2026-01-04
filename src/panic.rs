use crate::dprintln;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    // NOTE: temporary
    dprintln!("PANIC: {}", info.message().as_str().unwrap_or_default());
    loop {}
}

#[allow(unused)]
pub fn panic_no_impl(msg: &str) -> ! {
    dprintln!("{}", msg);
    panic!("Not yet implemented!");
}
