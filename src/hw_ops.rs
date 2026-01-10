#[allow(unused)]
pub trait HWWrite {
    fn hw_write_char(&mut self, char: u8) -> u8;
    fn hw_write_string(&mut self, msg: &[u8]) -> u8;
}

#[allow(unused)]
pub trait HWRead {}
