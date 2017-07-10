extern crate tcod;

use tcod::{Console, RootConsole};

const HEART_CHAR: char = 1 as char;
const DIAMOND_CHAR: char = 2 as char;
const SPADE_CHAR: char = 3 as char;
const CLUB_CHAR: char = 4 as char;

fn main() {
    let mut root = RootConsole::initializer()
        .size(80, 50)
        .title("Using custom character mapping with libtcod")
        .init();


    // Using the default 'terminal.png' font, we are going to map our custom
    // ASCII codes to the font characater in column 0, rows 3, 4, 5 and 6
    root.map_ascii_code_to_font(HEART_CHAR as i32, 0, 3);
    root.map_ascii_code_to_font(DIAMOND_CHAR as i32, 0, 4);
    root.map_ascii_code_to_font(SPADE_CHAR as i32, 0, 5);
    root.map_ascii_code_to_font(CLUB_CHAR as i32, 0, 6);

    root.clear();

    root.set_char(40, 25, HEART_CHAR);
    root.set_char(41, 25, DIAMOND_CHAR);
    root.set_char(42, 25, SPADE_CHAR);
    root.set_char(43, 25, CLUB_CHAR);

    root.flush();
    root.wait_for_keypress(true);
}
