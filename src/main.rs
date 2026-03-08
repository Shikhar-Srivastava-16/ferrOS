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
use bootloader::{entry_point, BootInfo};
use memory::active_level_4_table;
use x86_64::{
    structures::paging::{Page, Size4KiB},
    VirtAddr,
};
// no_mangle: do not change the name of this function during compilation; extern "C" to allow use
// of the underlying C-based ABI
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    init_tables();

    crate::vga_printf!("HELLO WORLD!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // let mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = memory::EmptyFrameAllocator;

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            dprintln!("L4 Entry {}: {:?}", i, entry);
        }
    }

    dprintln!("MEOW");

    // map an unused page
    // max 4096
    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(4095));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(0).write_volatile(0x_f021_f077_f065_f04e) };

    /* Potential Book Entry:
    *
    * We have discovered that we can write to the vga buffer using this setup. How much? And what
    * stops us?
    *
    * There are two potential answers - because there are two potential limiting factors:
    *   1. We run out of space, or...
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
