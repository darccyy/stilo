/// Print many stylized strings individually, concatenated, with newline.
///
/// Similar: `stylize_many!`
///
/// For each argument, creates a `Style` struct and format text.
///
/// Separate arguments with a semicolon.
/// New lines will **not** be added between arguments.
///
/// Each argument is used the same as the `stylize!` macro.
///
/// # Examples
///
/// ```
/// # use stilo::println_styles;
/// let world = "World!";
///
/// // `println_styles!` would also work in this example
/// println_styles!(
///     // Red
///     "Hello\n": Red;
///     // Red, italic, and bold
///     "Hello\n": Red italic bold;
///     // Default color, italic and bold
///     "Hello\n": - i b;
///     // Format string
///     "Hello {}": Green i b, world;
/// );
/// ```
#[macro_export]
macro_rules! println_styles {
    ( $( $arg: tt )* ) => {
        println!("{}", $crate::stylize_many!($( $arg )*))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn println_styles_works() {
        let world = "World!";

        println_styles!(
            "Hello";
            "\nHello": Red;
            "\nHello": Blue italic;
            "\nHello": - i b;
            "\n";
            "\nHello {}": Red, world;
            "\nHello {}": Blue italic, world;
            "\nHello {}": - italic, world;
            "\nHello {} {}": - underline dim, world, 123;
            "\nHello {world}": Green;
            "\nHello {world:?}": Green b;
            "\nHello {} {}": Red, world, 123;
            "\nHello {} {}": Red d, world, 123;
            "\nHello {world} {}", 123;
        );
    }
}
