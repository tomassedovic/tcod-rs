extern crate tcod;

use tcod::{Console, Color, background_flag};
use tcod::colors;


fn main() {
    let mut con = Console::init_root(80, 50, "Using colours with libtcod", false);
    con.set_default_background(colors::darkest_green);
    con.set_default_foreground(colors::lighter_azure);

    con.clear();
    // Uses the default foreground and background:
    con.put_char(40, 25, '@', background_flag::Set);
    // Custom foreground and background:
    con.put_char_ex(42, 25, '!', Color::new(240, 13, 20), Color::new(0, 0, 0));

    Console::flush();

    // Press any key to exit:
    Console::wait_for_keypress(true);
}
