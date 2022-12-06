use core::str::FromStr;

use crate::serial_println;

// Gruvbox material medium dark
pub static THEME: &[Color] = &[
    Color::new(0x66, 0x5c, 0x54), // Black
    Color::new(0xEA, 0x69, 0x62), // Red
    Color::new(0xA9, 0xB6, 0x65), // Green
    Color::new(0xD8, 0xA6, 0x57), // Yellow
    Color::new(0x7D, 0xAE, 0xA3), // Blue
    Color::new(0xD3, 0x86, 0x9B), // Magenta
    Color::new(0x89, 0xB4, 0x82), // Cyan
    Color::new(0xDD, 0xC7, 0xA1), // White
    
    Color::new(0x29, 0x28, 0x28), // Background
    Color::new(0xEB, 0xDB, 0xD2), // Foreground
];

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ColorName {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    
    Background,
    Foreground,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_framebuffer_pixel(self) -> [u8; 3] {
        [self.b, self.g, self.r]
    }
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix('#') {
            if hex.len() == 6 {
                if !hex.chars().any(|c| !c.is_ascii_hexdigit()) {
                    let r = &hex[0..2];
                    let g = &hex[2..4];
                    let b = &hex[4..6];

                    let r = u8::from_str_radix(r, 16).unwrap();
                    let g = u8::from_str_radix(g, 16).unwrap();
                    let b = u8::from_str_radix(b, 16).unwrap();

                    Ok(Color { r, g, b })
                } else {
                    serial_println!("{hex}");
                    Err("Color hex code must be valid hexadecimal")
                }
            } else {
                Err("Color hex code must be 6 characters long")
            }
        } else {
            Err("Color hex code must start with '#'")
        }
    }
}

pub trait ColorExt<'a> {
    fn fg(self, fg: ColorName) -> ColoredStr<'a>;
    // TODO:
    // fn background(&self, bg: Color) -> ColoredStr;
}

impl<'a> ColorExt<'a> for &'a str {
    fn fg(self, fg: ColorName) -> ColoredStr<'a> {
        ColoredStr { fg, s: self }
    }
}

impl<'a> ColorExt<'a> for ColoredStr<'a> {
    fn fg(self, fg: ColorName) -> ColoredStr<'a> {
        ColoredStr { fg, ..self }
    }
}

pub struct ColoredStr<'a> {
    pub fg: ColorName,
    // TODO: pub bg: Color,
    pub s: &'a str,
}

impl core::fmt::Display for ColoredStr<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Calling `write` directly causes the escape sequence to be printed out in chunks,
        // which breaks the framebuffer's ANSI parsing... Thus, we allocate a String containing
        // the full ANSI sequence, then print it out.
        let full = alloc::format!("\x1B[3{}m{}\x1B[3{}m", self.fg as u8, self.s, ColorName::Foreground as u8);
        write!(f, "{full}")
    }
}

