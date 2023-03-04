/// Stylize many strings individually, and concatenate.
///
/// Similar: `println_styles!`
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
/// // `println_styles!` would also work in this example
/// println!("{}", stylize_many!(
///     // Red
///     "Hello\n": Red;
///     // Red, italic, and bold
///     "Hello\n": Red + italic + bold;
///     // Default color, italic and bold
///     "Hello\n": +i+b;
///     // Format string
///     "Hello {}": Green + i+b, world;
/// ));
/// ```
#[macro_export]
macro_rules! stylize_many {
    // Creating styles
    (
        $(
            $text: literal
            $(
                :
                $( $color: ident )?
                $( + $decor: ident )*
                $( if $condition: expr )?
            )?
            $(, $arg: expr )* $(,)?
        );* $(;)?
    ) => {{
        let mut text = String::new();
        $(
            text += &$crate::stylize!(
                $text
                $(
                    :
                    $( $color )?
                    $( + $decor )*
                    $( if $condition )?
                )?
                $(, $arg )*
            );
        )*
        text
    }};

    // Using existing styles
    (
        $(
            $text: literal
            $(
                :
                $( $style: block )?
                $( if $condition: expr )?
            )?
            $(, $arg: expr )* $(,)?
        );* $(;)?
    ) => {{
        let mut text = String::new();
        $(
            text += &$crate::stylize!(
                $text
                $(
                    :
                    $( $style )?
                    $( if $condition )?
                )?
                $(, $arg )*
            );
        )*
        text
    }};
}

#[cfg(test)]
mod tests {
    use crate::style;

    #[test]
    fn stylize_many_works() {
        let world = "World!";

        let actual = stylize_many!(
            "Hello";
            "\nHello": Red;
            "\nHello": Blue+italic;
            "\nHello": +i+b;
            "\n";
            "\nHello {}": Red, world;
            "\nHello {}": Blue+italic, world;
            "\nHello {}": + italic, world;
            "\nHello {} {}": + underline+dim, world, 123;
            "\nHello {world}": Green;
            "\nHello {world:?}": Green+b;
            "\nHello {} {}": Red, world, 123;
            "\nHello {} {}": Red+d, world, 123;
            "\nHello {world} {}", 123;
            "\nHello": Red if true;
            "\nHello": Red if false;
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
            "\x1b[31m\nHello\x1b[0m",
            "\nHello",
        );

        // Check each line
        let mut expected_lines = expected.split("\n");
        for actual_line in actual.split("\n") {
            let expected_line = expected_lines.next().unwrap();

            println!("{:?}\n{:?}\n", actual_line, expected_line);
            assert_eq!(actual_line, expected_line);
        }

        // Check all
        assert_eq!(actual, expected);
    }

    #[test]
    fn stylize_many_works_w_existing_styles() {
        let world = "World!";

        let style = style!(Yellow + italic);

        let actual = stylize_many!(
            "Hello";
            "\nHello": {style};
            "\nHello {}": {style}, world;
            "\nHello": {style} if true;
            "\nHello": {style} if false;
            "\nHello": {style!(Red)};
        );

        let expected = concat!(
            "Hello",
            "\x1b[33;3m\nHello\x1b[0m",
            "\x1b[33;3m\nHello World!\x1b[0m",
            "\x1b[33;3m\nHello\x1b[0m",
            "\nHello",
            "\x1b[31m\nHello\x1b[0m",
        );

        // Check each line
        let mut expected_lines = expected.split("\n");
        for actual_line in actual.split("\n") {
            let expected_line = expected_lines.next().unwrap();

            println!("{:?}\n{:?}\n", actual_line, expected_line);
            assert_eq!(actual_line, expected_line);
        }

        // Check all
        assert_eq!(actual, expected);
    }
}
