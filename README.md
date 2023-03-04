# Stilo

A small library for stylizing terminal text with ANSI color codes, with ergonomic macros.

# Usage

Add `stilo = "0.2.0"` to your `Cargo.toml` dependencies

## stylize!

Creates a `Style` struct and formats text.

[Docs](https://docs.rs/stilo/latest/stilo/macro.stylize.html)

```rust
use stilo::stylize;

// Red
println!("{}", stylize!("Hello": Red));

// Red, italic, and bold
println!("{}", stylize!("Hello": Red italic bold));

// Default color, italic and bold
println!("{}", stylize!("Hello": - i b));

// Format string
let world = "World!";
println!("{}", stylize!("Hello {}": Green i b, world));
```

## stylize_many! and println!\_styles!

Stylize many strings individually, and concatenate.

- `stylize_many!`: Returns a `String` of formatted text
- `println_styles!`: Prints a `String` of formatted text to stdout, with newline.
- - Same as `println!("{}", stylize_many!( ... ))`

[Docs](https://docs.rs/stilo/latest/stilo/macro.stylize_many.html)

```rust
use stilo::stylize_many;
let world = "World!";

// `println!("{}", stylize_many!` would also work in this example
println_styles!(
    // Red
    "Hello\n": Red;
    // Red, italic, and bold
    "Hello\n": Red italic bold;
    // Default color, italic and bold
    "Hello\n": - i b;
    // Format string
    "Hello {}": Green i b, world;
);
```

## style!

Creates a `Style` struct, without formatting text.

[Docs](https://docs.rs/stilo/latest/stilo/macro.style.html)

```rust
// Red
let style = style!(Red);
println!("{}", style.format("Hello"));

// Red, italic, and bold
let style = style!(Red italic bold);
println!("{}", style.format("Hello"));

// Default color, italic and bold
let style = style!(- i b);
println!("{}", style.format("Hello"));
```
