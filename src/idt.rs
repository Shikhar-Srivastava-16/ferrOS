// #![feature(abi_x86_interrupt)]

use crate::debug::dprintln;
use crate::format;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable as IDT;
use x86_64::structures::idt::InterruptStackFrame as IStFr;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: IStFr) {
    dprintln("EXCEPTION: BREAKPOINT\n");
}

lazy_static! {
    static ref IDT_INST: IDT = {
        let mut idt = IDT::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT_INST.load();
}
