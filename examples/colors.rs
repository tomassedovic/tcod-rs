extern crate tcod;

use tcod::{Console, Color, BackgroundFlag};
use tcod::colors;


fn main() {
    let mut con = Console::init_root(80, 50, "Using colours with libtcod", false);
    con.set_default_background(colors::darkest_green);
    con.set_default_foreground(colors::lighter_azure);

    con.clear();
    // Uses the default foreground and background:
    con.put_char(40, 25, '@', BackgroundFlag::Set);
    // Custom foreground and background:
    con.put_char_ex(42, 25, '!', Color{r: 240, g: 13, b: 20}, Color{r: 0, g: 0, b: 0});

    Console::flush();

    // Press any key to exit:
    Console::wait_for_keypress(true);
}
