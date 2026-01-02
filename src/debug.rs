#[allow(dead_code)]
// write a message to the debug port, only for qemu
pub fn dprintf(msg: &[u8]) -> () {
    for &byte in msg.iter() {
        outb(0x00E9, byte);
    }
    ()
}

// write a single character to a port
pub fn outb(port: u16, data: u8) {
    unsafe {
        x86_64::instructions::port::Port::new(port).write(data);
    }
}
