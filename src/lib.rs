#[cfg(test)]
mod tests;

use Color::*;

macro_rules! color_enum {
    ( $( $number: literal $kind: ident ),* $(,)? ) => {
        /// Color for `Style`
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Color {
            $( $kind, )*
        }

        impl Color {
            /// Get the ANSI parameter for the `Color`
            fn param(&self) -> usize {
                match self {
                    $( $kind => $number , )*
                }
            }
        }
    };
}

color_enum!(
    30 Black,
    31 Red,
    32 Green,
    33 Yellow,
    34 Blue,
    35 Magenta,
    36 Cyan,
    37 White,
);

/// Stylize text with ANSI codes
///
/// Create with `Style::new()`
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Style {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    color: Option<Color>,
}

macro_rules! style_method {
    ( $short: ident $long: ident ) => {
        /// Add a style
        pub fn $long(&mut self) -> &mut Self {
            self.$long = true;
            self
        }
        /// Add a style
        pub fn $short(&mut self) -> &mut Self {
            self.$long = true;
            self
        }
    };
}

impl Style {
    /// Create an empty `Style` struct
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a color
    ///
    /// Overrides any previous color
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    style_method!(b bold);
    style_method!(d dim);
    style_method!(i italic);
    style_method!(u underline);

    /// Format text with `Style`
    ///
    /// Returns text as `String` if no styles or color is given
    pub fn format(&self, text: &str) -> String {
        let mut params = Vec::new();

        if let Some(color) = self.color {
            params.push(color.param());
        }

        macro_rules! style_param {
            ( $( $number: literal $name: ident ),* $(,)? ) => {
                $(
                    if self.$name {
                        params.push($number);
                    }
                )*
            };
        }

        style_param!(
            1 bold,
            2 dim,
            3 italic,
            4 underline,
        );

        if params.is_empty() {
            text.into()
        } else {
            format!(
                "\x1b[{}m{text}\x1b[0m",
                params
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(";"),
            )
        }
    }
}

/// Apply a `Style` to text
pub fn stylize(text: &str, style: &Style) -> String {
    style.format(text)
}

/// Apply any color and style to text
///
/// Similar to the `stylize` function
#[macro_export]
macro_rules! stylize {
    // No style
    ( $text: expr $(,)? ) => {{
        $text
    }};

    // Only color
    ( $text: expr, $color: ident ) => {{
        $crate::Style::new()
            .color($crate::Color::$color)
            .format($text)
    }};


    // Only styles
    ( $text: expr, - $( $style: ident )+ ) => {{
        $crate::Style::new()
            $(
                .$style()
            )*
            .format($text)
    }};

    // Style and color
    ( $text: expr, $color: ident $( $style: ident )* ) => {{
        $crate::Style::new()
            .color($crate::Color::$color)
            $(
                .$style()
            )*
            .format($text)
    }};
}
