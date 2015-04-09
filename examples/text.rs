extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag, TextAlignment};

fn main() {
    let mut root = RootConsole::initializer().size(80, 50).title("Displaying text").init();

    root.print_ex(1, 1, BackgroundFlag::None, TextAlignment::Left,
                  "Text aligned to left.");
    root.print_ex(78, 1, BackgroundFlag::None, TextAlignment::Right,
                  "Text aligned to right.");
    root.print_ex(40, 15, BackgroundFlag::None, TextAlignment::Center,
                  "And this bit of text is centered.");
    root.print_ex(40, 19, BackgroundFlag::None, TextAlignment::Center,
                  "Press any key to quit.");
    root.flush();
    root.wait_for_keypress(true);
}
