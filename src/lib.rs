#[cfg(test)]
mod tests;

mod wrappers;
mod style;
mod stylize;
mod stylize_many;

use Color::*;

/// Private macro for creating `Color` enum, and adding code parameters
macro_rules! color_enum {
    ( $( $number: literal $kind: ident ),* $(,)? ) => {
        /// Color for `Style`
        /// 
        /// Import all colors with `use Color::*`
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Color {
            $( $kind, )*
        }

        impl Color {
            /// Get the ANSI parameter for the `Color`
            fn param(self) -> usize {
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
/// Color uses `Color` enum
///
/// Decorations include `bold`, `dim`, `italic`, and `underline`
///
/// Create with `Style::new()`
/// 
/// # Examples
/// 
/// ```
/// # use stilo::{Style, Color::*};
/// let style = Style::new().color(Red).italic();
/// 
/// println!("{}", style.format("Hello!"));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Style {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    color: Option<Color>,
}

/// Private macro for implementing decoration methods to `Style` struct
macro_rules! decor_method {
    ( $short: ident $long: ident ) => {
        /// Add a decoration
        pub fn $long(mut self) -> Self {
            self.$long = true;
            self
        }
        /// Add a decoration
        pub fn $short(mut self) -> Self {
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
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    decor_method!(b bold);
    decor_method!(d dim);
    decor_method!(i italic);
    decor_method!(u underline);

    /// Format text with `Style`
    ///
    /// Returns text as `String` if no color or decorations are given
    pub fn format(self, text: &str) -> String {
        let mut params = Vec::new();

        if let Some(color) = self.color {
            params.push(color.param());
        }

        /// Private macro for adding decoration codes to params
        macro_rules! decor_param {
            ( $( $code: literal $name: ident ),* $(,)? ) => {
                $(
                    if self.$name {
                        params.push($code);
                    }
                )*
            };
        }

        decor_param!(
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
/// 
/// Alternate syntax for `style.format(...)`
pub fn style_format(text: &str, style: Style) -> String {
    style.format(text)
}
