extern crate tcod;

use tcod::{Console, RootConsole};
use tcod::chars;

fn main() {
    let mut root = RootConsole::initializer()
        .size(80, 50)
        .title("Example of libtcod's special chars")
        .init();

    root.clear();

    // The top half of the box
    root.set_char(40, 25, chars::HLINE);
    root.set_char(41, 25, chars::NE);
    root.set_char(41, 26, chars::VLINE);
    root.set_char(41, 27, chars::SE);

    // Draw the heart:
    root.set_char(40, 26, chars::HEART);

    // The bottom half of the box
    root.set_char(40, 27, chars::HLINE);
    root.set_char(39, 27, chars::SW);
    root.set_char(39, 26, chars::VLINE);
    root.set_char(39, 25, chars::NW);

    root.flush();
    root.wait_for_keypress(true);
}

