use core::fmt::Write;

use bootloader_api::info::FrameBufferInfo;

use crate::{font, Lazy};

pub static mut FB_WRITER: Lazy<FrameBufferWriter> = Lazy::Empty;

pub(super) fn init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    unsafe {
        FB_WRITER.unsafe_init(FrameBufferWriter::new(buffer, info));
    }
}

pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
    cell: (u16, u16),
}

impl FrameBufferWriter {
    fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut fb = FrameBufferWriter {
            buffer,
            info,
            cell: (0, 0)
        };

        fb.clear();

        fb
    }

    fn clear(&mut self) {
        let count = 3 * self.info.width * self.info.height;

        // Faster than `self.buffer.fill(0)` and `for x in self.buffer`
        // Due to Iterators using clone over copy.
        unsafe {
            core::ptr::write_bytes(self.buffer.as_mut_ptr(), 0x00, count - 100);
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
        for char in s.chars() {
            if char == '\n' {
                self.newline();
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

                    self.draw_scaled_pixel(pixel as u8 * 0xFF, font::FONT_SCALE, (draw_x as u16, draw_y as u16));
                }
            }

            if self.cell.0 + 1 >= self.info.width as u16 / (font::FONT_WIDTH * font::FONT_SCALE) {
                self.newline();
            } else {
                self.cell.0 += 1;
            }
        }
    }

    fn draw_pixel(&mut self, color: u8, (x, y): (u16, u16)) {
        let pixel_index = (y as usize * self.info.width * 3) + (x as usize * 3);
        self.buffer[pixel_index    ] = color;
        self.buffer[pixel_index + 1] = color;
        self.buffer[pixel_index + 2] = color;
    }

    fn draw_scaled_pixel(&mut self, color: u8, scale: u16, (x, y): (u16, u16)) {
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
    unsafe {
        crate::serial::_print(args);

        if let Lazy::Initialized(ref mut framebuffer) = FB_WRITER {
            framebuffer.write_fmt(args)
                .expect("Failed to write to framebuffer");
        }
    }
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

