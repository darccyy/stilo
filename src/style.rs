/// Creates a `Style` struct, without formatting text.
/// 
/// Similar to `Style` struct builder pattern.
///
/// # Examples
///
/// ```
/// # use stilo::style;
/// // Red
/// let style = style!(Red);
/// println!("{}", style.format("Hello"));
///
/// // Red, italic, and bold
/// let style = style!(Red + italic + bold);
/// println!("{}", style.format("Hello"));
///
/// // Default color, italic and bold
/// let style = style!(+i+b);
/// println!("{}", style.format("Hello"));
/// ```
#[macro_export]
macro_rules! style {
    // Blank style
    () => {
        $crate::Style::new()
    };

    // Color and decoration
    ( $( $color: ident )? $( + $decor: ident )* ) => {
        $crate::Style::new()
            $( .color($crate::Color::$color) )?
            $(
                .$decor()
            )*
    };
}

#[cfg(test)]
mod tests {
    use crate::{Color::*, Style};

    #[test]
    fn style_works() {
        assert_eq!(
            style!(Red),
            Style {
                color: Some(Red),
                ..Default::default()
            }
        );

        assert_eq!(
            style!(Red+bold),
            Style {
                color: Some(Red),
                bold: true,
                ..Default::default()
            }
        );

        assert_eq!(
            style!(Green+b),
            Style {
                color: Some(Green),
                bold: true,
                ..Default::default()
            }
        );

        assert_eq!(
            style!(Blue + italic+b),
            Style {
                color: Some(Blue),
                italic: true,
                bold: true,
                ..Default::default()
            }
        );

        assert_eq!(
            style!(+u+d+bold),
            Style {
                underline: true,
                dim: true,
                bold: true,
                ..Default::default()
            }
        );
    }
}
