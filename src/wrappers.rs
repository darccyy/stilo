/// Format stylized strings individually, concatenated.
///
/// Wrapper for `stylize_many!`
#[macro_export]
macro_rules! print_styles {
    ( $( $arg: tt )* ) => {
        print!("{}", $crate::stylize_many!($( $arg )*))
    };
}

/// Format stylized strings individually, concatenated.
///
/// Wrapper for `stylize_many!`
#[macro_export]
macro_rules! println_styles {
    ( $( $arg: tt )* ) => {
        println!("{}", $crate::stylize_many!($( $arg )*))
    };
}

/// Format stylized strings individually, concatenated.
///
/// Wrapper for `stylize_many!`
#[macro_export]
macro_rules! write_styles {
    ( $f: expr, $( $arg: tt )* ) => {
        write!($f, "{}", $crate::stylize_many!($( $arg )*))
    };
}

/// Format stylized strings individually, concatenated.
///
/// Wrapper for `stylize_many!`
#[macro_export]
macro_rules! writeln_styles {
    ( $f: expr, $( $arg: tt )* ) => {
        writeln!($f, "{}", $crate::stylize_many!($( $arg )*))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn println_styles_compiles() {
        let world = "World!";

        println_styles!(
            "Hello";
            "\nHello": Red;
            "\nHello": Blue+italic;
            "\nHello": + i+b;
            "\n";
            "\nHello {}": Red, world;
            "\nHello {}": Blue+italic, world;
            "\nHello {}": +italic, world;
            "\nHello {} {}": +underline+dim, world, 123;
            "\nHello {world}": Green;
            "\nHello {world:?}": Green+b;
            "\nHello {} {}": Red, world, 123;
            "\nHello {} {}": Red+d, world, 123;
            "\nHello {world} {}", 123;
        );
    }
}
