# Stilo

A small library for stylizing terminal text with ANSI color codes.

# Usage

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
