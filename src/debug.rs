use core::fmt::Error as fmtErr;

#[allow(dead_code)]
// write a message to the debug port, only for qemu
#[repr(u16)]
enum Debuggers {
    Qemu = 0x00E9,
}

#[derive(Copy, Clone)]
struct DebugPort {
    port: u16,
}

impl core::fmt::Write for DebugPort {
    fn write_str(&mut self, s: &str) -> Result<(), fmtErr> {
        self.debug_writeln(s);
        Ok(()) // figure out return types
    }
}

impl Default for DebugPort {
    fn default() -> Self {
        DebugPort { port: 0x00E9 }
    }
}

impl DebugPort {
    fn debug_writeln(self: DebugPort, msg: &str) -> () {
        for &byte in msg.as_bytes() {
            self.debug_writeb(byte);
        }
        ()
    }

    // write a single character to a port
    fn debug_writeb(self, data: u8) {
        unsafe {
            x86_64::instructions::port::Port::new(self.port).write(data);
        }
    }
}

pub fn _dprintf(args: core::fmt::Arguments) {
    use core::fmt::Write;
    DebugPort::default()
        .write_fmt(args)
        .unwrap_or_else(|e| panic!("`dprintf` action failed: {:?}", e));
}

#[macro_export]
macro_rules! dprintf {
    ($($arg:tt)*) => ($crate::debug::_dprintf(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dprintln {
    () => ($crate::dprintf!("\n"));
    ($($arg:tt)*) => ($crate::dprintf!("{}\n", format_args!($($arg)*)));
}
