// disallow use of standard libraries for OS development
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// modules
mod debug;
mod std;
mod panic;
mod vga;
mod hw_ops;
mod idt;

// imports
use crate::debug::dprintln;
use crate::vga::VGAScreen;
use crate::hw_ops::HWWrite;

// no_mangle: do not change the name of this function during compilation; extern "C" to allow use
// of the underlying C-based ABI 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    // NOTE: Temp block start
    // panic!("MEOW");
    idt::init_idt();
    dprintln("meow1!");
    x86_64::instructions::interrupts::int3();
    dprintln("meow2!");

    let mut scr = VGAScreen::default();
    scr.hw_write_string(b"foobar");
    // NOTE: Temp block end
    loop {}
}
