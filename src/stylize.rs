/// Don't use this
#[doc(hidden)]
#[macro_export]
macro_rules! fallback_metavar {(
    { $($tt:tt)* }
    $($ignored:tt)?
) => (
    $($tt)*
)}

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
/// println!("{}", stylize!("Hello": Red + italic + bold));
///
/// // Default color, italic and bold
/// println!("{}", stylize!("Hello": +i+b));
///
/// // Format string
/// let world = "World!";
/// println!("{}", stylize!("Hello {}": Green + i+b, world));
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

    // Conditional, with no style
    (
        $text: literal :
        $( if $condition: expr )?
        $(, $arg: expr )* $(,)?
    ) => {
        compile_error!("Cannot use conditional style, if no styles are included");
    };

    // With existing style
    (
        $text: literal :
        $style: block
        $( if $condition: expr )?
        $(, $arg: expr )* $(,)?
    ) => {
        if $crate::fallback_metavar!( $({ $condition })? {true} ) {
            $style.format(
                &format!($text, $( $arg, )*)
            )
        } else {
            format!($text, $( $arg, )*)
        }
    };

    // Color and decoration
    (
        $text: literal :
        $( $color: ident )?
        $( + $decor: ident )*
        $( if $condition: expr )?
        $(, $arg: expr )* $(,)?
    ) => {
        if $crate::fallback_metavar!( $({ $condition })? {true}) {
            $crate::Style::new()
                $( .color($crate::Color::$color) )?
                $(
                    .$decor()
                )*
                .format(
                    &format!($text, $( $arg, )*)
                )
        } else {
            format!($text, $( $arg, )*)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::style;

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

        assert_eq!(stylize!("Hello": Red + bold), "\x1b[31;1mHello\x1b[0m");
        assert_eq!(stylize!("Hello": Green + b), "\x1b[32;1mHello\x1b[0m");

        assert_eq!(
            stylize!("Hello": Blue + italic + b,),
            "\x1b[34;1;3mHello\x1b[0m"
        );
        assert_eq!(
            stylize!("Hello {}": Blue + italic + b, world,),
            "\x1b[34;1;3mHello World!\x1b[0m"
        );

        assert_eq!(stylize!("Hello": +u+d+bold), "\x1b[1;2;4mHello\x1b[0m");
        assert_eq!(
            stylize!("Hello {}": +u+d+bold, world),
            "\x1b[1;2;4mHello World!\x1b[0m"
        );

        assert_eq!(stylize!("Hello": Red if true), "\x1b[31mHello\x1b[0m");
        assert_eq!(stylize!("Hello": Red if false), "Hello");

        assert_eq!(
            stylize!("Hello": Red+italic+ bold if true),
            "\x1b[31;1;3mHello\x1b[0m"
        );
        assert_eq!(stylize!("Hello": Red+italic+bold if false), "Hello");

        assert_eq!(
            stylize!("Hello {}": Red+italic+bold if true, world),
            "\x1b[31;1;3mHello World!\x1b[0m"
        );
        assert_eq!(
            stylize!("Hello {}": Red+italic+bold if false, world),
            "Hello World!"
        );

        assert_eq!(
            stylize!("Hello {}": +i if true, world),
            "\x1b[3mHello World!\x1b[0m"
        );
        assert_eq!(stylize!("Hello {}": +i if false, world), "Hello World!");

        let style = style!(Red + italic);

        assert_eq!(stylize!("Hello": {style}), "\x1b[31;3mHello\x1b[0m");
        assert_eq!(stylize!("Hello": {style} if true), "\x1b[31;3mHello\x1b[0m");
        assert_eq!(stylize!("Hello": {style} if false), "Hello");

        assert_eq!(stylize!("Hello": {style!(Blue)}), "\x1b[34mHello\x1b[0m");
        assert_eq!(
            stylize!("Hello": {style!(Blue)} if true),
            "\x1b[34mHello\x1b[0m"
        );
        assert_eq!(stylize!("Hello": {style!(Blue)} if false), "Hello");

        assert_eq!(
            stylize!("Hello": {
                    if true {
                        style!(Red)
                    } else {
                        style!()
                    }
            }
            ),
            "\x1b[31mHello\x1b[0m"
        );
        assert_eq!(
            stylize!("Hello": {if false { style!(Red) } else { style!() }}),
            "Hello"
        );

        assert_eq!(
            stylize!("Hello": {if false { style!(Red) } else { style!() }} if true),
            "Hello"
        );
    }
}
