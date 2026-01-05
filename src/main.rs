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
mod gdt;

// imports
// use crate::debug;
use crate::vga::VGAScreen;
use crate::hw_ops::HWWrite;

// no_mangle: do not change the name of this function during compilation; extern "C" to allow use
// of the underlying C-based ABI 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    init_tables();

    let mut a = 0;
    for _ in 1..10000{
        a += 10;
    }

    dprintln!("Breakpoint exception: ");
    x86_64::instructions::interrupts::int3();
    
    let mut scr = VGAScreen::default();


    // let scr = spin::Mutex::new(VGAScreen::default());
    // scr.lock().hw_write_string(b"foobar");
    

    scr.hw_write_string(b"foobar");
    loop {
        x86_64::instructions::hlt();
    }
}

fn init_tables() {
    idt::init_idt();
    gdt::init();
    x86_64::instructions::interrupts::enable();
    unsafe { idt::PICS.lock().initialize() };
}
