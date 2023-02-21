use super::*;

#[test]
fn builder_works() {
    assert_eq!(Style::new().format("hello"), "hello");
    assert_eq!(
        Style::new().color(Red).format("hello"),
        "\x1b[31mhello\x1b[0m"
    );
    assert_eq!(Style::new().italic().format("hello"), "\x1b[3mhello\x1b[0m");
    assert_eq!(
        Style::new().color(Blue).italic().format("hello"),
        "\x1b[34;3mhello\x1b[0m"
    );
    assert_eq!(
        Style::new().color(Green).bold().italic().format("hello"),
        "\x1b[32;1;3mhello\x1b[0m"
    );
    assert_eq!(
        stylize("hello", Style::new().color(Green).bold().italic()),
        "\x1b[32;1;3mhello\x1b[0m"
    );
}

#[test]
fn style_macro_works() {
    assert_eq!(style!(Red), Style::new().color(Red));
    assert_eq!(style!(Blue),  Style::new().color(Blue));
    assert_eq!(style!(Red bold), Style::new().color(Red).bold());
    assert_eq!(style!(Red bold italic),Style::new().color(Red).b().i());
    assert_eq!(style!(-bold), Style::new().b());
    assert_eq!(style!(- bold italic), Style::new().bold().italic());
}

#[test]
fn stylize_macro_works() {
    assert_eq!(stylize!("hello"), "hello");
    assert_eq!(stylize!("hello", Red), "\x1b[31mhello\x1b[0m");
    assert_eq!(stylize!("hello", Blue), "\x1b[34mhello\x1b[0m");
    assert_eq!(stylize!("hello", Red bold), "\x1b[31;1mhello\x1b[0m");
    assert_eq!(
        stylize!("hello", Red bold italic),
        "\x1b[31;1;3mhello\x1b[0m"
    );
    assert_eq!(stylize!("hello", -bold), "\x1b[1mhello\x1b[0m");
    assert_eq!(stylize!("hello", - bold italic), "\x1b[1;3mhello\x1b[0m");
}
