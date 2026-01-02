// disallow use of standard libraries for OS development
#![no_std]
#![no_main]

// modules
mod debug;
mod std;
mod panic;
mod vga;
mod hw_ops;

// imports
use crate::debug::dprintf;
use crate::vga::VGAScreen;
use crate::hw_ops::HWWrite;

// no_mangle: do not change the name of this function during compilation; extern "C" to allow use
// of the underlyin C-based ABI 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    // NOTE: Temp block start
    dprintf(b"meow!");

    let mut scr = VGAScreen::default();
    scr.hw_write_string(b"foobar");
    // NOTE: Temp block end
    loop {}
}



