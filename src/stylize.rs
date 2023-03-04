/// Creates a `Style` struct and formats text.
///
/// Similar to the `stylize` function.
///
/// # Examples
///
/// ```
/// # use stilo::stylize;
/// // Red
/// println!("{}", stylize!("Hello": Red));
///
/// // Red, italic, and bold
/// println!("{}", stylize!("Hello": Red italic bold));
///
/// // Default color, italic and bold
/// println!("{}", stylize!("Hello": - i b));
///
/// // Format string
/// let world = "World!";
/// println!("{}", stylize!("Hello {}": Green i b, world));
/// ```
#[macro_export]
macro_rules! stylize {
    // No style, just text
    (
        $text: literal
        $(, $arg: expr )* $(,)?
    ) => {
        format!($text, $( $arg, )*)
    };

    // Only color, no decoration
    (
        $text: literal
        : $color: ident
        $(, $arg: expr )* $(,)?
    ) => {
        $crate::Style::new()
            .color($crate::Color::$color)
            .format(
                &format!($text, $( $arg, )*)
            )
    };

    // Color and decoration
    (
        $text: literal
        : $color: ident
        $( $decor: ident )*
        $(, $arg: expr )* $(,)?
    ) => {
        $crate::Style::new()
            .color($crate::Color::$color)
            $(
                .$decor()
            )*
            .format(
                &format!($text, $( $arg, )*)
            )
    };

    // Only decoration, not color
    (
        $text: literal
        : -
        $( $decor: ident )*
        $(, $arg: expr )* $(,)?
    ) => {
        $crate::Style::new()
            $(
                .$decor()
            )*
            .format(
                &format!($text, $( $arg, )*)
            )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn stylize_works() {
        let world = "World!";

        assert_eq!(stylize!("Hello"), "Hello");

        assert_eq!(stylize!("Hello {}", world), "Hello World!");
        assert_eq!(stylize!("Hello {world}"), "Hello World!");
        assert_eq!(stylize!("Hello {0} {1}", world, 123,), "Hello World! 123");

        assert_eq!(stylize!("Hello": Red), "\x1b[31mHello\x1b[0m");
        assert_eq!(stylize!("Hello": Red,), "\x1b[31mHello\x1b[0m");
        assert_eq!(
            stylize!("Hello {}": Red, world,),
            "\x1b[31mHello World!\x1b[0m"
        );

        assert_eq!(stylize!("Hello": Red bold), "\x1b[31;1mHello\x1b[0m");
        assert_eq!(stylize!("Hello": Green b), "\x1b[32;1mHello\x1b[0m");

        assert_eq!(
            stylize!("Hello": Blue italic b,),
            "\x1b[34;1;3mHello\x1b[0m"
        );
        assert_eq!(
            stylize!("Hello {}": Blue italic b, world,),
            "\x1b[34;1;3mHello World!\x1b[0m"
        );

        assert_eq!(stylize!("Hello": - u d bold), "\x1b[1;2;4mHello\x1b[0m");
        assert_eq!(
            stylize!("Hello {}": - u d bold, world),
            "\x1b[1;2;4mHello World!\x1b[0m"
        );
    }
}
