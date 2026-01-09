// disallow use of standard libraries for OS development
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// modules
mod debug;
mod gdt;
mod hw_ops;
mod idt;
mod panic;
mod std;
mod vga;

// imports
use crate::hw_ops::HWWrite;
use crate::vga::VGAScreen;

// no_mangle: do not change the name of this function during compilation; extern "C" to allow use
// of the underlying C-based ABI
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {

bootloader::entry_point!(kernel_main);

fn kernel_main(_info: &'static bootloader::BootInfo) -> ! {
    init_tables();

    let scr = spin::Mutex::new(VGAScreen::default());
    scr.lock().hw_write_string(b"Hello World!");

    loop {
        x86_64::instructions::hlt();
    }
}

fn init_tables() {
    dprintln!("..initialising IDT..");
    idt::init_idt();
    dprintln!("..initialising GDT..");
    gdt::init();
    dprintln!("..enabling generic interrupts..");
    x86_64::instructions::interrupts::enable();
    dprintln!("!!UNSAFE ACTION!!..initializing PIC..");
    unsafe { idt::PICS.lock().initialize() };
}
