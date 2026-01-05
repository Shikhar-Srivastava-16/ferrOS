use crate::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::dprintln!("EXCEPTION: BREAKPOIN\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn dft_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(dft_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[Interrupts::Timer as u8].set_handler_fn(timer_interrupt_handler);
        idt[Interrupts::Keyboard as u8].set_handler_fn(keyboard_int_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// ------------- For PIC -----------------

use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[repr(u8)]
#[allow(unused)]
enum Interrupts {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

extern "x86-interrupt" fn keyboard_int_handler(_stack_frame: InterruptStackFrame) {
    crate::dprintf!("k");

    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::dprintf!("{}", scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(Interrupts::Keyboard as u8);
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // crate::dprintf!(".");

    unsafe {
        PICS.lock().notify_end_of_interrupt(Interrupts::Timer as u8);
    }
}
