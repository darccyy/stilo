# Stilo

A small library for stylizing terminal text with ANSI color codes.

# Usage

Add `stilo = "0.1.0"` to your `Cargo.toml` dependencies

```rs
use stilo::stylize;

fn main() {
    // Red
    println!("{}", stylize!("Hello", Red));
    
    // Red, italic, and bold
    println!("{}", stylize!("Hello", Red italic bold));
    
    // Default color, italic and bold
    println!("{}", stylize!("Hello", - i b));
}
```
