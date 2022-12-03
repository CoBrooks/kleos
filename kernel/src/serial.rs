use spin::{Lazy, Mutex};
use uart_16550::SerialPort;

static SERIAL1: Lazy<Mutex<SerialPort>> = Lazy::new(|| unsafe {
    let mut serial_port = SerialPort::new(0x3F8);
    serial_port.init();
    serial_port.into()
});

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use ::core::fmt::Write;
    use ::x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1.lock()
            .write_fmt(args)
            .expect("Failed to write to serial port 1");
    })
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    ()            => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

