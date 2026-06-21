// disallow use of standard libraries for OS development
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// modules
mod gdt;
mod hw_ops;
mod idt;
mod std;
mod vga;

// imports
use bootloader::{BootInfo, entry_point};
// use memory::translate_addr;
// use memory::translate_addr_inner;
use macros::dprintf;
use macros::dprintln;
use macros::panic;
use memory::BootInfoFrameAllocator;
use memory::translate_addr;
use pc_keyboard::KeyCode::W;
use x86_64::{
    VirtAddr,
    structures::paging::{Page, Size4KiB, Translate},
};

// no_mangle: do not change the name of this function during compilation; extern "C" to allow use
// of the underlying C-based ABI
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {

// static X_TRAIN: [f64; 160] = [
//     1.16, 8.97, 6.84, 4.56, 3.18, 3.49, 0.46, 2.91, 3.01, 7.30, 6.33, 2.26, 1.56, 8.18, 1.87, 8.15,
//     9.62, 0.25, 5.12, 5.23, 5.91, 4.28, 1.71, 8.32, 8.08, 5.39, 0.34, 2.42, 1.20, 5.19, 7.75, 8.61,
//     6.34, 4.95, 8.93, 8.08, 2.00, 9.72, 0.31, 1.61, 7.32, 0.74, 3.12, 5.55, 8.97, 3.25, 6.32, 3.05,
//     0.93, 2.71, 2.92, 2.52, 6.36, 9.49, 9.70, 3.39, 7.28, 0.58, 5.14, 8.07, 1.56, 0.65, 4.97, 0.37,
//     0.77, 2.85, 0.21, 8.29, 9.30, 6.10, 3.21, 3.75, 8.87, 4.89, 7.72, 1.10, 2.81, 2.59, 5.36, 5.92,
//     1.22, 2.29, 2.42, 6.72, 7.85, 3.66, 2.58, 5.03, 6.23, 4.40, 0.41, 5.20, 7.71, 9.86, 9.09, 3.64,
//     5.99, 2.49, 8.95, 3.38, 3.41, 9.37, 6.60, 1.85, 8.63, 9.66, 8.66, 2.90, 7.13, 0.64, 6.91, 1.13,
//     4.72, 6.01, 2.12, 3.25, 0.17, 2.22, 5.25, 0.06, 1.74, 9.43, 1.87, 8.77, 3.57, 5.98, 7.56, 9.70,
//     7.26, 0.45, 5.47, 8.87, 1.39, 1.96, 3.68, 9.00, 5.11, 0.98, 7.62, 6.33, 9.51, 9.39, 2.79, 4.17,
//     2.40, 5.09, 1.08, 8.04, 6.38, 7.07, 8.96, 7.80, 6.12, 5.30, 1.99, 4.10, 1.82, 7.61, 1.38, 3.14,
// ];

// static Y_TRAIN: [f64; 160] = [
//     11.70, 37.76, 29.61, 23.00, 15.19, 15.00, 6.05, 14.75, 15.45, 30.55, 34.07, 11.28, 13.88,
//     35.84, 13.92, 38.81, 38.92, 4.11, 21.91, 30.59, 29.47, 23.18, 14.42, 39.03, 35.08, 21.57, 8.72,
//     16.20, 12.70, 26.00, 37.29, 35.97, 33.09, 20.59, 42.56, 31.54, 13.19, 39.53, 14.47, 9.57,
//     33.50, 11.63, 18.08, 24.74, 36.88, 18.94, 31.57, 15.12, 10.99, 18.45, 18.06, 18.36, 32.37,
//     42.35, 38.48, 14.52, 35.78, 5.46, 33.16, 38.75, 18.06, 8.61, 25.47, 9.21, 9.52, 19.67, 11.20,
//     35.06, 38.78, 30.77, 19.20, 18.07, 38.98, 24.48, 36.09, 12.24, 14.51, 13.41, 25.82, 29.61,
//     11.91, 12.23, 13.70, 32.17, 38.84, 22.30, 13.55, 26.48, 26.86, 24.97, 8.72, 21.77, 32.56,
//     45.33, 39.98, 19.10, 25.81, 13.60, 35.51, 12.96, 19.16, 39.75, 29.14, 15.15, 37.35, 42.22,
//     39.29, 7.42, 34.56, 11.13, 33.06, 8.94, 23.18, 25.11, 15.67, 21.79, 13.86, 11.06, 23.11, 5.78,
//     15.48, 39.55, 13.14, 37.27, 21.76, 29.47, 35.77, 44.18, 30.25, 10.30, 27.21, 35.67, 12.91,
//     25.42, 22.35, 37.10, 26.86, 5.87, 33.05, 29.85, 40.97, 35.75, 15.07, 26.37, 17.22, 23.35,
//     13.53, 33.81, 31.87, 31.93, 38.39, 38.72, 28.18, 23.07, 9.12, 22.69, 15.83, 30.03, 9.12, 13.44,
// ];

// static X_TEST: [f64; 40] = [
//     4.94, 1.83, 6.08, 2.38, 0.07, 8.71, 9.87, 6.78, 6.45, 6.63, 1.41, 9.25, 8.35, 3.58, 8.17, 3.87,
//     0.88, 1.45, 3.31, 0.75, 2.28, 3.04, 0.51, 5.61, 5.43, 3.89, 3.11, 8.02, 4.27, 1.20, 7.08, 4.32,
//     9.22, 7.29, 9.08, 9.08, 3.23, 7.03, 0.90, 7.71,
// ];

// static Y_TEST: [f64; 40] = [
//     22.32, 19.11, 25.05, 14.66, 7.58, 33.21, 35.94, 28.26, 33.06, 30.65, 10.48, 42.29, 35.30,
//     17.36, 36.84, 22.42, 11.64, 12.01, 25.01, 16.55, 15.57, 16.91, 6.31, 25.64, 25.28, 23.47,
//     11.81, 35.32, 20.15, 17.59, 34.14, 19.45, 40.81, 29.28, 41.03, 42.57, 20.08, 29.74, 12.21,
//     31.85,
// ];

static X_TRAIN: [i32; 160] = [
    1, 8, 6, 4, 3, 3, 0, 2, 3, 7, 6, 2, 1, 8, 1, 8, 9, 0, 5, 5, 5, 4, 1, 8, 8, 5, 0, 2, 1, 5, 7, 8,
    6, 4, 8, 8, 2, 9, 0, 1, 7, 0, 3, 5, 8, 3, 6, 3, 0, 2, 2, 2, 6, 9, 9, 3, 7, 0, 5, 8, 1, 0, 4, 0,
    0, 2, 0, 8, 9, 6, 3, 3, 8, 4, 7, 1, 2, 2, 5, 5, 1, 2, 2, 6, 7, 3, 2, 5, 6, 4, 0, 5, 7, 9, 9, 3,
    5, 2, 8, 3, 3, 9, 6, 1, 8, 9, 8, 2, 7, 0, 6, 1, 4, 6, 2, 3, 0, 2, 5, 0, 1, 9, 1, 8, 3, 5, 7, 9,
    7, 0, 5, 8, 1, 1, 3, 9, 5, 0, 7, 6, 9, 9, 2, 4, 2, 5, 1, 8, 6, 7, 8, 7, 6, 5, 1, 4, 1, 7, 1, 3,
];

static Y_TRAIN: [i32; 160] = [
    11, 37, 29, 23, 15, 15, 6, 14, 15, 30, 34, 11, 13, 35, 13, 38, 38, 4, 21, 30, 29, 23, 14, 39,
    35, 21, 8, 16, 12, 26, 37, 35, 33, 20, 42, 31, 13, 39, 14, 9, 33, 11, 18, 24, 36, 18, 31, 15,
    10, 18, 18, 18, 32, 42, 38, 14, 35, 5, 33, 38, 18, 8, 25, 9, 9, 19, 11, 35, 38, 30, 19, 18, 38,
    24, 36, 12, 14, 13, 25, 29, 11, 12, 13, 32, 38, 22, 13, 26, 26, 24, 8, 21, 32, 45, 39, 19, 25,
    13, 35, 12, 19, 39, 29, 15, 37, 42, 39, 7, 34, 11, 33, 8, 23, 25, 15, 21, 13, 11, 23, 5, 15,
    39, 13, 37, 21, 29, 35, 44, 30, 10, 27, 35, 12, 25, 22, 37, 26, 5, 33, 29, 40, 35, 15, 26, 17,
    23, 13, 33, 31, 31, 38, 38, 28, 23, 9, 22, 15, 30, 9, 13,
];

static X_TEST: [i32; 40] = [
    4, 1, 6, 2, 0, 8, 9, 6, 6, 6, 1, 9, 8, 3, 8, 3, 0, 1, 3, 0, 2, 3, 0, 5, 5, 3, 3, 8, 4, 1, 7, 4,
    9, 7, 9, 9, 3, 7, 0, 7,
];

static Y_TEST: [i32; 40] = [
    22, 19, 25, 14, 7, 33, 35, 28, 33, 30, 10, 42, 35, 17, 36, 22, 11, 12, 25, 16, 15, 16, 6, 25,
    25, 23, 11, 35, 20, 17, 34, 19, 40, 29, 41, 42, 20, 29, 12, 31,
];

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    init_tables();

    crate::vga_printf!("HELLO WORLD!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    dprintln!("MEOW");
    dprintln!("memory map: {:#?}", &boot_info.memory_map);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    unsafe { page_ptr.offset(450).write_volatile(0x_f021_f077_f065_f04e) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = unsafe { translate_addr(virt, phys_mem_offset) };
    //     dprintln!("{:?} -> {:?}", virt, phys);
    // }

    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    /* Potential Book Entry:
    *
    * We have discovered that we can write to the vga buffer using this setup. How much? And what
    * stops us?
    *
    * There are two potential answers - because there are two potential limiting factors:
    *   1. We run out of space on the screen, or...
    *   2. We run out of cells in the buffer
    * And the secret third option: both happen at the same time.
    * Of course, there is also the super-secret fourth option -- neither.
    *
    * Let us try and figure this out using a little experiment, and then decide which it is from
    * the information we glean.
    *
    * Two loops, each looping over indices from 0 to 2000. Why 2000? No idea, it's a random number
    * that popped into my head for no reason in particular. Much as it may look like the size of
    * the VGA buffer (25 x 80 cells in the buffer, which is a 2D array), it is absolutely not that
    * number at all.
    *
    ```rust
            for i in 0..2000 {
                dprintln!("{}", i);
                // write 'New!'
                unsafe { page_ptr.offset(i).write_volatile(0x_f021_f077_f065_f04e) };
            }

            for i in 0..10 {
                dprintln!("{}", i);
                // write something else
                unsafe { page_ptr.offset(i).write_volatile(0x_f022_f078_f066_f04f) };
            }
    ```
    * The first loop stays constant, the second one is used to find out how many get overwritten
    *
    * */

    dprintln!("!! Beginning Linear Regression !!");

    dprintln!(
        "X: {:?}\nY: {:?}\nX_T: {:?}\nY_T: {:?}",
        X_TRAIN,
        Y_TRAIN,
        X_TEST,
        Y_TEST
    ); //

    loop {
        x86_64::instructions::hlt();
    }
}

fn init_tables() {
    // NOTE: Easy mistake to make here: the order matters. The Interrupts depend on the IDT AND the
    // PIC. I was enabling them before initialising the PIC and got stuck
    dprintln!("..initialising IDT..");
    idt::init_idt();
    dprintln!("..initialising GDT..");
    gdt::init();
    dprintln!("!!UNSAFE ACTION!!..initializing PIC..");
    unsafe { idt::PICS.lock().initialize() };
    dprintln!("..enabling generic interrupts..");
    x86_64::instructions::interrupts::enable();
}

// tmp
// stubs for mathematical operations, to be replaced with the float type
// further optimised using binary operations

#[allow(non_camel_case_types)]
type k_float = i32;

pub fn add(op1: k_float, op2: k_float) -> k_float {
    op1 + op2
}

pub fn subtract(op1: k_float, op2: k_float) -> k_float {
    op1 - op2
}

pub fn mul(op1: k_float, op2: k_float) -> k_float {
    op1 * op2
}

pub fn div(op1: k_float, op2: k_float) -> k_float {
    op1 / op2
}

// helpers, also stubs
pub fn sq(op: k_float) -> k_float {
    op * op
}
