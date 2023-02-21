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
fn macro_works() {
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
