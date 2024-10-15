use alloc::string::String;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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
    None = u8::MAX,
}

pub struct ColorStr {
    pub name: &'static str,
    pub color: Color,
}

pub const STR_COLORS: &[ColorStr] = &[
    ColorStr {
        name: "black",
        color: Color::Black,
    },
    ColorStr {
        name: "blue",
        color: Color::Blue,
    },
    ColorStr {
        name: "green",
        color: Color::Green,
    },
    ColorStr {
        name: "cyan",
        color: Color::Cyan,
    },
    ColorStr {
        name: "red",
        color: Color::Red,
    },
    ColorStr {
        name: "magenta",
        color: Color::Magenta,
    },
    ColorStr {
        name: "brown",
        color: Color::Brown,
    },
    ColorStr {
        name: "lightgray",
        color: Color::LightGray,
    },
    ColorStr {
        name: "lightgreen",
        color: Color::LightGreen,
    },
    ColorStr {
        name: "lightcyan",
        color: Color::LightCyan,
    },
    ColorStr {
        name: "lightred",
        color: Color::LightRed,
    },
    ColorStr {
        name: "pink",
        color: Color::Pink,
    },
    ColorStr {
        name: "yellow",
        color: Color::Yellow,
    },
    ColorStr {
        name: "white",
        color: Color::White,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
#[derive(Clone)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

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

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
    }

    pub fn change_color(&mut self, fgc: Color, bgc: Color) {
        self.color_code = ColorCode::new(fgc, bgc);
    }

    pub fn print_colored(&mut self, string: String, fgc: Color, bgc: Color) {
        let prv_colorcode = self.color_code;

        self.color_code = ColorCode::new(fgc, bgc);
        self.write_string(&string);
        self.color_code = prv_colorcode;
    }

    pub fn decrement_column_position(&mut self) {
        self.column_position -= 1;
    }
    pub fn increment_column_position(&mut self) {
        self.column_position += 1;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
