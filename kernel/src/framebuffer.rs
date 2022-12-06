use core::fmt::Write;

use bootloader_api::info::FrameBufferInfo;
use spin::{Mutex, Once};

use crate::{font, color::{ColorName, Color, THEME}, serial_println};

pub static FB_WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

pub(super) fn init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    FB_WRITER.call_once(|| FrameBufferWriter::new(buffer, info).into());
}

pub struct TextStyle {
    fg: Color,
    bg: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            fg: THEME[ColorName::Foreground as usize],
            bg: THEME[ColorName::Background as usize],
        }
    }
}

pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
    cell: (u16, u16),
    text_style: TextStyle
}

impl FrameBufferWriter {
    fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut fb = FrameBufferWriter {
            buffer,
            info,
            cell: (0, 0),
            text_style: TextStyle::default()
        };

        fb.clear();

        fb
    }

    fn clear(&mut self) {
        let bg = THEME[ColorName::Background as usize].to_framebuffer_pixel();

        // This is faster than using for i in 0..num_subpixels,
        // since for loops use the `Iterator` trait under the hood,
        // which uses Clone, rather than Copy.
        //
        // This could likely be optimized even further with some
        // cursed pointer stuff, but it is fast enough as is.
        let mut i = 0;
        let num_subpixels = self.buffer.len();
        while i < num_subpixels {
            self.buffer[i] = bg[i % 3];

            i += 1;
        }
    }

    fn newline(&mut self) {
        if self.cell.1 + 1 >= self.info.height as u16 / (font::FONT_HEIGHT * font::FONT_SCALE) {
            self.clear();
            self.cell = (0, 0);
        } else {
            self.cell.1 += 1;
            self.cell.0 = 0;
        }
    }

    fn write_string(&mut self, s: &str) {
        let mut chars = s.chars();

        while let Some(char) = chars.next() {
            if char == '\n' {
                self.newline();
                continue;
            }

            if char == '\x1B' {
                let _square_bracket = chars.next();
                let color_code = [chars.next(), chars.next()];

                if let [Some(mode), Some(color)] = color_code {
                    if mode != '3' && mode != '4' { continue }

                    let color_index = color as u8 - b'0';
                    let color = THEME[color_index as usize];

                    match mode {
                        '3' => self.text_style.fg = color,
                        '4' => self.text_style.bg = color,
                        _ => { }
                    }
                }

                let _m = chars.next();

                continue;
            }

            let glyph = &font::FONT[char as usize];

            // (0, 0) is at the bottom left of the glyph,
            // while `glyph.raster` starts at the top left,
            // so the glyph has to be offset accordingly
            let cell_offset_y = font::FONT_SCALE * (font::FONT_HEIGHT.max(glyph.height) - glyph.height);

            for y in 0..glyph.height {
                for x in 0..glyph.width {
                    let index = y * glyph.width + x;
                    let pixel = glyph.raster[index as usize];

                    let x = x * font::FONT_SCALE;
                    let y = y * font::FONT_SCALE;
                    
                    let (sx, sy) = self.cell;

                    let cell_x = font::FONT_SCALE * sx * font::FONT_WIDTH;
                    let cell_y = font::FONT_SCALE * sy * font::FONT_HEIGHT;

                    let draw_x = (cell_x + x) as isize + (font::FONT_SCALE as isize * glyph.offset_x);
                    let draw_y = (cell_y + y + cell_offset_y) as isize - (font::FONT_SCALE as isize * glyph.offset_y);

                    let color = if pixel { self.text_style.fg } else { self.text_style.bg };

                    self.draw_scaled_pixel(color, font::FONT_SCALE, (draw_x as u16, draw_y as u16));
                }
            }

            if self.cell.0 + 1 >= self.info.width as u16 / (font::FONT_WIDTH * font::FONT_SCALE) {
                self.newline();
            } else {
                self.cell.0 += 1;
            }
        }
    }

    fn draw_pixel(&mut self, color: Color, (x, y): (u16, u16)) {
        let pixel_index = (y as usize * self.info.width * 3) + (x as usize * 3);
        self.buffer[pixel_index    ] = color.b;
        self.buffer[pixel_index + 1] = color.g;
        self.buffer[pixel_index + 2] = color.r;
    }

    fn draw_scaled_pixel(&mut self, color: Color, scale: u16, (x, y): (u16, u16)) {
        for sy in 0..scale {
            for sx in 0..scale {
                self.draw_pixel(color, (x + sx, y + sy));
            }
        }
    }
}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use ::x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        crate::serial::_print(args);

        FB_WRITER.get()
            .expect("Framebuffer has not been initialized")
            .lock()
            .write_fmt(args)
            .expect("Failed to write to framebuffer");
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ()            => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

