use crate::hw_ops::HWWrite;
use volatile::Volatile;
/// The height of the text buffer (normally 25 lines).
const BUFFER_HEIGHT: usize = 25;
/// The width of the text buffer (normally 80 columns).
const BUFFER_WIDTH: usize = 80;

// unused enum variants should not throw warnings
#[allow(dead_code)]
// semantic derives
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// each variant is a u8
#[repr(u8)]
pub enum VGAColour {
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

#[repr(transparent)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VGACode(u8);

#[allow(dead_code)]
impl VGACode {
    fn new(fg: VGAColour, bg: VGAColour) -> VGACode {
        VGACode((bg as u8) << 4 | (fg as u8))
    }
}

impl Default for VGACode {
    // White on Black
    fn default() -> VGACode {
        VGACode::new(VGAColour::White, VGAColour::Black)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VGACharacter {
    // 2 byte struct
    // {Character code: 8bit}{BG Colour: 3bit}{FGColour: 4bit}{Bright On/Off: 1bit}
    ascii_character: u8,
    code: VGACode,
}

pub struct VGAScreen {
    column_position: u8,
    buffer: &'static mut VGABuffer,
}

impl Default for VGAScreen {
    fn default() -> Self {
        VGAScreen {
            column_position: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
        }
    }
}

struct VGABuffer {
    chars: [[Volatile<VGACharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VGAScreen {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row by overwriting it with blank characters.
    fn clear_row(&mut self, row: usize) {
        let blank = VGACharacter {
            ascii_character: b' ',
            code: VGACode::default(),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl HWWrite for VGAScreen {
    /// Writes a single ASCII character to the buffer
    fn hw_write_char(&mut self, byte: u8) -> u8 {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if usize::from(self.column_position) >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // dprintf(&[byte]);
                let code = VGACode::default();
                self.buffer.chars[usize::from(row)][usize::from(col)].write(VGACharacter {
                    ascii_character: byte,
                    code,
                });
                self.column_position += 1;
            }
        }

        0
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
    fn hw_write_string(&mut self, s: &[u8]) -> u8 {
        for byte in s {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.hw_write_char(*byte),
                // not part of printable ASCII range
                _ => self.hw_write_char(0xfe),
            };
        }
        0
    }
}
