use super::*;

#[test]
fn builder_works() {
    assert!(matches!(Style::new(), Style { .. }));
    assert!(matches!(Style::new().color(Red), Style { .. }));

    assert_eq!(Style::new().format("hello"), "hello");

    let style = Style::new().color(Red);
    assert_eq!(style.format("hello"), "\x1b[31mhello\x1b[0m");
    assert_eq!(
        style,
        Style {
            color: Some(Red),
            ..Default::default()
        }
    );

    let style = Style::new().italic();
    assert_eq!(style.format("hello"), "\x1b[3mhello\x1b[0m");
    assert_eq!(
        style,
        Style {
            italic: true,
            ..Default::default()
        }
    );

    let style = Style::new().color(Red).color(Blue).italic();
    assert_eq!(style.format("hello"), "\x1b[34;3mhello\x1b[0m");
    assert_eq!(
        style,
        Style {
            color: Some(Blue),
            italic: true,
            ..Default::default()
        }
    );

    let style = Style::new().bold().italic();
    assert_eq!(style.format("hello"), "\x1b[1;3mhello\x1b[0m");
    assert_eq!(
        style,
        Style {
            bold: true,
            italic: true,
            ..Default::default()
        }
    );

    let style = Style::new().bold().italic().color(Green);
    assert_eq!(style.format("hello"), "\x1b[32;1;3mhello\x1b[0m");
    assert_eq!(
        style,
        Style {
            bold: true,
            italic: true,
            color: Some(Green),
            ..Default::default()
        }
    );
}
