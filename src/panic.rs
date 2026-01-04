use crate::dprintln;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    // NOTE: temporary
    dprintln!("PANIC: {:#?}", info.message());
    loop {}
}

#[allow(unused)]
pub fn panic_no_impl(msg: &str) -> ! {
    dprintln!("{}", msg);
    panic!("Not yet implemented!");
}
