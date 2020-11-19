mod console_printing {
    /// Prints to the console video output.
    ///
    /// Equivalent to the [`println!`] macro except that a newline is not printed at
    /// the end of the message.
    ///
    ///

    #[macro_export]
    macro_rules! c_str {
        ($s:expr) => {{
            concat!($s, "\0").as_ptr()
        }};
    }

    //Taken from https://github.com/rust-wii/ogc-rs/blob/master/ogc-rs/src/utils.rs
    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => {
            let s = ::alloc::fmt::format(format_args!($($arg)*));
            $crate::printf(s.as_ptr());
        }
    }

    //Taken from https://github.com/rust-wii/ogc-rs/blob/master/ogc-rs/src/utils.rs
    /// Prints to the standard output, with a newline.
    #[macro_export]
    macro_rules! println {
        () => (print!("\n"));
        ($fmt:expr) => (print!(concat!($fmt, "\n\0")));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n\0"), $($arg)*));
}
}
