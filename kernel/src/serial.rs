use uart_16550::SerialPort;

use crate::Lazy;

static mut SERIAL1: Lazy<SerialPort> = Lazy::Uninitialized(|| unsafe {
    let mut serial_port = SerialPort::new(0x3F8);
    serial_port.init();
    serial_port
});

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use ::core::fmt::Write;

    unsafe {
        let logger = SERIAL1.unwrap();
        logger.write_fmt(args)
            .expect("Failed to write to serial port 1");
    }
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

