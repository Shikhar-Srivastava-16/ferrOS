#[allow(dead_code)]
// write a message to the debug port, only for qemu
pub fn dprintf(msg: &str) -> () {
    for &byte in msg.as_bytes() {
        debug_writeb(0x00E9, byte);
    }
    ()
}

pub fn dprintln(msg: &str) -> () {
    dprintf(msg);
    debug_writeb(0x00E9, b'\n');
    ()
}

// write a single character to a port
pub fn debug_writeb(port: u16, data: u8) {
    unsafe {
        x86_64::instructions::port::Port::new(port).write(data);
    }
}

#[macro_export]
macro_rules! format {
    () => {};
    ($($arg:tt)*) => {
        (format_args!($($arg)*)).as_str()
    };
}
