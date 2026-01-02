use crate::dprintf;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // NOTE: temporary
    loop {}
}

pub fn panic_no_impl(msg: &[u8]) -> ! {
    dprintf(msg);
    panic!("Not yet implemented!");
}
