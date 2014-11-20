extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag, TextAlignment};

fn main() {
    Console::init_root(80, 50, "Displaying text", false);
    RootConsole.print_ex(1, 1, BackgroundFlag::None, TextAlignment::Left,
                         "Text aligned to left.");
    RootConsole.print_ex(78, 1, BackgroundFlag::None, TextAlignment::Right,
                         "Text aligned to right.");
    RootConsole.print_ex(40, 15, BackgroundFlag::None, TextAlignment::Center,
                         "And this bit of text is centered.");
    RootConsole.print_ex(40, 19, BackgroundFlag::None, TextAlignment::Center,
                         "Press any key to quit.");
    Console::flush();
    Console::wait_for_keypress(true);
}
