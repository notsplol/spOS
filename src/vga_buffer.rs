use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;


#[allow(dead_code)]  //avoid unused variant error from compiler
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

//In VGA text mode bits 0-7 is the printed ASCII (or rather code page 437) character
//bits 8-11 specifies the foreground color, 12-14 specifies the background color
//bit 15 just speicifes whether the char should blink or not
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; 80]; 25],
}

pub struct Writer {
    column_position: usize,
    code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= 80 {
                    self.new_line();
                }

                let row = 24;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    character: byte,
                    code: self.code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..25 {
            for col in 0..80 {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(24);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            code: self.code,
        };
        for col in 0..80 {
            self.buffer.chars[row][col] = blank;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), 
            }
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        code: ColorCode::new(Color::LightCyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[25 - 2][i];
        assert_eq!(char::from(screen_char.character), c);
    }
}