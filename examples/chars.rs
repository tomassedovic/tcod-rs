extern crate tcod;

use tcod::{Console, RootConsole};
use tcod::chars;

fn main() {
    Console::init_root(80, 50, "Example of libtcod's special chars", false);
    RootConsole.clear();

    // The top half of the box
    RootConsole.set_char(40, 25, chars::HLINE);
    RootConsole.set_char(41, 25, chars::NE);
    RootConsole.set_char(41, 26, chars::VLINE);
    RootConsole.set_char(41, 27, chars::SE);

    // Draw the heart:
    RootConsole.set_char(40, 26, chars::HEART);

    // The bottom half of the box
    RootConsole.set_char(40, 27, chars::HLINE);
    RootConsole.set_char(39, 27, chars::SW);
    RootConsole.set_char(39, 26, chars::VLINE);
    RootConsole.set_char(39, 25, chars::NW);

    Console::flush();
    Console::wait_for_keypress(true);
}

