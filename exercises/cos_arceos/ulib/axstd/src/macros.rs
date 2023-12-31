//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}


/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}

/// [`println_prefix!`]: crate::println_prefix
#[macro_export]
macro_rules! println_prefix {
    ($arg1:tt,$($arg2:tt)*) => {
        $crate::io::__print_impl(format_args!($arg1));
        $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg2)*)));
    }
}