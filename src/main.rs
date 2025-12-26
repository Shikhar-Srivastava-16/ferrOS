// disallow use of standard libraries for OS development
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // NOTE: temporary
    loop {}
}
static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    // NOTE: Temp block start
    let vga_buffer = 0xb8000 as *mut u8;

    dprintf(b"meow!");

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x9;
        }
    }
    // NOTE: Temp block end
    loop {}
}

// unused enum variants should not throw warnings
#[allow(dead_code)]
// write a message to the debug port, only for qemu
pub fn dprintf(msg: &[u8]) {
    for &byte in msg.iter() {
        unsafe {
            outb(0x00E9, byte);
        }
    }
}

// write a single character to a port
pub unsafe fn outb(port: u16, data: u8) {
    unsafe {
        x86_64::instructions::port::Port::new(port).write(data);
    }
}

// semantic derives
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// each variant is a u8
#[repr(u8)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

struct VGACharacter {

    // 2 byte struct
    // {Character code: 8bit}{BG Colour: 3bit}{FGColour: 4bit}{Blink On/Off: 1bit}
}
