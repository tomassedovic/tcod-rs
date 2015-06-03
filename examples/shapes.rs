extern crate tcod;

use tcod::{Console, RootConsole, Color, BackgroundFlag};

fn main() {
    let mut root = RootConsole::initializer()
        .size(80, 50)
        .title("Example of libtcod's special chars")
        .init();

    root.clear();

    root.set_default_background(Color{r: 255, g: 0, b: 255});

    root.rect(5, 5, 10, 5, false, BackgroundFlag::Set);

    root.horizontal_line(30, 2, 15, BackgroundFlag::Default);
    root.vertical_line(28, 4, 9, BackgroundFlag::Default);
    root.horizontal_line(30, 14, 15, BackgroundFlag::Set);
    root.vertical_line(46, 4, 9, BackgroundFlag::Set);

    root.print_frame(15, 25, 35, 10, false, BackgroundFlag::Set, Some("Hello World!"));

    root.flush();
    root.wait_for_keypress(true);
}
