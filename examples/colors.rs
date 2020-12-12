extern crate tcod;

use tcod::colors;
use tcod::error::Result;
use tcod::{BackgroundFlag, Color, Console, RootConsole};

fn main() -> Result<()> {
    let mut con = RootConsole::initializer()
        .size(80, 50)
        .title("Using colours with libtcod")
        .init()?;
    con.set_default_background(Color::DARKEST_GREEN);
    con.set_default_foreground(Color::LIGHTER_AZURE);

    con.clear();
    // Uses the default foreground and background:
    con.put_char(40, 25, '@', BackgroundFlag::Set);
    // Custom foreground and background:
    con.put_char_ex(
        42,
        25,
        '!',
        Color {
            r: 240,
            g: 13,
            b: 20,
        },
        Color { r: 0, g: 0, b: 0 },
    );

    let black = Color::new(0, 0, 0);
    let red = Color::new(200, 0, 0);

    con.put_char_ex(30, 30, '.', black, red);
    con.put_char_ex(32, 30, '.', black, red * Color::new(200, 0, 0));
    con.put_char_ex(34, 30, '.', black, red * 0.3);
    con.put_char_ex(36, 30, '.', black, red + Color::new(25, 60, 0));
    con.put_char_ex(36, 30, '.', black, red - Color::new(25, 60, 0));
    con.put_char_ex(38, 30, '.', black, colors::lerp(red, black, 0.8));

    let (h, s, v) = red.hsv();
    println!("Red colour's hue: {}, saturation: {}, value: {}", h, s, v);

    con.flush()?;

    // Press any key to exit:
    con.wait_for_keypress(true);

    Ok(())
}
