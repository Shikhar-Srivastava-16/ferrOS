use crate::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::dprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
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

        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.cp_protection_exception.set_handler_fn(cp_protection_exception_handler);
        idt.hv_injection_exception.set_handler_fn(hv_injection_exception_handler);
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);

        idt[Interrupts::Timer as u8].set_handler_fn(timer_interrupt_handler);
        idt[Interrupts::Keyboard as u8].set_handler_fn(keyboard_int_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn divide_error_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  divide_error_handler\n",);
}
extern "x86-interrupt" fn debug_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  debug_handler\n",);
}
extern "x86-interrupt" fn non_maskable_interrupt_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  non_maskable_interrupt_handler\n",);
}
extern "x86-interrupt" fn overflow_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  overflow_handler\n",);
}
extern "x86-interrupt" fn bound_range_exceeded_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  bound_range_exceeded_handler\n",);
}
extern "x86-interrupt" fn invalid_opcode_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  invalid_opcode\n");
}
extern "x86-interrupt" fn device_not_available_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  device_not_available_handler\n",);
}
extern "x86-interrupt" fn invalid_tss_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  invalid_tss_handler\n",);
}
extern "x86-interrupt" fn segment_not_present_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  segment_not_present_handler\n",);
}
extern "x86-interrupt" fn stack_segment_fault_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  stack_segment_fault_handler\n",);
}
extern "x86-interrupt" fn general_protection_fault_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  general_protection_fault_handler\n",);
}
extern "x86-interrupt" fn page_fault_handler(_st_fr: InterruptStackFrame, code: PageFaultErrorCode) {
    crate::dprintf!("TODO INT:  page_fault_handler\n",);
}
extern "x86-interrupt" fn x87_floating_point_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  x87_floating_point_handler\n",);
}
extern "x86-interrupt" fn alignment_check_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  alignment_check_handler\n",);
}
extern "x86-interrupt" fn machine_check_handler(_st_fr: InterruptStackFrame) -> ! {
    crate::dprintf!("TODO INT:  machine_check_handler\n",);
    loop{}
}
extern "x86-interrupt" fn simd_floating_point_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  simd_floating_point_handler\n",);
}
extern "x86-interrupt" fn virtualization_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  virtualization_handler\n",);
}
extern "x86-interrupt" fn cp_protection_exception_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  cp_protection_exception_handler\n",);
}
extern "x86-interrupt" fn hv_injection_exception_handler(_st_fr: InterruptStackFrame) {
    crate::dprintf!("TODO INT:  hv_injection_exception_handler\n",);
}
extern "x86-interrupt" fn vmm_communication_exception_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  vmm_communication_exception_handler\n",);
}
extern "x86-interrupt" fn security_exception_handler(_st_fr: InterruptStackFrame, code: u64) {
    crate::dprintf!("TODO INT:  security_exception_handler\n",);
}

// ------------- For PIC -----------------

use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::PageFaultErrorCode;

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