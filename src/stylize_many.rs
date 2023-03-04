/// Stylize many strings individually, and concatenate.
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
/// # use stilo::stylize_many;
/// let world = "World!";
///
/// println!("{}", stylize_many!(
///     // Red
///     "Hello\n": Red;
///     // Red, italic, and bold
///     "Hello\n": Red italic bold;
///     // Default color, italic and bold
///     "Hello\n": - i b;
///     // Format string
///     "Hello {}": Green i b, world;
/// ));
/// ```
#[macro_export]
macro_rules! stylize_many {
    (
        $(
            $text: literal
            $(
                : $color: tt
                $( $decor: ident )*
            )?
            $(, $( $arg: tt ),* $(,)? )?
        );* $(;)?
    ) => {
        // ? Could this string concatenation be optimized ?
        String::new() + $(
            &$crate::stylize!(
                $text
                $(
                    : $color
                    $( $decor )*
                )?
                $(, $( $arg ),* )?
            ) +
        )* ""
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn stylize_many_works() {
        let world = "World!";

        let actual = stylize_many!(
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

        let expected = concat!(
            "Hello",
            "\x1b[31m\nHello\x1b[0m",
            "\x1b[34;3m\nHello\x1b[0m",
            "\x1b[1;3m\nHello\x1b[0m",
            "\n",
            "\x1b[31m\nHello World!\x1b[0m",
            "\x1b[34;3m\nHello World!\x1b[0m",
            "\x1b[3m\nHello World!\x1b[0m",
            "\x1b[2;4m\nHello World! 123\x1b[0m",
            "\x1b[32m\nHello World!\x1b[0m",
            "\x1b[32;1m\nHello \"World!\"\x1b[0m",
            "\x1b[31m\nHello World! 123\x1b[0m",
            "\x1b[31;2m\nHello World! 123\x1b[0m",
            "\nHello World! 123",
        );

        // Check each line
        let mut expected_lines = expected.split("\n");
        for actual_line in actual.split("\n") {
            let expected_line = expected_lines.next().unwrap();

            println!("{:?}\n{:?}", actual_line, expected_line);
            assert_eq!(actual_line, expected_line);
        }

        // Check all
        assert_eq!(actual, expected);
    }
}
